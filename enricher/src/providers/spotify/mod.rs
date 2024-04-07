use std::{fmt::Display, path::Path};

use anyhow::{anyhow, Result};

use super::MetadataProvider;

const SPOTIFY_SEARCH_URL: &'static str = "https://api.spotify.com/v1/search";

/// Singleton Spotify Client
struct SpotifyClient {
    http_client: reqwest::Client,
}

impl MetadataProvider for SpotifyClient {
    async fn provide<'a>(&self, path: &'a Path) -> Result<crate::MusicMetadata> {
        if let Some(filename) = path.file_name().and_then(|o| o.to_str()) {
            let request = SpotifySearchRequest {
                search_query: filename,
                ..Default::default()
            };
            self.http_client
                .get(request.to_string())
                .send()
                .await
                .map_err(|request_error| {
                    anyhow!(
                        "An error occurred during HTTP request to the Spotify API: {:?}",
                        op
                    )
                })?
                .json::<J()
                .await
                .map_err(|op| anyhow!("An error occurred during JSON parsing: {:?}", op))
        } else {
            return Err(MetadataProviderError::InvalidPath(&path));
        }
    }
}

impl Display for SpotifySearchRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}?q={}&type={}&limit={}",
            SPOTIFY_SEARCH_URL, self.search_query, self.search_type, self.limit
        )
    }
}

impl Default for SpotifySearchRequest<'_> {
    fn default() -> Self {
        Self {
            search_query: Default::default(),
            search_type: SpotifySearchType::TRACK,
            limit: 10,
        }
    }
}

/// Represents the GET parameters for the Spotify /search endpoint
struct SpotifySearchRequest<'a> {
    search_query: &'a str,
    search_type: SpotifySearchType,
    limit: u16,
}

/// Types of Spotify search queries
#[derive(Debug)]
enum SpotifySearchType {
    TRACK,
}

impl Display for SpotifySearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
