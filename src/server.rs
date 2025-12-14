use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
    Router,
    routing::{get, get_service},
};
use std::fs;
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::network::get_host_ip;

#[derive(Clone)]
struct AppState {
    root_dir: String,
}

#[tokio::main]
pub async fn serve(root_dir: &str) {
    let state = AppState {
        root_dir: root_dir.to_string(),
    };

    let app = Router::new()
        .route("/", get(homepage))
        .route("/shows/{*path}", get(directory_listing))
        .nest_service(
            "/files",
            get_service(ServeDir::new(root_dir)).handle_error(|error| async move {
                println!("Error serving directory: {}", error);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong",
                )
            }),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let ip = get_host_ip().unwrap_or_else(|| "127.0.0.1".parse().unwrap());
    println!("Serving on http://{}:8080", ip);
    println!("Homepage: http://{}:8080/", ip);

    axum::serve(listener, app).await.unwrap();
}

async fn homepage(State(state): State<AppState>) -> Response {
    let root_path = PathBuf::from(&state.root_dir);
    
    let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>oxicast - Podcast Server</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1 {
            color: #333;
            border-bottom: 3px solid #007bff;
            padding-bottom: 10px;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
        }
        .shows-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }
        .show-card {
            background: white;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .show-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 8px rgba(0,0,0,0.15);
        }
        .show-card h2 {
            margin-top: 0;
            color: #007bff;
            font-size: 1.3em;
        }
        .show-card a {
            color: #007bff;
            text-decoration: none;
        }
        .show-card a:hover {
            text-decoration: underline;
        }
        .links {
            margin-top: 15px;
            padding-top: 15px;
            border-top: 1px solid #eee;
        }
        .links a {
            display: inline-block;
            margin-right: 15px;
            font-size: 0.9em;
        }
        .footer {
            margin-top: 50px;
            padding-top: 20px;
            border-top: 1px solid #ddd;
            text-align: center;
            color: #666;
            font-size: 0.9em;
        }
        .no-shows {
            background: white;
            padding: 40px;
            text-align: center;
            border-radius: 8px;
            color: #666;
        }
    </style>
</head>
<body>
    <h1>🎙️ oxicast - Podcast Server</h1>
    <p class="subtitle">Turn folders of audio files into podcast shows</p>
    <div class="shows-grid">
"#);

    // Read all subdirectories (shows)
    match fs::read_dir(&root_path) {
        Ok(entries) => {
            let mut has_shows = false;
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        has_shows = true;
                        let show_name = entry.file_name().to_string_lossy().to_string();
                        let show_path = format!("/shows/{}", urlencoding::encode(&show_name));
                        let feed_path = format!("/files/{}/feed.xml", urlencoding::encode(&show_name));
                        
                        html.push_str(&format!(r#"
        <div class="show-card">
            <h2>{}</h2>
            <div class="links">
                <a href="{}">📁 Browse Files</a>
                <a href="{}">📡 RSS Feed</a>
            </div>
        </div>
"#, show_name, show_path, feed_path));
                    }
                }
            }
            
            if !has_shows {
                html.push_str(r#"
        <div class="no-shows">
            <p>No podcast shows found in the directory.</p>
            <p>Add folders with audio files to get started!</p>
        </div>
"#);
            }
        }
        Err(_) => {
            html.push_str(r#"
        <div class="no-shows">
            <p>Error reading directory.</p>
        </div>
"#);
        }
    }

    html.push_str(r#"
    </div>
    <div class="footer">
        <p>Powered by <a href="https://github.com/javonharper/oxicast" target="_blank">oxicast</a></p>
    </div>
</body>
</html>
"#);

    Html(html).into_response()
}

async fn directory_listing(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Response {
    let full_path = PathBuf::from(&state.root_dir).join(&path);
    
    if !full_path.starts_with(&state.root_dir) {
        return (
            axum::http::StatusCode::FORBIDDEN,
            Html("<h1>403 Forbidden</h1>".to_string()),
        )
            .into_response();
    }

    if !full_path.exists() {
        return (
            axum::http::StatusCode::NOT_FOUND,
            Html("<h1>404 Not Found</h1>".to_string()),
        )
            .into_response();
    }

    let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Browse - oxicast</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        h1 {
            color: #333;
            border-bottom: 3px solid #007bff;
            padding-bottom: 10px;
        }
        .breadcrumb {
            margin: 20px 0;
            color: #666;
        }
        .breadcrumb a {
            color: #007bff;
            text-decoration: none;
        }
        .breadcrumb a:hover {
            text-decoration: underline;
        }
        .file-list {
            background: white;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .file-item {
            padding: 12px;
            border-bottom: 1px solid #eee;
            display: flex;
            align-items: center;
            transition: background-color 0.2s;
        }
        .file-item:hover {
            background-color: #f8f9fa;
        }
        .file-item:last-child {
            border-bottom: none;
        }
        .file-item a {
            color: #333;
            text-decoration: none;
            flex-grow: 1;
        }
        .file-item a:hover {
            color: #007bff;
        }
        .icon {
            margin-right: 10px;
            font-size: 1.2em;
        }
        .back-link {
            display: inline-block;
            margin-bottom: 20px;
            color: #007bff;
            text-decoration: none;
        }
        .back-link:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
"#);

    html.push_str(&format!("<h1>📁 {}</h1>", path));
    html.push_str(r#"<a href="/" class="back-link">← Back to Home</a>"#);
    html.push_str(r#"<div class="file-list">"#);

    match fs::read_dir(&full_path) {
        Ok(entries) => {
            let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            items.sort_by_key(|e| e.file_name());

            for entry in items {
                if let (Ok(metadata), Ok(file_name)) = (entry.metadata(), entry.file_name().into_string()) {
                    let is_dir = metadata.is_dir();
                    let icon = if is_dir { "📁" } else { "📄" };
                    let file_path = format!("/files/{}/{}", urlencoding::encode(&path), urlencoding::encode(&file_name));
                    let link = if is_dir {
                        format!("/shows/{}/{}", urlencoding::encode(&path), urlencoding::encode(&file_name))
                    } else {
                        file_path
                    };

                    html.push_str(&format!(
                        r#"<div class="file-item"><span class="icon">{}</span><a href="{}">{}</a></div>"#,
                        icon, link, file_name
                    ));
                }
            }
        }
        Err(_) => {
            html.push_str("<p>Error reading directory.</p>");
        }
    }

    html.push_str(r#"
    </div>
</body>
</html>
"#);

    Html(html).into_response()
}
