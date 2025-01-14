use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

pub type SearchLimit = Option<NonZeroU8>;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SearchStrategy {
    Match,
    MatchName,
    #[default]
    MatchNameOrRelPath,
}

/// Representation of search results.
/// Created via [crate::providers::catalog::ClientTrait::search],
/// which translates raw api responses to this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
    pub count: ResultCount,
}
pub type ResultCount = Option<u64>;

/// A package search result
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Which input the package came from
    ///
    /// This is also abused to be the name of the catalog.
    /// At some point we should rename this to catalog.
    pub input: String,
    /// The system that the package can be built for
    pub system: String,
    /// The part of the attribute path after `<subtree>.<system>`.
    ///
    /// For an arbitrary flake this will simply be the name of the package, but
    /// for nixpkgs this can be something like `python310Packages.flask`
    pub attr_path: Vec<String>,
    /// The package path including catalog name
    pub pkg_path: String,
    /// The package name
    pub pname: Option<String>,
    /// The package version
    pub version: Option<String>,
    /// The package description
    pub description: Option<String>,
    /// Which license the package is licensed under
    pub license: Option<String>,
}
