use actix_multipart::Multipart;
use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use mysql::*;
use mysql::prelude::*;

#[derive(serde::Serialize)]
struct PersonInfo {
    name: String,
    info: String,
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    let mut filepath = String::new();
    while let Some(Ok(field)) = payload.next().await {
        let filename = field.content_disposition().get_filename().unwrap_or("upload.png");
        let filepath_tmp = format!("./tmp/{}", filename);
        filepath = filepath_tmp.clone();

        let mut f = File::create(filepath_tmp).unwrap();
        let mut field = field;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }
    }

    // Run the Python recognizer script
    let output = Command::new("python3")
        .arg("recognizer.py")
        .arg(&filepath)
        .output()
        .expect("Failed to run Python script");

    let recognized_name = String::from_utf8_lossy(&output.stdout).to_string();

    // MySQL connection details
    let url = "mysql://root:password@localhost:3306/db_name"; // update with your actual MySQL connection string
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Query for person info
    let result: Option<(String, String)> = conn.exec_first(
        "SELECT name, info FROM persons WHERE name = :name",
        params! {
            "name" => &recognized_name,
        },
    ).unwrap();

    let person = match result {
        Some((name, info)) => PersonInfo { name, info },
        None => PersonInfo {
            name: "Unknown".to_string(),
            info: "No info available".to_string(),
        },
    };

    HttpResponse::Ok().json(person)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(upload))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
