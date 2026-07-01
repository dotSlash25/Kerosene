use std::fs;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

mod discovery;
mod state;
mod structs;
mod videos;
use discovery::*;
use state::*;
use videos::*;

const PORT: u16 = 8000;
const SAVE_PATH: &'static str = "kerosene_data.txt";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("/videos to access all videos\n/videos/{id} to access any video")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("<html><body>This is manual <i>Hello</i></body></html>")
}

fn get_ips() -> Option<Vec<String>> {
    let ips: Vec<String> = ipconfig::get_adapters()
        .ok()?
        .iter()
        .map(|ip| ip.ip_addresses())
        .flatten()
        .filter(|i| i.is_ipv4())
        .map(|i| i.to_string())
        .collect();
    Some(ips)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_name = fs::read_to_string(SAVE_PATH).unwrap_or(register_server());

    let videos = scan_videos();
    let state = web::Data::new(AppState {
        name: server_name,
        videos: videos,
    });
    let ips = get_ips().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NetworkUnreachable,
            "No network interfaces found",
        )
    })?;
    let _mdns = broadcast_service().unwrap();
    println!("Kerosene server initiated");
    println!("Server running on ips:");
    for ip in ips.iter().filter(|ip| ip.starts_with("192")) {
        println!("{ip}:{PORT}");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(hello)
            .service(get_info)
            .service(get_videos)
            .service(get_video)
            .route("/helu", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

fn register_server() -> String {
    println!("Looks like it is your first run");
    println!("Name your server:");
    let mut buf = String::new();
    let n = std::io::stdin().read_line(&mut buf).unwrap_or_default();
    if n == 0 {
        buf = "Server".to_string();
    }
    fs::write(SAVE_PATH, &buf)
        .expect("Failed to create savefile, you might have to register again");
    buf.clone()
}
