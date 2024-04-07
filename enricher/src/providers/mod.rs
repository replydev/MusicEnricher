mod spotify;
use std::path::Path;

use anyhow::Result;

use crate::MusicMetadata;

/// Implementations of this trait should be responsible of retrieving
/// metadata from a particular data source.
pub trait MetadataProvider {
    /// Provides metadata related to the passed file
    async fn provide(&self, path: &Path) -> Result<MusicMetadata>;
}
