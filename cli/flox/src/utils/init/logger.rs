use std::sync::OnceLock;

use tracing::{debug, error};
use tracing_indicatif::util::FilteredFormatFields;
use tracing_subscriber::prelude::*;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry, filter};

use crate::commands::Verbosity;
use crate::utils::init::logger::indicatif::PROGRESS_TAG;
use crate::utils::metrics::MetricsLayer;

static LOGGER_HANDLE: OnceLock<Handle<EnvFilter, Registry>> = OnceLock::new();

pub(crate) fn init_logger(verbosity: Option<Verbosity>) {
    let verbosity = verbosity.unwrap_or_default();

    let log_filter = match verbosity {
        // Show only errors
        Verbosity::Quiet => "off,flox=error",
        // Only show warnings, and user facing messages
        Verbosity::Verbose(0) => "warn,flox::utils::message=info",
        // Show internal info logs
        Verbosity::Verbose(1) => "warn,flox=info,flox-rust-sdk=info,flox-core=info",
        // Show debug logs from our libraries
        Verbosity::Verbose(2) => "warn,flox=debug,flox-rust-sdk=debug,flox-core=debug",
        // Show trace logs from our libraries
        Verbosity::Verbose(3) => "warn,flox=trace,flox-rust-sdk=trace,flox-core=trace",
        // Show trace for all libraries
        Verbosity::Verbose(_) => "trace",
    };

    let filter_handle = LOGGER_HANDLE.get_or_init(|| {
        let (subscriber, reload_handle) = create_registry_and_filter_reload_handle();
        subscriber.init();
        reload_handle
    });

    update_filters(filter_handle, log_filter);
}

pub fn update_filters(filter_handle: &Handle<EnvFilter, Registry>, log_filter: &str) {
    let result = filter_handle.modify(|layer| {
        match EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(log_filter)) {
            Ok(new_filter) => *layer = new_filter,
            Err(err) => {
                error!("Updating logger filter failed: {}", err);
            },
        };
    });
    if let Err(err) = result {
        error!("Updating logger filter failed: {}", err);
    }
}

pub fn create_registry_and_filter_reload_handle() -> (
    impl tracing_subscriber::layer::SubscriberExt,
    Handle<EnvFilter, Registry>,
) {
    debug!("Initializing logger (how are you seeing this?)");

    let (progress_layer, writer) = indicatif::progress_layer();
    // The first time this layer is set it establishes an upper boundary for `log` verbosity.
    // If you try to `modify` this layer later, `log` will not accept any higher verbosity events.
    //
    // Before we used to replace both the fmt layer _and_ this layer.
    // That purged enough internal state to reset the `log` verbosity filter.
    // For simplicity, we'll now just set the filter to `trace`,
    // and then modify it later to the actual level below.
    // Logs are being passed through by the `log` crate and correctly filtered by `tracing`.
    let filter = tracing_subscriber::filter::EnvFilter::try_new("trace").unwrap();

    let (filter, filter_reload_handle) = tracing_subscriber::reload::Layer::new(filter);
    let use_colors = supports_color::on(supports_color::Stream::Stderr).is_some();

    // Tracing layer that handles user facing messages.
    // That is messages that are produced by the `crate::utils::message` module,
    // and target the flox _user_, rather than revealing internals.
    let message_fmt = tracing_subscriber::fmt::format()
        .compact()
        .without_time()
        .with_level(false)
        .with_target(false);
    let message_layer = tracing_subscriber::fmt::layer()
        .with_writer(writer.clone())
        .with_ansi(use_colors)
        .event_format(message_fmt)
        .with_filter(filter::filter_fn(|meta| {
            meta.target().starts_with("flox::utils::message")
        }));

    // Tracing layer that handles all other logs.
    //
    // Span data is added to _all_ events within them, and "stack" if multiple spans are active.
    // While the JSON formatter seems to support to suppress this span information,
    // the same is not possible with either of the other builtin formatters.
    //
    // An existing issue on that upstream appears not to have active development:
    // <https://github.com/tokio-rs/tracing/issues/3254>
    //
    // What is possible however is to filter out the _"progress" fields_,
    // so that spans are still printed but we don't repeat the messages.
    // That is using the `FilteredFormatFields` utility from `tracing_indicative`,
    // which is a visitor implementation that just dropts fields based on a filter function,
    // here: a test for the field name "progress".
    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(writer.clone())
        .with_ansi(use_colors)
        .map_fmt_fields(|format| {
            FilteredFormatFields::new(format, |field| field.name() != PROGRESS_TAG)
        })
        .with_filter(filter::filter_fn(|meta| {
            !meta.target().starts_with("flox::utils::message")
        }));

    // The combined layer that handles tracing events and formats them,
    // either for user facing messages or for internal logs.
    // The verbosity of these logs is controlled by the `filter` env filter.
    let combined_log_layer = log_layer.and_then(message_layer).with_filter(filter);

    let metrics_layer = MetricsLayer::new();
    let sentry_layer = sentry::integrations::tracing::layer().enable_span_attributes();
    // Filtered layer must come first.
    // This appears to be the only way to avoid logs of the `flox_command` trace
    // which is processed by the `log_layer` irrepective of the filter applied to it.
    // My current understanding is, that it because the `metrics_layer` (at least) is
    // registering `Interest` for the event and that somehow bypasses the filter?!
    let registry = tracing_subscriber::registry()
        .with(combined_log_layer)
        .with(progress_layer)
        .with(metrics_layer)
        .with(sentry_layer);

    (registry, filter_reload_handle)
}

// region: indicatif
mod indicatif {
    use std::fmt::{self, Display, Write};

    use indicatif::{ProgressState, ProgressStyle};
    use tracing::Subscriber;
    use tracing::field::{Field, Visit};
    use tracing_indicatif::IndicatifWriter;
    use tracing_subscriber::field::RecordFields;
    use tracing_subscriber::fmt::FormatFields;
    use tracing_subscriber::fmt::format::Writer;
    use tracing_subscriber::layer::Layer;
    use tracing_subscriber::registry;

    pub(super) const PROGRESS_TAG: &str = "progress";

    pub fn progress_layer<S>() -> (impl tracing_subscriber::Layer<S>, IndicatifWriter)
    where
        S: Subscriber + for<'span> registry::LookupSpan<'span> + 'static,
    {
        #[derive(Debug, Default)]
        struct Visitor {
            message: Option<String>,
        }
        impl Display for Visitor {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(message) = &self.message {
                    write!(f, "{message}")
                } else {
                    write!(f, "👻 How can you see me?")
                }
            }
        }
        impl Visit for Visitor {
            fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
                self.record_str(field, &format!("{:?}", value));
            }

            fn record_str(&mut self, field: &Field, value: &str) {
                if field.name() == PROGRESS_TAG {
                    self.message = Some(value.to_string());
                }
            }
        }

        struct Formatter;
        impl<'writer> FormatFields<'writer> for Formatter {
            /// Format the provided `fields` to the provided [`Writer`], returning a result.
            fn format_fields<R: RecordFields>(
                &self,
                mut writer: Writer<'writer>,
                fields: R,
            ) -> fmt::Result {
                let mut visitor = Visitor::default();
                fields.record(&mut visitor);

                write!(&mut writer, "{visitor}")?;

                Ok(())
            }
        }

        // The progress bar style, a spinner the progress message
        // and the elapsed time if it's running longer than 1 second.
        let style =
            ProgressStyle::with_template("{span_child_prefix}{spinner} {span_fields} {wide_msg}")
                .unwrap()
                .with_key(
                    "elapsed",
                    |state: &ProgressState, writer: &mut dyn Write| {
                        if state.elapsed() > std::time::Duration::from_secs(1) {
                            let seconds = state.elapsed().as_secs();
                            let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
                            let _ = writer.write_str(&format!("{}.{}s", seconds, sub_seconds));
                        }
                    },
                );

        let layer = tracing_indicatif::IndicatifLayer::new()
            .with_progress_style(style)
            .with_span_field_formatter(Formatter);

        let writer = layer.get_stderr_writer();

        let filtered = layer.with_filter(tracing_subscriber::filter::FilterFn::new(|meta| {
            meta.fields()
                .iter()
                .any(|field| field.name() == PROGRESS_TAG)
        }));

        (filtered, writer)
    }
}
// endregion: indicatif
