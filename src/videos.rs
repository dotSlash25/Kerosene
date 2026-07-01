use std::hash::{DefaultHasher, Hash, Hasher};

use crate::AppState;
use crate::structs::{VideoData, VideoJson};
use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, Result, error::ErrorNotFound, get, web};
use walkdir::WalkDir;

fn hash(t: String) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    format!("{:016x}", s.finish())
}

#[get("/videos")]
async fn get_videos(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(
        data.videos
            .iter()
            .map(|v| VideoJson::from(v))
            .collect::<Vec<VideoJson>>(),
    )
}

#[get("/streams/{id}")]
async fn get_video(path: web::Path<String>, data: web::Data<AppState>) -> Result<NamedFile> {
    let id = path.into_inner();

    let video = data
        .videos
        .iter()
        .find(|v| v.id == id)
        .ok_or_else(|| ErrorNotFound("Video not found"))?;

    Ok(NamedFile::open(&video.path)?)
}

pub fn scan_videos() -> Vec<VideoData> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file()
                && matches!(
                    e.path().extension().and_then(|s| s.to_str()),
                    Some("mp4" | "mkv")
                )
        })
        .map(|e| VideoData {
            id: hash(e.path().display().to_string()),
            name: e.file_name().to_string_lossy().into_owned(),
            path: e.path().display().to_string(),
        })
        .collect()
}
