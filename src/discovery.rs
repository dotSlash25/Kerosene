use actix_web::{HttpResponse, Responder, get, web::Data};
use mdns_sd::{Error, ServiceDaemon, ServiceInfo};
use serde::Serialize;

use crate::{AppState, PORT, get_ips};

#[derive(Serialize)]
pub struct ServerInfo {
    name: String,
    num_videos: usize,
}

#[get("/info")]
async fn get_info(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(ServerInfo {
        name: data.name.clone(),
        num_videos: data.videos.len(),
    })
}

pub fn broadcast_service() -> Result<ServiceDaemon, Error> {
    let mdns = ServiceDaemon::new()?;
    let service_type = "_http._tcp.local.";
    let instance_name = "Kerosene";
    let hostname = "kerosene.local.";
    let ip = get_ips()
        .ok_or(Error::Msg("G".to_string()))?
        .into_iter()
        .filter(|ip| ip.starts_with("192"))
        .next()
        .unwrap();
    let properties = [("app", "Lantern"), ("version", "1")];
    let service = ServiceInfo::new(
        service_type,
        instance_name,
        hostname,
        ip,
        PORT,
        &properties[..],
    )?;
    mdns.register(service)?;
    Ok(mdns)
}
