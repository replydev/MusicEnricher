/// Defines common music metadata to be used to enrich music files
pub struct MusicMetadata {
    /// The title of the track
    pub title: String,

    /// The optional track artist
    pub artist: Option<String>,

    /// The optional track album
    pub album: Option<String>,
}
