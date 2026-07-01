use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct VideoData {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Clone)]
pub struct VideoJson {
    pub id: String,
    pub name: String,
}

impl From<&VideoData> for VideoJson {
    fn from(data: &VideoData) -> Self {
        VideoJson {
            id: data.id.clone(),
            name: data.name.clone(),
        }
    }
}
