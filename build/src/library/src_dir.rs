use std::path::PathBuf;

use anyhow::Result;
use tracing::info;

use super::lib_rs::LibRs;

#[derive(Debug)]
pub(crate) struct SrcDir {
    pub path: PathBuf,
    pub lib_rs: LibRs,
}

impl SrcDir {
    /// Removes everything inside and creates a fresh lib.rs file.
    pub async fn init(&mut self) -> Result<()> {
        info!(path = ?self.path, "Creating new src directory");
        tokio::fs::create_dir(&self.path).await?;

        self.lib_rs.init().await?;
        Ok(())
    }
}
