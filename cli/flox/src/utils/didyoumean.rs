use std::fmt::Display;
use std::num::NonZeroU8;

use anyhow::Result;
use flox_rust_sdk::flox::Flox;
use flox_rust_sdk::models::search::SearchResults;
use flox_rust_sdk::providers::catalog::{Client, ClientTrait};
use pollster::FutureExt;
use tracing::{debug, instrument};

use super::search::{DisplayItems, DisplaySearchResults};

pub const SUGGESTION_SEARCH_LIMIT: u8 = 3;

/// Dynamically generate a "did you mean" message for a given search term.
/// Will look up curated suggested search terms and query related search results.
///
/// [DidYouMean] is parameterized by a type `S`,
/// which is used to distinguish input types for the suggestion
/// and specific suggestion output.
#[derive(Debug)]
pub struct DidYouMean<'a, S> {
    searched_term: &'a str,
    curated: Option<&'static str>,
    search_results: SearchResults,
    _suggestion: S,
}

#[derive(Debug)]
pub struct InstallSuggestion;

impl<S> DidYouMean<'_, S> {
    pub fn has_suggestions(&self) -> bool {
        self.curated.is_some() || !self.search_results.results.is_empty()
    }
}

/// Suggestions for `install` subcommand
impl<'a> DidYouMean<'a, InstallSuggestion> {
    /// `install` specific curated terms
    fn suggest_curated_package(input: &str) -> Option<&'static str> {
        let suggestion = match input {
            "java" => "jdk",
            "node" => "nodejs",
            "npm" => "nodejs",
            "rust" => "cargo",
            "sed" => "gnused",
            "make" => "gnumake",
            "awk" => "gawk",
            "diff" => "diffutils",
            "grep" => "gnugrep",
            _ => return None,
        };
        Some(suggestion)
    }

    fn suggest_searched_packages(flox: &Flox, term: &str) -> Result<SearchResults> {
        tracing::debug!("using client for install suggestions");
        Self::suggest_searched_packages_catalog(&flox.catalog_client, term, flox.system.clone())
    }

    /// Collects installation suggestions for a given query using the catalog
    #[instrument(skip(client), fields(progress = "Looking for alternative suggestions"))]
    fn suggest_searched_packages_catalog(
        client: &Client,
        term: &str,
        system: String,
    ) -> Result<SearchResults> {
        let results = client
            .search_with_spinner(
                term,
                system.to_string(),
                NonZeroU8::new(SUGGESTION_SEARCH_LIMIT),
            )
            .block_on()?;
        Ok(results)
    }

    /// Create a new [DidYouMean] instance for the given search term
    /// in the context of an [Environment].
    ///
    /// This will attempt to find curated suggestions for the given term,
    /// based on the lockfile of the given environment.
    pub fn new(flox: &Flox, term: &'a str) -> Self {
        let curated = Self::suggest_curated_package(term);
        let searched_term = curated.unwrap_or(term);
        let search_results = match Self::suggest_searched_packages(flox, searched_term) {
            Ok(results) => results,
            Err(err) => {
                debug!("failed to search for suggestions: {}", err);
                SearchResults {
                    results: Default::default(),
                    count: Some(0),
                }
            },
        };
        Self {
            searched_term,
            curated,
            search_results,
            _suggestion: InstallSuggestion,
        }
    }
}

impl Display for DidYouMean<'_, InstallSuggestion> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(curated) = self.curated {
            writeln!(f, "Try 'flox install {curated}' instead.")?;
        }

        if self.search_results.results.is_empty() {
            return Ok(());
        }

        writeln!(f)?;
        writeln!(f, "Here are a few other similar options:")?;

        // apparently its permitted that the count is absent,
        // if the catalog service does not send it
        // see: `providers::catalog::collect_search_results`.
        let count_message = match self.search_results.count {
            Some(n) => format!("up to {n}"),
            None => "more".to_string(),
        };

        let display_items: DisplayItems = self.search_results.results.clone().into();
        for result in display_items.iter() {
            writeln!(f, "  $ flox install {result}",)?;
        }

        write!(
            f,
            "...or see {count_message} options with 'flox search {term}'",
            term = self.searched_term
        )?;

        Ok(())
    }
}

pub struct SearchSuggestion;

/// Suggestions for `search` subcommand
impl<'a> DidYouMean<'a, SearchSuggestion> {
    /// `search` specific curated terms
    fn suggest_curated_package(input: &str) -> Option<&'static str> {
        let suggestion = match input {
            "node" => "nodejs",
            "java" => "jdk",
            "npm" => "nodejs",
            "rust" => "cargo",
            "diff" => "diffutils",
            "make" => "gnumake",
            "k8s" => "kubernetes",
            _ => return None,
        };
        Some(suggestion)
    }

    #[instrument(skip(client), fields(progress = "Looking for alternative suggestions"))]
    fn suggest_searched_packages_catalog(
        client: &Client,
        term: &str,
        system: String,
    ) -> Result<SearchResults> {
        let results = client
            .search_with_spinner(
                term,
                system.to_string(),
                NonZeroU8::new(SUGGESTION_SEARCH_LIMIT),
            )
            .block_on()?;
        Ok(results)
    }

    /// Create a new [DidYouMean] instance for the given search term.
    ///
    /// This will attempt to find curated suggestions for the given term,
    /// and then query for related search results.
    /// Either of these may fail, in which case we will return with empty [SearchResults]
    /// and log the error.
    pub fn new(term: &'a str, catalog_client: &Client, system: String) -> Self {
        let curated = Self::suggest_curated_package(term);

        let default_results = SearchResults {
            results: Default::default(),
            count: Some(0),
        };

        let search_results = if let Some(curated) = curated {
            let res = Self::suggest_searched_packages_catalog(catalog_client, curated, system);

            match res {
                Ok(results) => results,
                Err(err) => {
                    debug!("failed to search for suggestions: {}", err);
                    default_results
                },
            }
        } else {
            default_results
        };

        Self {
            searched_term: term,
            curated,
            search_results,
            _suggestion: SearchSuggestion,
        }
    }
}

impl Display for DidYouMean<'_, SearchSuggestion> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(curated) = self.curated else {
            debug!("no curated suggestions");
            return Ok(());
        };

        let search_results_rendered =
            match DisplaySearchResults::from_search_results(curated, self.search_results.clone()) {
                Ok(rendered) => rendered,
                Err(err) => {
                    debug!("failed to render search results: {}", err);
                    return Ok(());
                },
            };

        writeln!(f, "Related search results for '{curated}':")?;
        write!(f, "{search_results_rendered}")?;

        Ok(())
    }
}
