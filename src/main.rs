use axum::Json;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::{
    Router,
    extract::{Path, Query},
    http::{Response, StatusCode},
    routing::get,
};
use clap::{ArgGroup, Parser};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path as StdPath;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(group(
    ArgGroup::new("server")
        .args(&["serve", "port"])
        .multiple(true)
))]
struct Args {
    /// Flag to start the server
    #[arg(short, long)]
    serve: bool,
    /// Port number to use
    #[arg(short = 'p', long, default_value = "3000")]
    port: u16,
}

#[derive(Debug, Deserialize)]
struct HashQuery {
    action: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.serve {
        // Server program to serve files and their hashes
        let app = Router::new()
            .route("/{*file_path}", get(handle_file_request))
            .route("/", get(handle_hashes_request));

        let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
        println!("🚀 Server listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        // Client program to download file differences from the server
        if let Ok(local_hashes) = calculate_file_hashes_recursive() {
            let server_url = format!("http://127.0.0.1:{}/", args.port);

            // Fetch server hashes
            let client = reqwest::Client::new();
            let server_hashes: HashMap<String, String> = match client.get(&server_url).send().await
            {
                Ok(response) => match response.json().await {
                    Ok(hashes) => hashes,
                    Err(e) => {
                        eprintln!("[ERROR] Failed to parse server hashes. : {}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("[ERROR] Failed to connect to server. : {}", e);
                    return;
                }
            };

            // Calculate differences
            let mut files_to_download = Vec::new();
            for (file, server_hash) in &server_hashes {
                if local_hashes.get(file) != Some(server_hash) {
                    files_to_download.push(file.clone());
                }
            }

            // Download and save differences
            for file in files_to_download {
                println!("[GET] {}", file);
                let file_url = format!("{}{}", server_url, file);
                match client.get(&file_url).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let file_path = std::path::Path::new(&file);
                            if let Some(parent) = file_path.parent() {
                                std::fs::create_dir_all(parent).unwrap();
                            }
                            let mut file_handle = std::fs::File::create(file_path).unwrap();
                            let content = match response.bytes().await {
                                Ok(bytes) => bytes,
                                Err(e) => {
                                    eprintln!("[ERROR] Failed to read response bytes. : {}", e);
                                    return;
                                }
                            };
                            std::io::copy(&mut content.as_ref(), &mut file_handle).unwrap();
                        } else {
                            eprintln!("[ERROR] Failed to download file: {}", file);
                        }
                    }
                    Err(_) => {
                        eprintln!("[ERROR] Error connecting to download file.");
                    }
                }
            }
        } else {
            eprintln!("[ERROR] Error calculating local file hashes.");
        }
    }
}

async fn handle_file_request(
    Path(file_path): Path<PathBuf>,
    Query(query): Query<HashQuery>,
) -> Response<Body> {
    // If the query is ?action=hashes
    if let Some(action) = query.action {
        if action == "hashes" {
            return handle_hashes_request().await.into_response();
        }
    }

    println!("[GET] {}", file_path.display());
    let path = PathBuf::from(format!("./{}", file_path.display()));

    if path.exists() && path.is_file() {
        match std::fs::File::open(&path) {
            Ok(mut file) => {
                let mut contents = Vec::new();
                if file.read_to_end(&mut contents).is_ok() {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(contents))
                        .unwrap();
                } else {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("File read error"))
                        .unwrap();
                }
            }
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("File open error"))
                    .unwrap();
            }
        }
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("File not found"))
        .unwrap()
}

/// Handler for hash list retrieval requests
async fn handle_hashes_request() -> impl IntoResponse {
    println!("[GET] FILE HASHES");
    match calculate_file_hashes_recursive() {
        Ok(hashes) => Json(json!(hashes)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error calculating hashes".to_string(),
        )
            .into_response(),
    }
}

/// Function to calculate the hash values of all files under the current directory and return a HashMap<String, String>
fn calculate_file_hashes_recursive() -> io::Result<HashMap<String, String>> {
    let mut file_hashes = HashMap::new();

    let current_dir = std::env::current_dir()?;
    visit_dirs(&current_dir, &current_dir, &mut file_hashes)?;

    Ok(file_hashes)
}

/// Function to recursively traverse directories and calculate file hash values
fn visit_dirs(
    base_dir: &StdPath,
    dir: &StdPath,
    file_hashes: &mut HashMap<String, String>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_dirs(base_dir, &path, file_hashes)?;
        } else if path.is_file() {
            if let Some(relative_path) = path.strip_prefix(base_dir).ok().and_then(|p| p.to_str()) {
                let hash = calculate_hash(&path)?;
                file_hashes.insert(relative_path.to_string(), hash);
            }
        }
    }
    Ok(())
}

/// Function to calculate the hash value of a specified file
fn calculate_hash(path: &StdPath) -> io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}
