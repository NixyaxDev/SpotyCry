#[derive(Clone, Debug)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<u64>,
    pub file_path: String,
    pub is_active: bool,
}
