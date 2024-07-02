use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SongQuery {
    pub search: Option<String>,
}
