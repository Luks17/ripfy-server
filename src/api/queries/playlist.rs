use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlaylistQuery {
    pub search: Option<String>,
}
