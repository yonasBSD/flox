use std::error;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use flox_core::canonical_path::CanonicalPath;
use thiserror::Error;

use super::build::ManifestBuilder;
use super::git::GitCommandProvider;
use crate::flox::{Flox, FloxhubToken};
use crate::models::environment::managed_environment::ManagedEnvironment;
use crate::models::environment::path_environment::PathEnvironment;
use crate::models::environment::Environment;
// use crate::models::lockfile;
use crate::models::lockfile::Lockfile;
// use crate::providers::build;
// use crate::models::lockfile::Lockfile;
use crate::providers::git::GitProvider;

pub enum PublishEnvironment {
    Path(PathEnvironment),
    Managed(ManagedEnvironment),
}

#[derive(Debug, Error)]
pub enum PublishError {
    #[error("This type of environment is not supported for publishing")]
    UnsupportedEnvironment,
    #[error("The environment must be locked to publish")]
    UnlockedEnvironment,

    #[error("The environment is in an unsupported state for publishing")]
    UnsupportEnvironmentState(#[source] Box<dyn error::Error>),

    #[error("Could not identify user from authentication info")]
    Unauthenticated,
}

/// The `Publish` trait describes the high level behavior of publishing a package to a catalog.
/// Authentication, upload, builds etc, are implementation details of the specific provider.
/// Modeling the behavior as a trait allows us to swap out the provider, e.g. a mock for testing.
pub trait Publish {
    fn publish(
        &self,
        flox: &Flox,
        environment: PublishEnvironment,
        builder: impl ManifestBuilder,
        package: &str,
    ) -> Result<(), PublishError>;
}

/// Simple struct to hold the information of a locked URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedUrlInfo {
    pub url: String,
    pub rev: String,
    pub rev_count: u64,
    pub rev_date: Option<DateTime<Utc>>,
}

/// Ensures that the required metadata for publishing is consistent from the environment
#[allow(clippy::manual_non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedEnvironmentMetadata {
    // There may or may not be a locked base catalog reference in the environment
    pub base_catalog_ref: Option<LockedUrlInfo>,
    // The build repo reference is always present
    pub build_repo_ref: LockedUrlInfo,

    // This field isn't "pub", so no one outside this module can construct this struct. That helps
    // ensure that we can only make this struct as a result of doing the "right thing."
    _private: (),
}

/// Ensures that the required metadata for publishing is consistent from the build process
#[allow(clippy::manual_non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedBuildMetadata {
    // Define metadata coming from the build, e.g. outpaths
    pub outputs: Option<Vec<String>>,

    // This field isn't "pub", so no one outside this module can construct this struct. That helps
    // ensure that we can only make this struct as a result of doing the "right thing."
    _private: (),
}

/// The `PublishProvider` is a concrete implementation of the `Publish` trait.
/// It is responsible for the actual implementation of the `Publish` trait,
/// i.e. the actual publishing of a package to a catalog.
///
/// The `PublishProvider` is a generic struct, parameterized by a `Builder` type,
/// to build packages before publishing.
pub struct PublishProvider<Builder> {
    /// Directory under which we will clone and build the
    _base_temp_dir: PathBuf,
    /// Token of the user to authenticate with the catalog
    _auth_token: Option<FloxhubToken>,
    /// Building of manifest packages
    _builder: Builder,
}

/// (default) implementation of the `Publish` trait, i.e. the publish interface to publish.
impl<Builder> Publish for PublishProvider<&Builder>
where
    Builder: ManifestBuilder,
{
    fn publish(
        &self,
        flox: &Flox,
        environment: PublishEnvironment,
        builder: impl ManifestBuilder,
        package: &str,
    ) -> Result<(), PublishError> {
        // Get metadata from the environment, like locked URLs.
        let _env_meta = match environment {
            PublishEnvironment::Managed(_env) => return Err(PublishError::UnsupportedEnvironment),
            PublishEnvironment::Path(env) => check_environment_metadata(flox, &env),
        };

        // Need to grab outputs from the build
        let _build_meta = check_builder_metadata(flox, builder, package);

        let _package_name = package;
        let _catalog_name = match &flox.floxhub_token {
            Some(token) => token.handle().to_string(),
            None => return Err(PublishError::Unauthenticated),
        };
        let _version: Option<String> = None;
        let _description: Option<String> = None;

        // Uses client to...
        // ... check access to the catalog
        // ... check presence of and create the package if needed
        // ... publish the build info
        // publish_to_catalog()
        Ok(())
    }
}

fn check_builder_metadata(
    _flox: &Flox,
    _builder: impl ManifestBuilder,
    _pkg: &str,
) -> Result<CheckedBuildMetadata, String> {
    // Build (if needed) and collect the meta data needed to publish
    // builder.build(tmp_dir, env.path(), pkg).unwrap();

    // Access a `flox build` builder
    // ... to clone into a sandbox and run
    // ... `flox activate; flox build;`
    // prepublish_build();
    // _sandbox = temp dir
    // Use GitCommandProvider to get remote and current rev of build_repo
    // Use GitCommandProvider to clone that remote/rev to _sandbox
    //   this will ensure it's in the remote
    // Load the manifest from the .flox of that repo to get the version/description/catalog
    // Confirm access to remote resources, login, etc, so as to not waste
    //   time if it's going to fail later or require user interaction to
    // continue the operation.

    // Obtains info from the build process (the sandboxed one above)
    // ... base_catalog_{locked_url, rev, rev_count, rev_date}
    // ... system, drv_path (??), outputs (??)
    // gather_build_info(_sandbox: &Path) {
    // Access lockfile of _sandbox to get base_catalog_{locked_url, rev, rev_count, rev_date}
    // populate the following:
    //      how do we get the drv_path, outputs(and paths), system from this?
    todo!()
}

fn gather_build_repo_meta(environment: &PathEnvironment) -> Result<LockedUrlInfo, PublishError> {
    // Gather build repo info
    let git = match environment.parent_path() {
        Ok(env_path) => GitCommandProvider::discover(env_path)
            .map_err(|e| PublishError::UnsupportEnvironmentState(Box::new(e)))?,
        Err(e) => return Err(PublishError::UnsupportEnvironmentState(Box::new(e))),
    };

    let origin = git
        .get_origin()
        .map_err(|e| PublishError::UnsupportEnvironmentState(Box::new(e)))?;

    let rev = origin
        .revision
        .ok_or(PublishError::UnsupportEnvironmentState(
            "No revision found".to_string().into(),
        ))?;

    let rev_count = git
        .rev_count(rev.as_str())
        .map_err(|e| PublishError::UnsupportEnvironmentState(Box::new(e)))?;

    Ok(LockedUrlInfo {
        url: origin.url,
        rev,
        rev_count,
        rev_date: None,
    })
}

fn gather_base_repo_meta(
    flox: &Flox,
    environment: &PathEnvironment,
) -> Result<Option<LockedUrlInfo>, PublishError> {
    // Gather locked base catalog page info
    let lockfile_path = CanonicalPath::new(
        environment
            .lockfile_path(flox)
            .map_err(|e| PublishError::UnsupportEnvironmentState(Box::new(e)))?,
    )
    .map_err(|e| PublishError::UnsupportEnvironmentState(Box::new(e)))?;

    let lockfile = Lockfile::read_from_file(&lockfile_path)
        .map_err(|e| PublishError::UnsupportEnvironmentState(Box::new(e)))?;

    let install_ids_in_toplevel_group = lockfile
        .manifest
        .pkg_descriptors_in_toplevel_group()
        .into_iter()
        .map(|(pkg, _desc)| pkg);

    // Require a lockfile, but don't require anything in the top level group.
    if install_ids_in_toplevel_group.clone().count() == 0 {
        return Ok(None);
    }

    let top_level_locked_descs = lockfile.packages.iter().filter(|pkg| {
        install_ids_in_toplevel_group
            .clone()
            .any(|id| id == pkg.install_id())
    });
    if let Some(pkg) = top_level_locked_descs.clone().next() {
        Ok(Some(LockedUrlInfo {
            url: pkg.as_catalog_package_ref().unwrap().locked_url.clone(),
            rev: pkg.as_catalog_package_ref().unwrap().rev.clone(),
            rev_count: pkg
                .as_catalog_package_ref()
                .unwrap()
                .rev_count
                .try_into()
                .unwrap(),
            rev_date: Some(pkg.as_catalog_package_ref().unwrap().rev_date),
        }))
    } else {
        Err(PublishError::UnsupportEnvironmentState(
            "Unable to find locked descriptor for toplevel package"
                .to_string()
                .into(),
        ))
    }
}

fn check_environment_metadata(
    flox: &Flox,
    environment: &PathEnvironment,
) -> Result<CheckedEnvironmentMetadata, PublishError> {
    // TODO - Ensure current commit is in remote (needed for repeatable builds)
    let build_repo_meta = gather_build_repo_meta(environment)?;

    let base_repo_meta = gather_base_repo_meta(flox, environment)?;

    Ok(CheckedEnvironmentMetadata {
        base_catalog_ref: base_repo_meta,
        build_repo_ref: build_repo_meta,
        _private: (),
    })
}

#[cfg(test)]
pub mod tests {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::flox::test_helpers::flox_instance;
    use crate::models::environment::path_environment::test_helpers::new_path_environment_from_env_files;
    // use crate::providers::build::FloxBuildMk;
    use crate::providers::catalog::GENERATED_DATA;

    fn example_remote() -> (tempfile::TempDir, GitCommandProvider, String) {
        let tempdir_handle = tempfile::tempdir_in(std::env::temp_dir()).unwrap();

        let repo = GitCommandProvider::init(tempdir_handle.path(), true).unwrap();

        let remote_uri = format!("file://{}", tempdir_handle.path().display());

        (tempdir_handle, repo, remote_uri)
    }

    // Commented to avoid warnings for now
    // fn example_builder() -> impl ManifestBuilder {
    //     let builder = FloxBuildMk;
    //     // let output_stream = builder
    //     //     .build(
    //     //         &env.parent_path().unwrap(),
    //     //         &env.activation_path(flox).unwrap(),
    //     //         &[package_name.to_owned()],
    //     //     )
    //     //     .unwrap();
    //     return builder;
    // }

    // const EXAMPLE_PACKAGE_NAME: &str = "mypkg";

    fn example_path_environment(
        flox: &Flox,
        remote: Option<&String>,
    ) -> (PathEnvironment, GitCommandProvider) {
        let env =
            new_path_environment_from_env_files(flox, GENERATED_DATA.join("envs/publish-simple"));

        let git = GitCommandProvider::init(
            env.parent_path().expect("Parent path must be accessible"),
            false,
        )
        .unwrap();

        git.checkout("main", true).expect("checkout main branch");
        git.add(&[&env.dot_flox_path()]).expect("adding flox files");
        git.commit("Initial commit").expect("be able to commit");

        if let Some(uri) = remote {
            git.add_remote("origin", uri.as_str()).unwrap();
            git.push("origin", true).expect("push to origin");
        }

        (env, git)
    }

    #[test]
    fn test_check_env_meta_failure() {
        let (flox, _temp_dir_handle) = flox_instance();
        let (env, _git) = example_path_environment(&flox, None);

        let meta = check_environment_metadata(&flox, &env);
        assert_eq!(meta.is_err(), true);
    }

    #[test]
    fn test_check_env_meta_nominal() {
        let (flox, _temp_dir_handle) = flox_instance();
        let (_tempdir_handle, _remote_repo, remote_uri) = example_remote();
        let (env, build_repo) = example_path_environment(&flox, Some(&remote_uri));

        let meta = check_environment_metadata(&flox, &env).unwrap();

        let build_repo_meta = meta.build_repo_ref;
        assert!(build_repo_meta.url.contains(&remote_uri));
        assert!(build_repo
            .contains_commit(build_repo_meta.rev.as_str())
            .is_ok());
        assert_eq!(build_repo_meta.rev_count, 1);

        assert!(meta.base_catalog_ref.is_some());
        let base_repo_meta = meta.base_catalog_ref.unwrap();

        let lockfile_path = CanonicalPath::new(env.lockfile_path(&flox).unwrap());
        let lockfile = Lockfile::read_from_file(&lockfile_path.unwrap()).unwrap();
        // Only the toplevel group in this example, so we can grap the first package
        let locked_base_pkg = lockfile.packages[0].as_catalog_package_ref().unwrap();
        assert_eq!(base_repo_meta.url, locked_base_pkg.locked_url);
        assert_eq!(base_repo_meta.rev, locked_base_pkg.rev);
        assert_eq!(
            base_repo_meta.rev_count,
            TryInto::<u64>::try_into(locked_base_pkg.rev_count).unwrap()
        );
        assert_eq!(base_repo_meta.rev_date.unwrap(), locked_base_pkg.rev_date);
    }

    // Template end to end test
    //     #[test]
    //     fn test_create_provider() {
    //         let (flox, _temp_dir_handle) = flox_instance();

    //         let publish_provider = PublishProvider {
    //             _base_temp_dir: PathBuf::from("/tmp"),
    //             _auth_token: None,
    //             _builder: &example_builder(),
    //         };
    //
    //          todo - test floxhub_token -> catalog name logic

    //         let (_tempdir_handle, _remote_repo, remote_uri) = example_remote();
    //         let (env, _git) = example_path_environment(&flox, Some(&remote_uri));
    //         let _res = publish_provider.publish(
    //             &flox,
    //             Client::Mock(Default::default()),
    //             PublishEnvironment::Path(env),
    //             example_builder(),
    //             EXAMPLE_PACKAGE_NAME,
    //         );

    //     }
}
