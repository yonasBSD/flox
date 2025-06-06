use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use flox_core::activations;

type Error = anyhow::Error;

#[derive(Debug, Args)]
pub struct SetReadyArgs {
    #[arg(help = "The path to the activation symlink for the environment.")]
    #[arg(short, long, value_name = "PATH")]
    pub flox_env: PathBuf,
    #[arg(help = "The ID for this particular activation of this environment.")]
    #[arg(short, long, value_name = "ID")]
    pub id: String,
    /// The path to the runtime directory keeping activation data.
    #[arg(long, value_name = "PATH")]
    pub runtime_dir: PathBuf,
}

impl SetReadyArgs {
    pub fn handle(self) -> Result<(), Error> {
        let activations_json_path =
            activations::activations_json_path(&self.runtime_dir, &self.flox_env);

        let (activations, lock) = activations::read_activations_json(&activations_json_path)?;
        let Some(activations) = activations else {
            anyhow::bail!("Expected an existing activations.json file");
        };

        let Ok(mut activations) = activations.check_version() else {
            anyhow::bail!("Invalid version in activations.json file");
        };

        let activation = activations
            .activation_for_id_mut(&self.id)
            .with_context(|| {
                format!(
                    "No activation with ID {} found for environment {}",
                    self.id,
                    self.flox_env.display()
                )
            })?;

        activation.set_ready();

        activations::write_activations_json(&activations, &activations_json_path, lock)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::cli::test::{read_activations, write_activations};

    #[test]
    fn set_ready() {
        let runtime_dir = TempDir::new().unwrap();
        let flox_env = PathBuf::from("/path/to/floxenv");
        let pid = 5678;

        let id = write_activations(&runtime_dir, &flox_env, |activations| {
            activations
                .create_activation("/store/path", pid)
                .unwrap()
                .id()
        });

        let ready = read_activations(&runtime_dir, &flox_env, |activations| {
            activations.activation_for_id_ref(&id).unwrap().ready()
        })
        .unwrap();

        assert!(!ready);

        let args = SetReadyArgs {
            flox_env: flox_env.clone(),
            id: id.clone(),
            runtime_dir: runtime_dir.path().to_path_buf(),
        };

        args.handle().unwrap();

        let ready = read_activations(&runtime_dir, &flox_env, |activations| {
            activations.activation_for_id_ref(id).unwrap().ready()
        })
        .unwrap();

        assert!(ready);
    }
}
