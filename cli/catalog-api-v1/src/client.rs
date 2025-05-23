#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///CatalogName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Catalog Name",
    ///  "type": "string",
    ///  "pattern": "[a-zA-Z0-9\\-_]{3,64}"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct CatalogName(String);
    impl std::ops::Deref for CatalogName {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<CatalogName> for String {
        fn from(value: CatalogName) -> Self {
            value.0
        }
    }
    impl From<&CatalogName> for CatalogName {
        fn from(value: &CatalogName) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for CatalogName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("[a-zA-Z0-9\\-_]{3,64}")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"[a-zA-Z0-9\\-_]{3,64}\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for CatalogName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for CatalogName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for CatalogName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for CatalogName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///CatalogPage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CatalogPage",
    ///  "examples": [
    ///    {
    ///      "attr_path": "foo.bar.curl",
    ///      "description": "A very nice Item",
    ///      "license": "foo",
    ///      "locked_url": "git:git?rev=xyz",
    ///      "name": "curl",
    ///      "outputs": "{}",
    ///      "outputs_to_install": "{}",
    ///      "pkg_path": "foo.bar.curl",
    ///      "pname": "curl",
    ///      "rev": "xyz",
    ///      "rev_count": 4,
    ///      "rev_date": 0,
    ///      "stabilities": [
    ///        "stable",
    ///        "unstable"
    ///      ],
    ///      "system": "x86_64-linux",
    ///      "version": "1.0"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "complete",
    ///    "messages",
    ///    "page",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "complete": {
    ///      "title": "Complete",
    ///      "type": "boolean"
    ///    },
    ///    "messages": {
    ///      "title": "Messages",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResolutionMessageGeneral"
    ///      }
    ///    },
    ///    "packages": {
    ///      "title": "Packages",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResolvedPackageDescriptor"
    ///      }
    ///    },
    ///    "page": {
    ///      "title": "Page",
    ///      "type": "integer"
    ///    },
    ///    "url": {
    ///      "title": "Url",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct CatalogPage {
        pub complete: bool,
        pub messages: Vec<ResolutionMessageGeneral>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub packages: Option<Vec<ResolvedPackageDescriptor>>,
        pub page: i64,
        pub url: String,
    }
    impl From<&CatalogPage> for CatalogPage {
        fn from(value: &CatalogPage) -> Self {
            value.clone()
        }
    }
    ///CatalogShareInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CatalogShareInfo",
    ///  "type": "object",
    ///  "properties": {
    ///    "allow_read_users": {
    ///      "title": "Allow Read Users",
    ///      "default": [],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct CatalogShareInfo {
        #[serde(default = "defaults::catalog_share_info_allow_read_users")]
        pub allow_read_users: Option<Vec<String>>,
    }
    impl From<&CatalogShareInfo> for CatalogShareInfo {
        fn from(value: &CatalogShareInfo) -> Self {
            value.clone()
        }
    }
    ///CatalogStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CatalogStatus",
    ///  "type": "object",
    ///  "required": [
    ///    "attribute_path_ct",
    ///    "catalogs",
    ///    "derivations_ct",
    ///    "latest_rev",
    ///    "pages_ct",
    ///    "schema_version",
    ///    "search_index_ct",
    ///    "systems",
    ///    "tags"
    ///  ],
    ///  "properties": {
    ///    "attribute_path_ct": {
    ///      "title": "Attribute Path Ct",
    ///      "type": "integer"
    ///    },
    ///    "catalogs": {
    ///      "title": "Catalogs",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "derivations_ct": {
    ///      "title": "Derivations Ct",
    ///      "type": "integer"
    ///    },
    ///    "latest_rev": {
    ///      "title": "Latest Rev",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "latest_scrape": {
    ///      "title": "Latest Scrape",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "narinfos_ct": {
    ///      "title": "Narinfos Ct",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "pages_ct": {
    ///      "title": "Pages Ct",
    ///      "type": "integer"
    ///    },
    ///    "schema_version": {
    ///      "title": "Schema Version",
    ///      "type": "string"
    ///    },
    ///    "search_index_ct": {
    ///      "title": "Search Index Ct",
    ///      "type": "integer"
    ///    },
    ///    "storepaths_ct": {
    ///      "title": "Storepaths Ct",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "systems": {
    ///      "title": "Systems",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "tags": {
    ///      "title": "Tags",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct CatalogStatus {
        pub attribute_path_ct: i64,
        pub catalogs: Vec<String>,
        pub derivations_ct: i64,
        pub latest_rev: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub latest_scrape: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub narinfos_ct: Option<i64>,
        pub pages_ct: i64,
        pub schema_version: String,
        pub search_index_ct: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub storepaths_ct: Option<i64>,
        pub systems: Vec<String>,
        pub tags: std::collections::HashMap<String, Vec<String>>,
    }
    impl From<&CatalogStatus> for CatalogStatus {
        fn from(value: &CatalogStatus) -> Self {
            value.clone()
        }
    }
    ///CatalogStoreConfigMetaOnly
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CatalogStoreConfigMetaOnly",
    ///  "type": "object",
    ///  "properties": {
    ///    "store_type": {
    ///      "title": "Store Type",
    ///      "default": "meta-only",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct CatalogStoreConfigMetaOnly {
        #[serde(default = "defaults::catalog_store_config_meta_only_store_type")]
        pub store_type: String,
    }
    impl From<&CatalogStoreConfigMetaOnly> for CatalogStoreConfigMetaOnly {
        fn from(value: &CatalogStoreConfigMetaOnly) -> Self {
            value.clone()
        }
    }
    ///CatalogStoreConfigNull
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CatalogStoreConfigNull",
    ///  "type": "object",
    ///  "properties": {
    ///    "store_type": {
    ///      "title": "Store Type",
    ///      "default": "null",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct CatalogStoreConfigNull {
        #[serde(default = "defaults::catalog_store_config_null_store_type")]
        pub store_type: String,
    }
    impl From<&CatalogStoreConfigNull> for CatalogStoreConfigNull {
        fn from(value: &CatalogStoreConfigNull) -> Self {
            value.clone()
        }
    }
    ///CatalogStoreConfigPublisher
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "CatalogStoreConfigPublisher",
    ///  "type": "object",
    ///  "required": [
    ///    "publisher_url"
    ///  ],
    ///  "properties": {
    ///    "publisher_url": {
    ///      "title": "Publisher Url",
    ///      "type": "string"
    ///    },
    ///    "store_type": {
    ///      "title": "Store Type",
    ///      "default": "publisher",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct CatalogStoreConfigPublisher {
        pub publisher_url: String,
        #[serde(default = "defaults::catalog_store_config_publisher_store_type")]
        pub store_type: String,
    }
    impl From<&CatalogStoreConfigPublisher> for CatalogStoreConfigPublisher {
        fn from(value: &CatalogStoreConfigPublisher) -> Self {
            value.clone()
        }
    }
    ///ErrorResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ErrorResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "detail"
    ///  ],
    ///  "properties": {
    ///    "detail": {
    ///      "title": "Detail",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ErrorResponse {
        pub detail: String,
    }
    impl From<&ErrorResponse> for ErrorResponse {
        fn from(value: &ErrorResponse) -> Self {
            value.clone()
        }
    }
    ///HealthCheck
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "HealthCheck",
    ///  "type": "object",
    ///  "required": [
    ///    "resolve_elapsed_ms",
    ///    "resolve_ok",
    ///    "search_elapsed_ms",
    ///    "search_ok",
    ///    "show_elapsed_ms",
    ///    "show_ok"
    ///  ],
    ///  "properties": {
    ///    "check_parameters": {
    ///      "$ref": "#/components/schemas/params"
    ///    },
    ///    "resolve_elapsed_ms": {
    ///      "title": "Resolve Elapsed Ms",
    ///      "type": "integer"
    ///    },
    ///    "resolve_ok": {
    ///      "title": "Resolve Ok",
    ///      "type": "boolean"
    ///    },
    ///    "search_elapsed_ms": {
    ///      "title": "Search Elapsed Ms",
    ///      "type": "integer"
    ///    },
    ///    "search_ok": {
    ///      "title": "Search Ok",
    ///      "type": "boolean"
    ///    },
    ///    "show_elapsed_ms": {
    ///      "title": "Show Elapsed Ms",
    ///      "type": "integer"
    ///    },
    ///    "show_ok": {
    ///      "title": "Show Ok",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct HealthCheck {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub check_parameters: Option<Params>,
        pub resolve_elapsed_ms: i64,
        pub resolve_ok: bool,
        pub search_elapsed_ms: i64,
        pub search_ok: bool,
        pub show_elapsed_ms: i64,
        pub show_ok: bool,
    }
    impl From<&HealthCheck> for HealthCheck {
        fn from(value: &HealthCheck) -> Self {
            value.clone()
        }
    }
    ///MessageLevel
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "MessageLevel",
    ///  "type": "string",
    ///  "enum": [
    ///    "trace",
    ///    "info",
    ///    "warning",
    ///    "error"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum MessageLevel {
        #[serde(rename = "trace")]
        Trace,
        #[serde(rename = "info")]
        Info,
        #[serde(rename = "warning")]
        Warning,
        #[serde(rename = "error")]
        Error,
    }
    impl From<&MessageLevel> for MessageLevel {
        fn from(value: &MessageLevel) -> Self {
            value.clone()
        }
    }
    impl ToString for MessageLevel {
        fn to_string(&self) -> String {
            match *self {
                Self::Trace => "trace".to_string(),
                Self::Info => "info".to_string(),
                Self::Warning => "warning".to_string(),
                Self::Error => "error".to_string(),
            }
        }
    }
    impl std::str::FromStr for MessageLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "trace" => Ok(Self::Trace),
                "info" => Ok(Self::Info),
                "warning" => Ok(Self::Warning),
                "error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for MessageLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for MessageLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for MessageLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Name
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Name",
    ///  "type": "string",
    ///  "pattern": "[a-zA-Z0-9\\-_]{3,64}"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct Name(String);
    impl std::ops::Deref for Name {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<Name> for String {
        fn from(value: Name) -> Self {
            value.0
        }
    }
    impl From<&Name> for Name {
        fn from(value: &Name) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for Name {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("[a-zA-Z0-9\\-_]{3,64}")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"[a-zA-Z0-9\\-_]{3,64}\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///NarInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "NarInfo",
    ///  "type": "object",
    ///  "properties": {
    ///    "ca": {
    ///      "title": "Ca",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "closureDownloadSize": {
    ///      "title": "Closuredownloadsize",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "closureSize": {
    ///      "title": "Closuresize",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "compression": {
    ///      "title": "Compression",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "compresssize": {
    ///      "title": "Compresssize",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "deriver": {
    ///      "title": "Deriver",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "downloadHash": {
    ///      "title": "Downloadhash",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "downloadSize": {
    ///      "title": "Downloadsize",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "narHash": {
    ///      "title": "Narhash",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "narSize": {
    ///      "title": "Narsize",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "references": {
    ///      "title": "References",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "registrationTime": {
    ///      "title": "Registrationtime",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "signatures": {
    ///      "title": "Signatures",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "ultimate": {
    ///      "title": "Ultimate",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "url": {
    ///      "title": "Url",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct NarInfo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ca: Option<String>,
        #[serde(
            rename = "closureDownloadSize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub closure_download_size: Option<i64>,
        #[serde(
            rename = "closureSize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub closure_size: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compression: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compresssize: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deriver: Option<String>,
        #[serde(
            rename = "downloadHash",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub download_hash: Option<String>,
        #[serde(
            rename = "downloadSize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub download_size: Option<i64>,
        #[serde(rename = "narHash", default, skip_serializing_if = "Option::is_none")]
        pub nar_hash: Option<String>,
        #[serde(rename = "narSize", default, skip_serializing_if = "Option::is_none")]
        pub nar_size: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub references: Option<Vec<String>>,
        #[serde(
            rename = "registrationTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_time: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signatures: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ultimate: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl From<&NarInfo> for NarInfo {
        fn from(value: &NarInfo) -> Self {
            value.clone()
        }
    }
    ///NarInfos
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "NarInfos",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "$ref": "#/components/schemas/NarInfo"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct NarInfos(pub std::collections::HashMap<String, NarInfo>);
    impl std::ops::Deref for NarInfos {
        type Target = std::collections::HashMap<String, NarInfo>;
        fn deref(&self) -> &std::collections::HashMap<String, NarInfo> {
            &self.0
        }
    }
    impl From<NarInfos> for std::collections::HashMap<String, NarInfo> {
        fn from(value: NarInfos) -> Self {
            value.0
        }
    }
    impl From<&NarInfos> for NarInfos {
        fn from(value: &NarInfos) -> Self {
            value.clone()
        }
    }
    impl From<std::collections::HashMap<String, NarInfo>> for NarInfos {
        fn from(value: std::collections::HashMap<String, NarInfo>) -> Self {
            Self(value)
        }
    }
    ///Output
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Output",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "store_path"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "store_path": {
    ///      "title": "Store Path",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Output {
        pub name: String,
        pub store_path: String,
    }
    impl From<&Output> for Output {
        fn from(value: &Output) -> Self {
            value.clone()
        }
    }
    ///Outputs
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Outputs",
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/Output"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Outputs(pub Vec<Output>);
    impl std::ops::Deref for Outputs {
        type Target = Vec<Output>;
        fn deref(&self) -> &Vec<Output> {
            &self.0
        }
    }
    impl From<Outputs> for Vec<Output> {
        fn from(value: Outputs) -> Self {
            value.0
        }
    }
    impl From<&Outputs> for Outputs {
        fn from(value: &Outputs) -> Self {
            value.clone()
        }
    }
    impl From<Vec<Output>> for Outputs {
        fn from(value: Vec<Output>) -> Self {
            Self(value)
        }
    }
    ///PackageDescriptor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackageDescriptor",
    ///  "examples": [
    ///    {
    ///      "attr_path": "curl",
    ///      "install_id": "curl",
    ///      "systems": [
    ///        "x86_64-linux"
    ///      ]
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "attr_path",
    ///    "install_id",
    ///    "systems"
    ///  ],
    ///  "properties": {
    ///    "allow_broken": {
    ///      "title": "Allow Broken",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "allow_insecure": {
    ///      "title": "Allow Insecure",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "allow_missing_builds": {
    ///      "title": "Allow Missing Builds",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "allow_pre_releases": {
    ///      "title": "Allow Pre Releases",
    ///      "default": false,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "allow_unfree": {
    ///      "title": "Allow Unfree",
    ///      "default": true,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "allowed_licenses": {
    ///      "title": "Allowed Licenses",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "attr_path": {
    ///      "title": "Attr Path",
    ///      "type": "string"
    ///    },
    ///    "derivation": {
    ///      "title": "Derivation",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "install_id": {
    ///      "title": "Install Id",
    ///      "type": "string"
    ///    },
    ///    "systems": {
    ///      "title": "Systems",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SystemEnum"
    ///      }
    ///    },
    ///    "version": {
    ///      "title": "Version",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackageDescriptor {
        #[serde(default = "defaults::package_descriptor_allow_broken")]
        pub allow_broken: Option<bool>,
        #[serde(default = "defaults::package_descriptor_allow_insecure")]
        pub allow_insecure: Option<bool>,
        #[serde(default = "defaults::package_descriptor_allow_missing_builds")]
        pub allow_missing_builds: Option<bool>,
        #[serde(default = "defaults::package_descriptor_allow_pre_releases")]
        pub allow_pre_releases: Option<bool>,
        #[serde(default = "defaults::package_descriptor_allow_unfree")]
        pub allow_unfree: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allowed_licenses: Option<Vec<String>>,
        pub attr_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub derivation: Option<String>,
        pub install_id: String,
        pub systems: Vec<SystemEnum>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    impl From<&PackageDescriptor> for PackageDescriptor {
        fn from(value: &PackageDescriptor) -> Self {
            value.clone()
        }
    }
    ///PackageGroup
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackageGroup",
    ///  "examples": [
    ///    {
    ///      "descriptors": [
    ///        {
    ///          "attr_path": "curl",
    ///          "install_id": "curl",
    ///          "systems": [
    ///            "x86_64-linux"
    ///          ]
    ///        },
    ///        {
    ///          "attr_path": "slack",
    ///          "install_id": "slack",
    ///          "systems": [
    ///            "x86_64-linux"
    ///          ]
    ///        },
    ///        {
    ///          "attr_path": "xorg.xeyes",
    ///          "install_id": "xeyes",
    ///          "systems": [
    ///            "x86_64-linux"
    ///          ]
    ///        }
    ///      ],
    ///      "name": "test"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "descriptors",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "descriptors": {
    ///      "title": "Descriptors",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PackageDescriptor"
    ///      }
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "stability": {
    ///      "title": "Stability",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackageGroup {
        pub descriptors: Vec<PackageDescriptor>,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stability: Option<String>,
    }
    impl From<&PackageGroup> for PackageGroup {
        fn from(value: &PackageGroup) -> Self {
            value.clone()
        }
    }
    ///PackageGroups
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackageGroups",
    ///  "examples": [
    ///    {
    ///      "items": [
    ///        {
    ///          "descriptors": [
    ///            {
    ///              "attr_path": "curl",
    ///              "install_id": "curl",
    ///              "systems": [
    ///                "x86_64-linux"
    ///              ]
    ///            },
    ///            {
    ///              "attr_path": "slack",
    ///              "install_id": "slack",
    ///              "systems": [
    ///                "x86_64-linux"
    ///              ]
    ///            },
    ///            {
    ///              "attr_path": "xorg.xeyes",
    ///              "install_id": "xeyes",
    ///              "systems": [
    ///                "x86_64-linux"
    ///              ]
    ///            }
    ///          ],
    ///          "name": "test"
    ///        }
    ///      ]
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PackageGroup"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackageGroups {
        pub items: Vec<PackageGroup>,
    }
    impl From<&PackageGroups> for PackageGroups {
        fn from(value: &PackageGroups) -> Self {
            value.clone()
        }
    }
    ///PackageInfoSearch
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackageInfoSearch",
    ///  "examples": [
    ///    {
    ///      "attr_path": "foo.bar.curl",
    ///      "description": "A very nice Item",
    ///      "name": "curl",
    ///      "pkg_path": "foo.bar.curl",
    ///      "pname": "curl",
    ///      "stabilities": [
    ///        "stable",
    ///        "unstable"
    ///      ],
    ///      "system": "x86_64-linux"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "attr_path",
    ///    "catalog",
    ///    "description",
    ///    "name",
    ///    "pkg_path",
    ///    "pname",
    ///    "stabilities",
    ///    "system",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "attr_path": {
    ///      "title": "Attr Path",
    ///      "type": "string"
    ///    },
    ///    "catalog": {
    ///      "title": "Catalog",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "title": "Description",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "pkg_path": {
    ///      "title": "Pkg Path",
    ///      "type": "string"
    ///    },
    ///    "pname": {
    ///      "title": "Pname",
    ///      "type": "string"
    ///    },
    ///    "stabilities": {
    ///      "title": "Stabilities",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "system": {
    ///      "$ref": "#/components/schemas/SystemEnum"
    ///    },
    ///    "version": {
    ///      "title": "Version",
    ///      "description": "While version should always be present, (and is required in the PackageResolutionInfo model), there are cases where it has been historically optional and thus is carried forward here.  Published derivations have an Optional version and this same model is used for both published derivations and base catalog derivations.  For this reason we cannot make it required here until/if we unify those models and ensure every derivation does in fact have a version.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackageInfoSearch {
        pub attr_path: String,
        pub catalog: Option<String>,
        pub description: Option<String>,
        pub name: String,
        pub pkg_path: String,
        pub pname: String,
        pub stabilities: Vec<String>,
        pub system: SystemEnum,
        ///While version should always be present, (and is required in the PackageResolutionInfo model), there are cases where it has been historically optional and thus is carried forward here.  Published derivations have an Optional version and this same model is used for both published derivations and base catalog derivations.  For this reason we cannot make it required here until/if we unify those models and ensure every derivation does in fact have a version.
        pub version: Option<String>,
    }
    impl From<&PackageInfoSearch> for PackageInfoSearch {
        fn from(value: &PackageInfoSearch) -> Self {
            value.clone()
        }
    }
    ///PackageName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Package Name",
    ///  "type": "string",
    ///  "pattern": "[a-zA-Z0-9\\.\\-_]{3,128}"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct PackageName(String);
    impl std::ops::Deref for PackageName {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<PackageName> for String {
        fn from(value: PackageName) -> Self {
            value.0
        }
    }
    impl From<&PackageName> for PackageName {
        fn from(value: &PackageName) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for PackageName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("[a-zA-Z0-9\\.\\-_]{3,128}")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"[a-zA-Z0-9\\.\\-_]{3,128}\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for PackageName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for PackageName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for PackageName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for PackageName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///PackageResolutionInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackageResolutionInfo",
    ///  "type": "object",
    ///  "required": [
    ///    "attr_path",
    ///    "broken",
    ///    "derivation",
    ///    "description",
    ///    "insecure",
    ///    "license",
    ///    "locked_url",
    ///    "missing_builds",
    ///    "name",
    ///    "outputs",
    ///    "outputs_to_install",
    ///    "pkg_path",
    ///    "pname",
    ///    "rev",
    ///    "rev_count",
    ///    "rev_date",
    ///    "scrape_date",
    ///    "stabilities",
    ///    "system",
    ///    "unfree",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "attr_path": {
    ///      "title": "Attr Path",
    ///      "type": "string"
    ///    },
    ///    "broken": {
    ///      "title": "Broken",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "cache_uri": {
    ///      "title": "Cache Uri",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "catalog": {
    ///      "title": "Catalog",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "derivation": {
    ///      "title": "Derivation",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "title": "Description",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "insecure": {
    ///      "title": "Insecure",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "license": {
    ///      "title": "License",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "locked_url": {
    ///      "title": "Locked Url",
    ///      "type": "string"
    ///    },
    ///    "missing_builds": {
    ///      "title": "Missing Builds",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "outputs": {
    ///      "title": "Outputs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Output"
    ///      }
    ///    },
    ///    "outputs_to_install": {
    ///      "title": "Outputs To Install",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "pkg_path": {
    ///      "title": "Pkg Path",
    ///      "type": "string"
    ///    },
    ///    "pname": {
    ///      "title": "Pname",
    ///      "type": "string"
    ///    },
    ///    "rev": {
    ///      "title": "Rev",
    ///      "type": "string"
    ///    },
    ///    "rev_count": {
    ///      "title": "Rev Count",
    ///      "type": "integer"
    ///    },
    ///    "rev_date": {
    ///      "title": "Rev Date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "scrape_date": {
    ///      "title": "Scrape Date",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "stabilities": {
    ///      "title": "Stabilities",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "system": {
    ///      "$ref": "#/components/schemas/SystemEnum"
    ///    },
    ///    "unfree": {
    ///      "title": "Unfree",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "title": "Version",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackageResolutionInfo {
        pub attr_path: String,
        pub broken: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub catalog: Option<String>,
        pub derivation: String,
        pub description: Option<String>,
        pub insecure: Option<bool>,
        pub license: Option<String>,
        pub locked_url: String,
        pub missing_builds: Option<bool>,
        pub name: String,
        pub outputs: Vec<Output>,
        pub outputs_to_install: Option<Vec<String>>,
        pub pkg_path: String,
        pub pname: String,
        pub rev: String,
        pub rev_count: i64,
        pub rev_date: chrono::DateTime<chrono::offset::Utc>,
        pub scrape_date: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub stabilities: Option<Vec<String>>,
        pub system: SystemEnum,
        pub unfree: Option<bool>,
        pub version: String,
    }
    impl From<&PackageResolutionInfo> for PackageResolutionInfo {
        fn from(value: &PackageResolutionInfo) -> Self {
            value.clone()
        }
    }
    ///PackageSearchResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackageSearchResult",
    ///  "examples": [
    ///    [
    ///      {
    ///        "attr_path": "foo.bar.curl",
    ///        "description": "A very nice Item",
    ///        "name": "curl",
    ///        "pkg_path": "foo.bar.curl",
    ///        "pname": "curl",
    ///        "stabilities": [
    ///          "stable",
    ///          "unstable"
    ///        ],
    ///        "system": "x86_64-linux"
    ///      }
    ///    ]
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "items",
    ///    "total_count"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PackageInfoSearch"
    ///      }
    ///    },
    ///    "total_count": {
    ///      "title": "Total Count",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackageSearchResult {
        pub items: Vec<PackageInfoSearch>,
        pub total_count: i64,
    }
    impl From<&PackageSearchResult> for PackageSearchResult {
        fn from(value: &PackageSearchResult) -> Self {
            value.clone()
        }
    }
    ///PackagesResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PackagesResult",
    ///  "type": "object",
    ///  "required": [
    ///    "items",
    ///    "total_count"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PackageResolutionInfo"
    ///      }
    ///    },
    ///    "total_count": {
    ///      "title": "Total Count",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PackagesResult {
        pub items: Vec<PackageResolutionInfo>,
        pub total_count: i64,
    }
    impl From<&PackagesResult> for PackagesResult {
        fn from(value: &PackagesResult) -> Self {
            value.clone()
        }
    }
    ///Params
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "params",
    ///  "type": "object",
    ///  "properties": {
    ///    "resolve_package": {
    ///      "title": "Resolve Package",
    ///      "default": "cowsay",
    ///      "type": "string"
    ///    },
    ///    "resolve_systems": {
    ///      "title": "Resolve Systems",
    ///      "default": [
    ///        "x86_64-linux"
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SystemEnum"
    ///      }
    ///    },
    ///    "search_system": {
    ///      "$ref": "#/components/schemas/SystemEnum"
    ///    },
    ///    "search_term": {
    ///      "title": "Search Term",
    ///      "default": "in Go",
    ///      "type": "string"
    ///    },
    ///    "show_term": {
    ///      "title": "Show Term",
    ///      "default": "hello",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct Params {
        #[serde(default = "defaults::params_resolve_package")]
        pub resolve_package: String,
        #[serde(default = "defaults::params_resolve_systems")]
        pub resolve_systems: Vec<SystemEnum>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub search_system: Option<SystemEnum>,
        #[serde(default = "defaults::params_search_term")]
        pub search_term: String,
        #[serde(default = "defaults::params_show_term")]
        pub show_term: String,
    }
    impl From<&Params> for Params {
        fn from(value: &Params) -> Self {
            value.clone()
        }
    }
    ///PkgPathsResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PkgPathsResult",
    ///  "type": "object",
    ///  "required": [
    ///    "items",
    ///    "total_count"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "total_count": {
    ///      "title": "Total Count",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PkgPathsResult {
        pub items: Vec<String>,
        pub total_count: i64,
    }
    impl From<&PkgPathsResult> for PkgPathsResult {
        fn from(value: &PkgPathsResult) -> Self {
            value.clone()
        }
    }
    ///PublishRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PublishRequest",
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PublishRequest(pub serde_json::Map<String, serde_json::Value>);
    impl std::ops::Deref for PublishRequest {
        type Target = serde_json::Map<String, serde_json::Value>;
        fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
            &self.0
        }
    }
    impl From<PublishRequest> for serde_json::Map<String, serde_json::Value> {
        fn from(value: PublishRequest) -> Self {
            value.0
        }
    }
    impl From<&PublishRequest> for PublishRequest {
        fn from(value: &PublishRequest) -> Self {
            value.clone()
        }
    }
    impl From<serde_json::Map<String, serde_json::Value>> for PublishRequest {
        fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///PublishResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PublishResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "catalog_store_config"
    ///  ],
    ///  "properties": {
    ///    "catalog_store_config": {
    ///      "$ref": "#/components/schemas/CatalogStoreConfig"
    ///    },
    ///    "ingress_auth": {
    ///      "title": "Ingress Auth",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ]
    ///    },
    ///    "ingress_uri": {
    ///      "title": "Ingress Uri",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct PublishResponse {
        pub catalog_store_config: crate::types::CatalogStoreConfig,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ingress_auth: Option<serde_json::Map<String, serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ingress_uri: Option<String>,
    }
    impl From<&PublishResponse> for PublishResponse {
        fn from(value: &PublishResponse) -> Self {
            value.clone()
        }
    }
    ///ResolutionMessageGeneral
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ResolutionMessageGeneral",
    ///  "type": "object",
    ///  "required": [
    ///    "context",
    ///    "level",
    ///    "message",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "context": {
    ///      "title": "Context",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "level": {
    ///      "$ref": "#/components/schemas/MessageLevel"
    ///    },
    ///    "message": {
    ///      "title": "Message",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "$ref": "#/components/schemas/MessageType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResolutionMessageGeneral {
        pub context: std::collections::HashMap<String, String>,
        pub level: MessageLevel,
        pub message: String,
        #[serde(rename = "type")]
        pub type_: crate::error::MessageType,
    }
    impl From<&ResolutionMessageGeneral> for ResolutionMessageGeneral {
        fn from(value: &ResolutionMessageGeneral) -> Self {
            value.clone()
        }
    }
    ///ResolvedPackageDescriptor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ResolvedPackageDescriptor",
    ///  "type": "object",
    ///  "required": [
    ///    "attr_path",
    ///    "broken",
    ///    "derivation",
    ///    "description",
    ///    "insecure",
    ///    "install_id",
    ///    "license",
    ///    "locked_url",
    ///    "missing_builds",
    ///    "name",
    ///    "outputs",
    ///    "outputs_to_install",
    ///    "pkg_path",
    ///    "pname",
    ///    "rev",
    ///    "rev_count",
    ///    "rev_date",
    ///    "scrape_date",
    ///    "stabilities",
    ///    "system",
    ///    "unfree",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "attr_path": {
    ///      "title": "Attr Path",
    ///      "type": "string"
    ///    },
    ///    "broken": {
    ///      "title": "Broken",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "cache_uri": {
    ///      "title": "Cache Uri",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "catalog": {
    ///      "title": "Catalog",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "derivation": {
    ///      "title": "Derivation",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "title": "Description",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "insecure": {
    ///      "title": "Insecure",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "install_id": {
    ///      "title": "Install Id",
    ///      "type": "string"
    ///    },
    ///    "license": {
    ///      "title": "License",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "locked_url": {
    ///      "title": "Locked Url",
    ///      "type": "string"
    ///    },
    ///    "missing_builds": {
    ///      "title": "Missing Builds",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "outputs": {
    ///      "title": "Outputs",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Output"
    ///      }
    ///    },
    ///    "outputs_to_install": {
    ///      "title": "Outputs To Install",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "pkg_path": {
    ///      "title": "Pkg Path",
    ///      "type": "string"
    ///    },
    ///    "pname": {
    ///      "title": "Pname",
    ///      "type": "string"
    ///    },
    ///    "rev": {
    ///      "title": "Rev",
    ///      "type": "string"
    ///    },
    ///    "rev_count": {
    ///      "title": "Rev Count",
    ///      "type": "integer"
    ///    },
    ///    "rev_date": {
    ///      "title": "Rev Date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "scrape_date": {
    ///      "title": "Scrape Date",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "stabilities": {
    ///      "title": "Stabilities",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "system": {
    ///      "$ref": "#/components/schemas/SystemEnum"
    ///    },
    ///    "unfree": {
    ///      "title": "Unfree",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "title": "Version",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResolvedPackageDescriptor {
        pub attr_path: String,
        pub broken: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub catalog: Option<String>,
        pub derivation: String,
        pub description: Option<String>,
        pub insecure: Option<bool>,
        pub install_id: String,
        pub license: Option<String>,
        pub locked_url: String,
        pub missing_builds: Option<bool>,
        pub name: String,
        pub outputs: Vec<Output>,
        pub outputs_to_install: Option<Vec<String>>,
        pub pkg_path: String,
        pub pname: String,
        pub rev: String,
        pub rev_count: i64,
        pub rev_date: chrono::DateTime<chrono::offset::Utc>,
        pub scrape_date: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub stabilities: Option<Vec<String>>,
        pub system: SystemEnum,
        pub unfree: Option<bool>,
        pub version: String,
    }
    impl From<&ResolvedPackageDescriptor> for ResolvedPackageDescriptor {
        fn from(value: &ResolvedPackageDescriptor) -> Self {
            value.clone()
        }
    }
    ///ResolvedPackageGroup
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ResolvedPackageGroup",
    ///  "examples": [
    ///    {
    ///      "attr_path": "foo.bar.curl",
    ///      "description": "A very nice Item",
    ///      "license": "foo",
    ///      "locked_url": "git:git?rev=xyz",
    ///      "name": "curl",
    ///      "outputs": "{}",
    ///      "outputs_to_install": "{}",
    ///      "pkg_path": "foo.bar.curl",
    ///      "pname": "curl",
    ///      "rev": "xyz",
    ///      "rev_count": 4,
    ///      "rev_date": 0,
    ///      "stabilities": [
    ///        "stable",
    ///        "unstable"
    ///      ],
    ///      "system": "x86_64-linux",
    ///      "version": "1.0"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "messages",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "messages": {
    ///      "title": "Messages",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResolutionMessageGeneral"
    ///      }
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "page": {
    ///      "$ref": "#/components/schemas/CatalogPage"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResolvedPackageGroup {
        pub messages: Vec<ResolutionMessageGeneral>,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub page: Option<CatalogPage>,
    }
    impl From<&ResolvedPackageGroup> for ResolvedPackageGroup {
        fn from(value: &ResolvedPackageGroup) -> Self {
            value.clone()
        }
    }
    ///ResolvedPackageGroups
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ResolvedPackageGroups",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResolvedPackageGroup"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResolvedPackageGroups {
        pub items: Vec<ResolvedPackageGroup>,
    }
    impl From<&ResolvedPackageGroups> for ResolvedPackageGroups {
        fn from(value: &ResolvedPackageGroups) -> Self {
            value.clone()
        }
    }
    ///SearchTerm
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "Search Term",
    ///  "type": "string",
    ///  "pattern": "[a-zA-Z0-9\\-\\.\\\\@%_,]{2,200}"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub struct SearchTerm(String);
    impl std::ops::Deref for SearchTerm {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }
    impl From<SearchTerm> for String {
        fn from(value: SearchTerm) -> Self {
            value.0
        }
    }
    impl From<&SearchTerm> for SearchTerm {
        fn from(value: &SearchTerm) -> Self {
            value.clone()
        }
    }
    impl std::str::FromStr for SearchTerm {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("[a-zA-Z0-9\\-\\.\\\\@%_,]{2,200}")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err(
                    "doesn't match pattern \"[a-zA-Z0-9\\-\\.\\\\@%_,]{2,200}\"".into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }
    impl std::convert::TryFrom<&str> for SearchTerm {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SearchTerm {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SearchTerm {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> serde::Deserialize<'de> for SearchTerm {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///ServiceStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ServiceStatus",
    ///  "type": "object",
    ///  "required": [
    ///    "service_version",
    ///    "start_tm",
    ///    "uptime_pretty",
    ///    "uptime_seconds"
    ///  ],
    ///  "properties": {
    ///    "service_version": {
    ///      "title": "Service Version",
    ///      "type": "string"
    ///    },
    ///    "start_tm": {
    ///      "title": "Start Tm",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "uptime_pretty": {
    ///      "title": "Uptime Pretty",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "uptime_seconds": {
    ///      "title": "Uptime Seconds",
    ///      "readOnly": true,
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct ServiceStatus {
        pub service_version: String,
        pub start_tm: chrono::DateTime<chrono::offset::Utc>,
        pub uptime_pretty: String,
        pub uptime_seconds: f64,
    }
    impl From<&ServiceStatus> for ServiceStatus {
        fn from(value: &ServiceStatus) -> Self {
            value.clone()
        }
    }
    ///StoreInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "StoreInfo",
    ///  "type": "object",
    ///  "required": [
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "auth": {
    ///      "title": "Auth",
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ]
    ///    },
    ///    "url": {
    ///      "title": "Url",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct StoreInfo {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth: Option<serde_json::Map<String, serde_json::Value>>,
        pub url: String,
    }
    impl From<&StoreInfo> for StoreInfo {
        fn from(value: &StoreInfo) -> Self {
            value.clone()
        }
    }
    ///StoreInfoRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "StoreInfoRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "outpaths"
    ///  ],
    ///  "properties": {
    ///    "drv_paths": {
    ///      "title": "Drv Paths",
    ///      "deprecated": true,
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "outpaths": {
    ///      "title": "Outpaths",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct StoreInfoRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub drv_paths: Option<Vec<String>>,
        pub outpaths: Vec<String>,
    }
    impl From<&StoreInfoRequest> for StoreInfoRequest {
        fn from(value: &StoreInfoRequest) -> Self {
            value.clone()
        }
    }
    ///StoreInfoResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "StoreInfoResponse",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/StoreInfo"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct StoreInfoResponse {
        pub items: std::collections::HashMap<String, Vec<StoreInfo>>,
    }
    impl From<&StoreInfoResponse> for StoreInfoResponse {
        fn from(value: &StoreInfoResponse) -> Self {
            value.clone()
        }
    }
    ///SystemEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "SystemEnum",
    ///  "type": "string",
    ///  "enum": [
    ///    "aarch64-darwin",
    ///    "aarch64-linux",
    ///    "x86_64-darwin",
    ///    "x86_64-linux",
    ///    "invalid"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum SystemEnum {
        #[serde(rename = "aarch64-darwin")]
        Aarch64Darwin,
        #[serde(rename = "aarch64-linux")]
        Aarch64Linux,
        #[serde(rename = "x86_64-darwin")]
        X8664Darwin,
        #[serde(rename = "x86_64-linux")]
        X8664Linux,
        #[serde(rename = "invalid")]
        Invalid,
    }
    impl From<&SystemEnum> for SystemEnum {
        fn from(value: &SystemEnum) -> Self {
            value.clone()
        }
    }
    impl ToString for SystemEnum {
        fn to_string(&self) -> String {
            match *self {
                Self::Aarch64Darwin => "aarch64-darwin".to_string(),
                Self::Aarch64Linux => "aarch64-linux".to_string(),
                Self::X8664Darwin => "x86_64-darwin".to_string(),
                Self::X8664Linux => "x86_64-linux".to_string(),
                Self::Invalid => "invalid".to_string(),
            }
        }
    }
    impl std::str::FromStr for SystemEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "aarch64-darwin" => Ok(Self::Aarch64Darwin),
                "aarch64-linux" => Ok(Self::Aarch64Linux),
                "x86_64-darwin" => Ok(Self::X8664Darwin),
                "x86_64-linux" => Ok(Self::X8664Linux),
                "invalid" => Ok(Self::Invalid),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for SystemEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for SystemEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for SystemEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///UserBuild
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserBuild",
    ///  "examples": [
    ///    {
    ///      "derivation": {
    ///        "description": "A very nice derivation",
    ///        "drv_path": "foo.bar.curl",
    ///        "license": "GnuFoo",
    ///        "name": "mydrv",
    ///        "outputs": {
    ///          "bin": "/nix/store/foo"
    ///        },
    ///        "outputs_to_install": [
    ///          "bin"
    ///        ],
    ///        "pname": "mydrv",
    ///        "system": "x86_64-linux",
    ///        "version": "1.0"
    ///      },
    ///      "locked_base_catalog_url": "https://github.com/flox/nixpkgs?rev=99dc8785f6a0adac95f5e2ab05cc2e1bf666d172",
    ///      "rev": "99dc8785f6a0adac95f5e2ab05cc2e1bf666d172",
    ///      "rev_count": 12345,
    ///      "rev_date": "2021-09-01T00:00:00Z",
    ///      "url": "https://github.com/org/example"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "derivation",
    ///    "rev",
    ///    "rev_count",
    ///    "rev_date",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "cache_uri": {
    ///      "title": "Cache Uri",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "derivation": {
    ///      "$ref": "#/components/schemas/UserDerivation-Output"
    ///    },
    ///    "locked_base_catalog_url": {
    ///      "title": "Locked Base Catalog Url",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rev": {
    ///      "title": "Rev",
    ///      "type": "string"
    ///    },
    ///    "rev_count": {
    ///      "title": "Rev Count",
    ///      "type": "integer"
    ///    },
    ///    "rev_date": {
    ///      "title": "Rev Date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "url": {
    ///      "title": "Url",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserBuild {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_uri: Option<String>,
        pub derivation: UserDerivationOutput,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locked_base_catalog_url: Option<String>,
        pub rev: String,
        pub rev_count: i64,
        pub rev_date: chrono::DateTime<chrono::offset::Utc>,
        pub url: String,
    }
    impl From<&UserBuild> for UserBuild {
        fn from(value: &UserBuild) -> Self {
            value.clone()
        }
    }
    ///UserBuildCreationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserBuildCreationResponse",
    ///  "type": "object",
    ///  "properties": {
    ///    "store": {
    ///      "$ref": "#/components/schemas/StoreInfo"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserBuildCreationResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub store: Option<StoreInfo>,
    }
    impl From<&UserBuildCreationResponse> for UserBuildCreationResponse {
        fn from(value: &UserBuildCreationResponse) -> Self {
            value.clone()
        }
    }
    ///UserBuildList
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserBuildList",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserBuild"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserBuildList {
        pub items: Vec<UserBuild>,
    }
    impl From<&UserBuildList> for UserBuildList {
        fn from(value: &UserBuildList) -> Self {
            value.clone()
        }
    }
    ///UserBuildPublish
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserBuildPublish",
    ///  "examples": [
    ///    {
    ///      "derivation": {
    ///        "description": "A very nice derivation",
    ///        "drv_path": "foo.bar.curl",
    ///        "license": "GnuFoo",
    ///        "name": "mydrv",
    ///        "outputs": {
    ///          "bin": "/nix/store/foo"
    ///        },
    ///        "outputs_to_install": [
    ///          "bin"
    ///        ],
    ///        "pname": "mydrv",
    ///        "system": "x86_64-linux",
    ///        "version": "1.0"
    ///      },
    ///      "locked_base_catalog_url": "https://github.com/flox/nixpkgs?rev=99dc8785f6a0adac95f5e2ab05cc2e1bf666d172",
    ///      "rev": "99dc8785f6a0adac95f5e2ab05cc2e1bf666d172",
    ///      "rev_count": 12345,
    ///      "rev_date": "2021-09-01T00:00:00Z",
    ///      "url": "https://github.com/org/example"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "derivation",
    ///    "rev",
    ///    "rev_count",
    ///    "rev_date",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "cache_uri": {
    ///      "title": "Cache Uri",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "derivation": {
    ///      "$ref": "#/components/schemas/UserDerivation-Input"
    ///    },
    ///    "locked_base_catalog_url": {
    ///      "title": "Locked Base Catalog Url",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "narinfos": {
    ///      "$ref": "#/components/schemas/NarInfos"
    ///    },
    ///    "narinfos_source_url": {
    ///      "title": "Narinfos Source Url",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "narinfos_source_version": {
    ///      "title": "Narinfos Source Version",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "rev": {
    ///      "title": "Rev",
    ///      "type": "string"
    ///    },
    ///    "rev_count": {
    ///      "title": "Rev Count",
    ///      "type": "integer"
    ///    },
    ///    "rev_date": {
    ///      "title": "Rev Date",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "url": {
    ///      "title": "Url",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserBuildPublish {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_uri: Option<String>,
        pub derivation: UserDerivationInput,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locked_base_catalog_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub narinfos: Option<NarInfos>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub narinfos_source_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub narinfos_source_version: Option<i64>,
        pub rev: String,
        pub rev_count: i64,
        pub rev_date: chrono::DateTime<chrono::offset::Utc>,
        pub url: String,
    }
    impl From<&UserBuildPublish> for UserBuildPublish {
        fn from(value: &UserBuildPublish) -> Self {
            value.clone()
        }
    }
    ///UserCatalog
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserCatalog",
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "title": "Created At",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "title": "Id",
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "owner_handle": {
    ///      "title": "Owner Handle",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserCatalog {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: i64,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner_handle: Option<String>,
    }
    impl From<&UserCatalog> for UserCatalog {
        fn from(value: &UserCatalog) -> Self {
            value.clone()
        }
    }
    ///UserDerivationInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserDerivation",
    ///  "examples": [
    ///    {
    ///      "description": "A very nice derivation",
    ///      "drv_path": "foo.bar.curl",
    ///      "license": "GnuFoo",
    ///      "name": "mydrv",
    ///      "outputs": {
    ///        "bin": "/nix/store/foo"
    ///      },
    ///      "outputs_to_install": [
    ///        "bin"
    ///      ],
    ///      "pname": "mydrv",
    ///      "system": "x86_64-linux",
    ///      "version": "1.0"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "drv_path",
    ///    "name",
    ///    "outputs",
    ///    "system"
    ///  ],
    ///  "properties": {
    ///    "broken": {
    ///      "title": "Broken",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "title": "Description",
    ///      "type": "string"
    ///    },
    ///    "drv_path": {
    ///      "title": "Drv Path",
    ///      "type": "string"
    ///    },
    ///    "license": {
    ///      "title": "License",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "outputs": {
    ///      "$ref": "#/components/schemas/Outputs"
    ///    },
    ///    "outputs_to_install": {
    ///      "title": "Outputs To Install",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "pname": {
    ///      "title": "Pname",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "system": {
    ///      "$ref": "#/components/schemas/SystemEnum"
    ///    },
    ///    "unfree": {
    ///      "title": "Unfree",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "title": "Version",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserDerivationInput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub broken: Option<bool>,
        pub description: String,
        pub drv_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub license: Option<String>,
        pub name: String,
        pub outputs: Outputs,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub outputs_to_install: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pname: Option<String>,
        pub system: SystemEnum,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unfree: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    impl From<&UserDerivationInput> for UserDerivationInput {
        fn from(value: &UserDerivationInput) -> Self {
            value.clone()
        }
    }
    ///UserDerivationOutput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserDerivation",
    ///  "examples": [
    ///    {
    ///      "description": "A very nice derivation",
    ///      "drv_path": "foo.bar.curl",
    ///      "license": "GnuFoo",
    ///      "name": "mydrv",
    ///      "outputs": {
    ///        "bin": "/nix/store/foo"
    ///      },
    ///      "outputs_to_install": [
    ///        "bin"
    ///      ],
    ///      "pname": "mydrv",
    ///      "system": "x86_64-linux",
    ///      "version": "1.0"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "drv_path",
    ///    "name",
    ///    "outputs",
    ///    "system"
    ///  ],
    ///  "properties": {
    ///    "broken": {
    ///      "title": "Broken",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "title": "Description",
    ///      "type": "string"
    ///    },
    ///    "drv_path": {
    ///      "title": "Drv Path",
    ///      "type": "string"
    ///    },
    ///    "license": {
    ///      "title": "License",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "outputs": {
    ///      "$ref": "#/components/schemas/Outputs"
    ///    },
    ///    "outputs_to_install": {
    ///      "title": "Outputs To Install",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "pname": {
    ///      "title": "Pname",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "system": {
    ///      "$ref": "#/components/schemas/SystemEnum"
    ///    },
    ///    "unfree": {
    ///      "title": "Unfree",
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "title": "Version",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserDerivationOutput {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub broken: Option<bool>,
        pub description: String,
        pub drv_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub license: Option<String>,
        pub name: String,
        pub outputs: Outputs,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub outputs_to_install: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pname: Option<String>,
        pub system: SystemEnum,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unfree: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    impl From<&UserDerivationOutput> for UserDerivationOutput {
        fn from(value: &UserDerivationOutput) -> Self {
            value.clone()
        }
    }
    ///UserPackage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserPackage",
    ///  "type": "object",
    ///  "required": [
    ///    "catalog",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "catalog": {
    ///      "title": "Catalog",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "title": "Name",
    ///      "type": "string"
    ///    },
    ///    "original_url": {
    ///      "title": "Original Url",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserPackage {
        pub catalog: String,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub original_url: Option<String>,
    }
    impl From<&UserPackage> for UserPackage {
        fn from(value: &UserPackage) -> Self {
            value.clone()
        }
    }
    ///UserPackageCreate
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserPackageCreate",
    ///  "type": "object",
    ///  "properties": {
    ///    "original_url": {
    ///      "title": "Original Url",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserPackageCreate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub original_url: Option<String>,
    }
    impl From<&UserPackageCreate> for UserPackageCreate {
        fn from(value: &UserPackageCreate) -> Self {
            value.clone()
        }
    }
    ///UserPackageList
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "UserPackageList",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "title": "Items",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserPackage"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct UserPackageList {
        pub items: Vec<UserPackage>,
    }
    impl From<&UserPackageList> for UserPackageList {
        fn from(value: &UserPackageList) -> Self {
            value.clone()
        }
    }
    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn catalog_share_info_allow_read_users() -> Option<Vec<String>> {
            Some(vec![])
        }
        pub(super) fn catalog_store_config_meta_only_store_type() -> String {
            "meta-only".to_string()
        }
        pub(super) fn catalog_store_config_null_store_type() -> String {
            "null".to_string()
        }
        pub(super) fn catalog_store_config_publisher_store_type() -> String {
            "publisher".to_string()
        }
        pub(super) fn package_descriptor_allow_broken() -> Option<bool> {
            Some(false)
        }
        pub(super) fn package_descriptor_allow_insecure() -> Option<bool> {
            Some(false)
        }
        pub(super) fn package_descriptor_allow_missing_builds() -> Option<bool> {
            Some(false)
        }
        pub(super) fn package_descriptor_allow_pre_releases() -> Option<bool> {
            Some(false)
        }
        pub(super) fn package_descriptor_allow_unfree() -> Option<bool> {
            Some(true)
        }
        pub(super) fn params_resolve_package() -> String {
            "cowsay".to_string()
        }
        pub(super) fn params_resolve_systems() -> Vec<super::SystemEnum> {
            vec![super::SystemEnum::X8664Linux]
        }
        pub(super) fn params_search_term() -> String {
            "in Go".to_string()
        }
        pub(super) fn params_show_term() -> String {
            "hello".to_string()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Flox Catalog Service


# Flox Catalog Service API

![packages](https://api.preview.flox.dev/api/v1/catalog/status/badges/packages.svg)


Version: vundefined*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "vundefined"
    }
}
#[allow(clippy::all)]
impl Client {
    /**Create a new user catalog

Create a new user catalog

Required Query Parameters:
- **name**: The name of the new catalog


Returns:
- **UserCatalog**: The new user catalog

Sends a `POST` request to `/api/v1/catalog/catalogs/`

*/
    pub async fn create_catalog_api_v1_catalog_catalogs_post<'a>(
        &'a self,
        name: &'a types::Name,
    ) -> Result<ResponseValue<types::UserCatalog>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/catalogs/", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("name", name.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get user catalog metadata

Get user catalog metadata

Required Query Parameters:
- **catalog_name**: The name of the catalog


Returns:
- **UserCatalog**: The user catalog

Sends a `GET` request to `/api/v1/catalog/catalogs/{catalog_name}`

*/
    pub async fn get_catalog_api_v1_catalog_catalogs_catalog_name_get<'a>(
        &'a self,
        catalog_name: &'a types::CatalogName,
    ) -> Result<ResponseValue<types::UserCatalog>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}", self.baseurl, encode_path(& catalog_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Delete a user catalog

Delete a user catalog

Required Query Parameters:
- **catalog_name**: The name of catalog to delete


Returns:
- **None**

Sends a `DELETE` request to `/api/v1/catalog/catalogs/{catalog_name}`

*/
    pub async fn delete_catalog_api_v1_catalog_catalogs_catalog_name_delete<'a>(
        &'a self,
        catalog_name: &'a types::CatalogName,
    ) -> Result<ResponseValue<serde_json::Value>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}", self.baseurl, encode_path(& catalog_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            501u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List packages available in a catalog

Lists available packages in a catalog

Path Parameters:
- **catalog_name**: The name of the catalog

Returns:
- **UserPackageList**

Sends a `GET` request to `/api/v1/catalog/catalogs/{catalog_name}/packages`

*/
    pub async fn get_catalog_packages_api_v1_catalog_catalogs_catalog_name_packages_get<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
    ) -> Result<ResponseValue<types::UserPackageList>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages", self.baseurl, encode_path(&
            catalog_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Create a new package in a user catalog

Creates a catalog package

Path Parameters:
- **catalog_name**: The name of catalog to place the package into

Required Query Parameters:
- **name**: The name of package (attr_path) to create

Returns:
- **UserPackage**

Sends a `POST` request to `/api/v1/catalog/catalogs/{catalog_name}/packages`

*/
    pub async fn create_catalog_package_api_v1_catalog_catalogs_catalog_name_packages_post<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        name: &'a types::Name,
        body: &'a types::UserPackageCreate,
    ) -> Result<ResponseValue<types::UserPackage>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages", self.baseurl, encode_path(&
            catalog_name.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        query.push(("name", name.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            201u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            409u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get package info

Get package info

Path Parameters:
- **catalog_name**: The name of the catalog
- **package_name**: The name of the package

Returns:
- **UserPackage**

Sends a `GET` request to `/api/v1/catalog/catalogs/{catalog_name}/packages/{package_name}`

*/
    pub async fn get_catalog_package_api_v1_catalog_catalogs_catalog_name_packages_package_name_get<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        package_name: &'a types::PackageName,
    ) -> Result<ResponseValue<types::UserPackage>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages/{}", self.baseurl, encode_path(&
            catalog_name.to_string()), encode_path(& package_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get a list of builds for a given package

Get the list of builds for a given package

Path Parameters:
- **catalog_name**: The name of the catalog
- **package_name**: The name of the package

Returns:
- **UserBuildList**

Sends a `GET` request to `/api/v1/catalog/catalogs/{catalog_name}/packages/{package_name}/builds`

*/
    pub async fn get_package_builds_api_v1_catalog_catalogs_catalog_name_packages_package_name_builds_get<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        package_name: &'a types::PackageName,
    ) -> Result<ResponseValue<types::UserBuildList>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages/{}/builds", self.baseurl,
            encode_path(& catalog_name.to_string()), encode_path(& package_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Update a build of a particular package

Create or update a build of a package

Path Parameters:
- **catalog_name**: The name of the catalog
- **package_name**: The name of the package
Body Content:
- **UserBuildPublish**: The build info to submit

Returns:
- **UserBuildCreationResponse**

Sends a `PUT` request to `/api/v1/catalog/catalogs/{catalog_name}/packages/{package_name}/builds`

*/
    pub async fn create_package_build_api_v1_catalog_catalogs_catalog_name_packages_package_name_builds_put<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        package_name: &'a types::PackageName,
        body: &'a types::UserBuildPublish,
    ) -> Result<
        ResponseValue<types::UserBuildCreationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages/{}/builds", self.baseurl,
            encode_path(& catalog_name.to_string()), encode_path(& package_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Submit a build of a particular package

Create or update a build of a package

Path Parameters:
- **catalog_name**: The name of the catalog
- **package_name**: The name of the package
Body Content:
- **UserBuildPublish**: The build info to submit

Returns:
- **UserBuildCreationResponse**

Sends a `POST` request to `/api/v1/catalog/catalogs/{catalog_name}/packages/{package_name}/builds`

*/
    pub async fn create_package_build_api_v1_catalog_catalogs_catalog_name_packages_package_name_builds_post<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        package_name: &'a types::PackageName,
        body: &'a types::UserBuildPublish,
    ) -> Result<
        ResponseValue<types::UserBuildCreationResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages/{}/builds", self.baseurl,
            encode_path(& catalog_name.to_string()), encode_path(& package_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            201u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Request access and info to publish a package

Request access and informatin to publish a package to this catalog.
Path Parameters:
- **catalog_name**: The name of the catalog
- **package_name**: The name of the package
Body Content:
- **PublishRequest**: The information needed to publish to the catalog
Returns:
- **PublishRequestResponse**

Sends a `POST` request to `/api/v1/catalog/catalogs/{catalog_name}/packages/{package_name}/publish`

*/
    pub async fn publish_request_api_v1_catalog_catalogs_catalog_name_packages_package_name_publish_post<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        package_name: &'a types::PackageName,
        body: &'a types::PublishRequest,
    ) -> Result<ResponseValue<types::PublishResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages/{}/publish", self.baseurl,
            encode_path(& catalog_name.to_string()), encode_path(& package_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Request access and info to publish a package

Request access and informatin to publish a package to this catalog.
Path Parameters:
- **catalog_name**: The name of the catalog
- **package_name**: The name of the package
Body Content:
- **PublishRequest**: The information needed to publish to the catalog
Returns:
- **PublishRequestResponse**

Sends a `POST` request to `/api/v1/catalog/catalogs/{catalog_name}/packages/{package_name}/publish/info`

*/
    pub async fn publish_request_api_v1_catalog_catalogs_catalog_name_packages_package_name_publish_info_post<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        package_name: &'a types::PackageName,
        body: &'a types::PublishRequest,
    ) -> Result<ResponseValue<types::PublishResponse>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/packages/{}/publish/info", self.baseurl,
            encode_path(& catalog_name.to_string()), encode_path(& package_name
            .to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get usernames that have read access to a catalog

Get the list of usernames with read access to a catalog

Path Parameters:
- **catalog_name**: The name of the catalog

Returns:
- **CatalogShareInfo**: The users with read access to the catalog

Sends a `GET` request to `/api/v1/catalog/catalogs/{catalog_name}/sharing`

*/
    pub async fn get_catalog_sharing_api_v1_catalog_catalogs_catalog_name_sharing_get<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
    ) -> Result<ResponseValue<types::CatalogShareInfo>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/sharing", self.baseurl, encode_path(&
            catalog_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Add usernames to the read access list for a catalog

Add usernames to the read access list for a catalog

Path Parameters:
- **catalog_name**: The name of the catalog

Body Content:
- **CatalogShareInfo**: The users to add to the read access list

Returns:
- **CatalogShareInfo**: The users with read access to the catalog

Sends a `POST` request to `/api/v1/catalog/catalogs/{catalog_name}/sharing/add-read-users`

*/
    pub async fn add_catalog_sharing_api_v1_catalog_catalogs_catalog_name_sharing_add_read_users_post<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        body: &'a types::CatalogShareInfo,
    ) -> Result<ResponseValue<types::CatalogShareInfo>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/sharing/add-read-users", self.baseurl,
            encode_path(& catalog_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Remove usernames from the read access list for a catalog

Remove usernames from the read access list for a catalog

Path Parameters:
- **catalog_name**: The name of the catalog

Body Content:
- **CatalogShareInfo**: The users to remove from the read access list

Returns:
- **CatalogShareInfo**: The users with read access to the catalog

Sends a `POST` request to `/api/v1/catalog/catalogs/{catalog_name}/sharing/remove-read-users`

*/
    pub async fn remove_catalog_sharing_api_v1_catalog_catalogs_catalog_name_sharing_remove_read_users_post<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        body: &'a types::CatalogShareInfo,
    ) -> Result<ResponseValue<types::CatalogShareInfo>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/sharing/remove-read-users", self.baseurl,
            encode_path(& catalog_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get store config

Get store configuration

Sends a `GET` request to `/api/v1/catalog/catalogs/{catalog_name}/store/config`

*/
    pub async fn get_catalog_store_config_api_v1_catalog_catalogs_catalog_name_store_config_get<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
    ) -> Result<
        ResponseValue<crate::types::CatalogStoreConfig>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/store/config", self.baseurl, encode_path(&
            catalog_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Set store config

Update store configuration

Sends a `PUT` request to `/api/v1/catalog/catalogs/{catalog_name}/store/config`

*/
    pub async fn set_catalog_store_config_api_v1_catalog_catalogs_catalog_name_store_config_put<
        'a,
    >(
        &'a self,
        catalog_name: &'a types::CatalogName,
        body: &'a crate::types::CatalogStoreConfig,
    ) -> Result<
        ResponseValue<crate::types::CatalogStoreConfig>,
        Error<types::ErrorResponse>,
    > {
        let url = format!(
            "{}/api/v1/catalog/catalogs/{}/store/config", self.baseurl, encode_path(&
            catalog_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**List of valid pkg paths

Get the list of valid pkg paths for base catalog packages.

Sends a `GET` request to `/api/v1/catalog/info/pkg-paths`

*/
    pub async fn get_pkg_paths_api_v1_catalog_info_pkg_paths_get<'a>(
        &'a self,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<ResponseValue<types::PkgPathsResult>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/info/pkg-paths", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &page_size {
            query.push(("pageSize", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Shows available packages of a specific package

Returns a list of versions for a given attr_path

Required Query Parameters:
- **attr_path**: The attr_path, must be valid.

Optional Query Parameters:
- **page**: Optional page number for pagination (def = 0)
- **pageSize**: Optional page size for pagination (def = 10)

Returns:
- **PackagesResult**: A list of PackageResolutionInfo and the total result count

Sends a `GET` request to `/api/v1/catalog/packages/{attr_path}`

*/
    pub async fn packages_api_v1_catalog_packages_attr_path_get<'a>(
        &'a self,
        attr_path: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<ResponseValue<types::PackagesResult>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/packages/{}", self.baseurl, encode_path(& attr_path
            .to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &page_size {
            query.push(("pageSize", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Resolve a list of Package Groups

Resolves a list of package groups, each being a list of package descriptors.

Required Body:
- **groups**: An object with an `items` array of PackageGroups to resolve.

Optional Query Parameters:
- **none**

Returns:
- **ResolvedPackageGroups**: A object with an `items` array of
    `ResolvedPackageGroup` items.

Resolution Rules:
- Each `PackageGroup` is resolved independently.
- Each page that has a package that meets each of the descriptors in that group is returned in the results
- The latest page will include details for each package in the group from that page
- The remainder pages are returned without details (to get those details... TBD)

A Package Descriptor match:
- **name**: [required] - is not used in matching, only for reference (TBD is
            there a uniqueness constraint?)
- **attr_path**: [required] - this must match the nix attribute path exactly and in full
- **version**: [optional] - Either a literal version to match or a **semver** constraint.
    This will be treated as a **semver** IFF TBD, otherwise it will be treated as
    a literal string match to the nix `version` field.  If this is detected as a **semver**,
    packages whose `version` field cannot be parsed as a **semver** will be excluded.
- **allow_pre_release**: [optional] - Defaults to False.  Only applies
    when a **semver** constraint is given.  If true, a `version` that can
    be parsed as a valid semver, that includes a pre-release suffix will
    be included as a candidate.  Otherwise, they will be excluded.
- **allow_broken**: [optional] - Defaults to False.  A package
    marked as broken = True will be excluded unless this is set to True.
- **allow_unfree**: [optional] - Defaults to True.  A package
    marked as unfree = True will be excluded unless this is set to True.
- **allow_insecure**: [optional] - Defaults to False.  A package
    marked as insecure = True will be excluded unless this is set to True.
- **allow_missing_builds**: [optional] - Defaults to
    False.  A package is expected to have been built if it
    is not marked as broken, unfree, or insecure.  A package
    that is expected to have been built, but none of it's outputs have been
    observed to build, will attempt to be excluded unless this is set to
    True.  This constraint may be softened if the group can not be resolved
    with it enforced.  If this occurs, the ressponse will note this by
    including a warning level message.

Sends a `POST` request to `/api/v1/catalog/resolve`

*/
    pub async fn resolve_api_v1_catalog_resolve_post<'a>(
        &'a self,
        body: &'a types::PackageGroups,
    ) -> Result<
        ResponseValue<types::ResolvedPackageGroups>,
        Error<types::ErrorResponse>,
    > {
        let url = format!("{}/api/v1/catalog/resolve", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Search for packages

Search the catalog(s) under the given criteria for matching packages.

Required Query Parameters:
- **search_term**: The search term to search on.
- **system**: This is returned but does not affect results

Optional Query Parameters:
- **catalogs**: Comma separated list of catalog names to search; defaults to all catalogs
- **page**: Optional page number for pagination (def = 0)
- **pageSize**: Optional page size for pagination (def = 10)

Returns:
- **PackageSearchResult**: A list of PackageInfo and the total result count

Sends a `GET` request to `/api/v1/catalog/search`

*/
    pub async fn search_api_v1_catalog_search_get<'a>(
        &'a self,
        catalogs: Option<&'a str>,
        page: Option<i64>,
        page_size: Option<i64>,
        search_term: Option<&'a types::SearchTerm>,
        system: types::SystemEnum,
    ) -> Result<ResponseValue<types::PackageSearchResult>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/search", self.baseurl,);
        let mut query = Vec::with_capacity(5usize);
        if let Some(v) = &catalogs {
            query.push(("catalogs", v.to_string()));
        }
        if let Some(v) = &page {
            query.push(("page", v.to_string()));
        }
        if let Some(v) = &page_size {
            query.push(("pageSize", v.to_string()));
        }
        if let Some(v) = &search_term {
            query.push(("search_term", v.to_string()));
        }
        query.push(("system", system.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Adjust various settings

Adjusts various settings on the catalog service.

Query Parameters:
- **key**: The the key to adjust.
    - "plan" - Enables the logging of the DB query plan for queries for
    **value** seconds.  It will be scheduled to turn off automatically after
    that.

Sends a `POST` request to `/api/v1/catalog/settings/{key}`

*/
    pub async fn settings_api_v1_catalog_settings_key_post<'a>(
        &'a self,
        key: &'a str,
        value: &'a str,
    ) -> Result<ResponseValue<serde_json::Value>, Error<types::ErrorResponse>> {
        let url = format!(
            "{}/api/v1/catalog/settings/{}", self.baseurl, encode_path(& key
            .to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        query.push(("value", value.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get basic catalog database status

Gather some basic status values from the database.

Returns:
- **CatalogStatus**: A dictionary of various status values.

Sends a `GET` request to `/api/v1/catalog/status/catalog`

*/
    pub async fn get_catalog_status_api_v1_catalog_status_catalog_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::CatalogStatus>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/status/catalog", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Perform basic catalog health check

Run some basic health checks on the catalog service.

Returns:
- **HealthCheck**: A dictionary of various health check values.

Sends a `GET` request to `/api/v1/catalog/status/healthcheck`

*/
    pub async fn get_catalog_health_check_api_v1_catalog_status_healthcheck_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::HealthCheck>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/status/healthcheck", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Trigger Error

Sends a `GET` request to `/api/v1/catalog/status/sentry-debug`

*/
    pub async fn trigger_error_api_v1_catalog_status_sentry_debug_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<serde_json::Value>, Error<()>> {
        let url = format!("{}/api/v1/catalog/status/sentry-debug", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get basic service status

Returns basic service status

Returns:
- **ServiceStatus**: A dictionary of various status values.

Sends a `GET` request to `/api/v1/catalog/status/service`

*/
    pub async fn get_service_status_api_v1_catalog_status_service_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ServiceStatus>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/status/service", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Get store info for a list of derivations

Get store info for a list of derivations

Body Parameters:
- **StoreInfoRequest**: A list of derivation paths

Returns:
- **StoreInfoResponse**: a map of derivation path to a list of store info objects

Sends a `POST` request to `/api/v1/catalog/store`

*/
    pub async fn get_store_info_api_v1_catalog_store_post<'a>(
        &'a self,
        body: &'a types::StoreInfoRequest,
    ) -> Result<ResponseValue<types::StoreInfoResponse>, Error<types::ErrorResponse>> {
        let url = format!("{}/api/v1/catalog/store", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            422u16 => {
                Err(Error::ErrorResponse(ResponseValue::from_response(response).await?))
            }
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
