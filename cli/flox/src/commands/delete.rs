use anyhow::{Result, bail};
use bpaf::Bpaf;
use flox_rust_sdk::flox::Flox;
use flox_rust_sdk::models::environment::{ConcreteEnvironment, Environment};
use indoc::formatdoc;
use tracing::instrument;

use super::{EnvironmentSelect, environment_select};
use crate::commands::environment_description;
use crate::environment_subcommand_metric;
use crate::utils::dialog::{Confirm, Dialog};
use crate::utils::message;

// Delete an environment
#[derive(Bpaf, Clone)]
pub struct Delete {
    /// Delete an environment without confirmation.
    #[bpaf(short, long)]
    force: bool,

    #[bpaf(external(environment_select), fallback(Default::default()))]
    environment: EnvironmentSelect,
}

impl Delete {
    #[instrument(name = "delete", skip_all)]
    pub async fn handle(self, flox: Flox) -> Result<()> {
        let environment = self
            .environment
            .detect_concrete_environment(&flox, "Delete")?;

        environment_subcommand_metric!("delete", environment);

        let description = environment_description(&environment)?;

        if matches!(environment, ConcreteEnvironment::Remote(_)) {
            let message = formatdoc! {"
                Environment {description} was not deleted.

                Remote environments on FloxHub can not yet be deleted.
            "};
            bail!("{message}")
        }

        let message = if let EnvironmentSelect::Unspecified = self.environment {
            format!("You are about to delete your environment {description}. Are you sure?")
        } else {
            "Are you sure?".to_string()
        };

        let confirm = Dialog {
            message: &message,
            help_message: Some("Use `-f` to force deletion"),
            typed: Confirm {
                default: Some(false),
            },
        };

        if !self.force && Dialog::can_prompt() && !confirm.prompt().await? {
            bail!("Environment deletion cancelled");
        }

        match environment {
            ConcreteEnvironment::Path(environment) => environment.delete(&flox),
            ConcreteEnvironment::Managed(environment) => environment.delete(&flox),
            ConcreteEnvironment::Remote(_) => unreachable!(),
        }?;

        message::deleted(format!("environment {description} deleted"));

        Ok(())
    }
}
