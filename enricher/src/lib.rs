mod providers;

/// Defines common music metadata to be used to enrich music files
pub struct MusicMetadata {
    /// The title of the track
    pub title: String,

    /// The optional track artist
    pub artist: Option<String>,

    /// The optional track album
    pub album: Option<String>,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
