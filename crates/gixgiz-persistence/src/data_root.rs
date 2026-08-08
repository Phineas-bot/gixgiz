use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::PersistenceError;

const PRODUCT_DIRECTORY: &str = "GixGiz";

/// Canonical per-user storage layout owned by the Rust platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataRoot {
    root: PathBuf,
    data: PathBuf,
    config: PathBuf,
    logs: PathBuf,
    cache: PathBuf,
    staging: PathBuf,
    content: PathBuf,
    backups: PathBuf,
}

impl DataRoot {
    /// Resolves and initializes `%LOCALAPPDATA%\GixGiz`.
    pub fn resolve_default() -> Result<Self, PersistenceError> {
        let local_app_data = env::var_os("LOCALAPPDATA")
            .filter(|value| !value.is_empty())
            .ok_or(PersistenceError::DataRootUnavailable)?;
        Self::initialize(PathBuf::from(local_app_data).join(PRODUCT_DIRECTORY))
    }

    /// Initializes an explicit root intended for tests and controlled hosts.
    pub fn from_override(root: impl AsRef<Path>) -> Result<Self, PersistenceError> {
        Self::initialize(root.as_ref().to_path_buf())
    }

    fn initialize(root: PathBuf) -> Result<Self, PersistenceError> {
        fs::create_dir_all(&root).map_err(PersistenceError::path_io)?;
        let root = fs::canonicalize(root).map_err(PersistenceError::path_io)?;

        let data = create_child(&root, "data")?;
        let config = create_child(&root, "config")?;
        let logs = create_child(&root, "logs")?;
        let cache = create_child(&root, "cache")?;
        let staging = create_child(&root, "staging")?;
        let content = create_child(&root, "content")?;
        let backups = create_child(&root, "backups")?;

        Ok(Self {
            root,
            data,
            config,
            logs,
            cache,
            staging,
            content,
            backups,
        })
    }

    /// Returns the canonical GixGiz data root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the directory containing the SQLite database.
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        &self.data
    }

    /// Returns the configuration directory.
    #[must_use]
    pub fn config_dir(&self) -> &Path {
        &self.config
    }

    /// Returns the sanitized structured-log directory.
    #[must_use]
    pub fn logs_dir(&self) -> &Path {
        &self.logs
    }

    /// Returns the disposable cache directory.
    #[must_use]
    pub fn cache_dir(&self) -> &Path {
        &self.cache
    }

    /// Returns the staging directory for future durable operations.
    #[must_use]
    pub fn staging_dir(&self) -> &Path {
        &self.staging
    }

    /// Returns the content directory for future verified large artifacts.
    #[must_use]
    pub fn content_dir(&self) -> &Path {
        &self.content
    }

    /// Returns the database backup directory.
    #[must_use]
    pub fn backups_dir(&self) -> &Path {
        &self.backups
    }

    /// Returns the SQLite database path beneath the canonical data directory.
    #[must_use]
    pub fn database_path(&self) -> PathBuf {
        self.data.join("gixgiz.db")
    }

    pub(crate) fn owner_lock_path(&self) -> PathBuf {
        self.data.join("gixgiz.owner.lock")
    }
}

fn create_child(root: &Path, name: &str) -> Result<PathBuf, PersistenceError> {
    let requested = root.join(name);
    fs::create_dir_all(&requested).map_err(PersistenceError::path_io)?;
    let canonical = fs::canonicalize(requested).map_err(PersistenceError::path_io)?;
    if !canonical.starts_with(root) {
        return Err(PersistenceError::PathEscapesRoot);
    }
    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_creates_only_the_expected_canonical_layout() {
        let temporary = tempfile::tempdir().expect("temporary directory is available");
        let root = DataRoot::from_override(temporary.path().join("root"))
            .expect("test data root initializes");

        assert!(root.database_path().starts_with(root.root()));
        for directory in [
            root.data_dir(),
            root.config_dir(),
            root.logs_dir(),
            root.cache_dir(),
            root.staging_dir(),
            root.content_dir(),
            root.backups_dir(),
        ] {
            assert!(directory.is_dir());
            assert!(directory.starts_with(root.root()));
        }
    }
}
