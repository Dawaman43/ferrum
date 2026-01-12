use anyhow::{anyhow, Result};
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, Router},
    Json,
};
use ferrum_core::formatter::FerrumFormatter;
use ferrum_core::parser::FerrumParser;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};

/// Pure Rust Development Server
/// NO JavaScript, NO Single HTML - Everything handled by Rust
pub struct RustDevServer {
    port: u16,
    project_path: String,
    compiled_components: Arc<RwLock<HashMap<String, String>>>,
    server_state: Arc<RwLock<ServerState>>,
}

#[derive(Clone)]
struct ServerState {
    last_reload: SystemTime,
    active_routes: Vec<String>,
    compiled_files: HashMap<String, String>,
}

impl RustDevServer {
    pub fn new(project_path: String, port: u16) -> Result<Self> {
        let compiled_components = Arc::new(RwLock::new(HashMap::new()));

        let server_state = Arc::new(RwLock::new(ServerState {
            last_reload: SystemTime::now(),
            active_routes: Vec::new(),
            compiled_files: HashMap::new(),
        }));

        Ok(Self {
            port,
            project_path,
            compiled_components,
            server_state,
        })
    }

    /// Start pure Rust development server
    pub async fn run(&self) -> Result<()> {
        println!("🦀 Starting Pure Rust Ferrum Server");
        println!("📁 Project: {}", self.project_path);
        println!("🌐 Port: {}", self.port);
        println!("🔥 Pure Rust: NO JavaScript, NO Single HTML");
        println!("💾 Manual save only (no auto-format)");
        println!("👀 Watching .frr files for compilation...");

        // Setup file watcher for .frr files
        self.setup_frr_watcher().await?;

        // Start pure Rust web server
        self.start_rust_server().await
    }

    /// Watch .frr files and recompile on changes
    async fn setup_frr_watcher(&self) -> Result<()> {
        let project_path = self.project_path.clone();
        let compiled_components = self.compiled_components.clone();
        let server_state = self.server_state.clone();

        tokio::spawn(async move {
            let (tx, mut rx) =
                tokio::sync::mpsc::unbounded_channel::<notify::Result<notify::Event>>();
            let mut watcher = match RecommendedWatcher::new(
                move |res| {
                    let _ = tx.send(res);
                },
                Config::default(),
            ) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("❌ Failed to create watcher: {}", e);
                    return;
                }
            };

            let src_path = Path::new(&project_path).join("src");
            if !src_path.exists() {
                eprintln!("❌ src/ directory not found");
                return;
            }

            println!("👀 Watching: {:?}", src_path);

            if let Err(e) = watcher.watch(&src_path, RecursiveMode::Recursive) {
                eprintln!("❌ Failed to watch directory: {}", e);
                return;
            }

            // Debounce: track file changes with timestamps
            let debounce_duration = std::time::Duration::from_millis(300);
            let mut pending_changes: std::collections::HashMap<String, tokio::time::Instant> =
                std::collections::HashMap::new();

            // Debounce task: process pending changes after debounce period
            let mut debounce_interval = tokio::time::interval(debounce_duration);
            debounce_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

            loop {
                tokio::select! {
                    // Handle file watcher events
                    res = rx.recv() => {
                        let Some(res) = res else { break };

                        match res {
                            Ok(event) => {
                                if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                                    continue;
                                }

                                for path in event.paths {
                                    if path.extension().and_then(|ext| ext.to_str()) != Some("frr") {
                                        continue;
                                    }

                                    // Update pending change timestamp
                                    let path_str = path.to_string_lossy().to_string();
                                    pending_changes.insert(path_str, tokio::time::Instant::now());
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Watch error: {:?}", e);
                            }
                        }
                    }

                    // Process pending changes after debounce
                    _ = debounce_interval.tick() => {
                        let now = tokio::time::Instant::now();

                        // Process files that haven't changed in debounce_duration
                        let mut files_to_process = Vec::new();
                        pending_changes.retain(|path, timestamp| {
                            if now.duration_since(*timestamp) >= debounce_duration {
                                files_to_process.push(path.clone());
                                false // Remove from pending
                            } else {
                                true // Keep pending
                            }
                        });

                        // Process each file once
                        for path_str in files_to_process {
                            let path = Path::new(&path_str);
                            println!("🔄 Changed: {:?}", path.file_name());

                            // Compile file only (no auto-format)
                            match compile_frr_file(&path) {
                                Ok(compiled) => {
                                    // Update compiled components
                                    let mut components = compiled_components.write().await;
                                    let path_str = path.to_string_lossy().to_string();
                                    components.insert(path_str.clone(), compiled.clone());

                                    // Update server state
                                    let mut state = server_state.write().await;
                                    state.last_reload = SystemTime::now();
                                    state
                                        .compiled_files
                                        .insert(path_str.clone(), compiled.clone());

                                    println!("✅ Compiled: {:?}", path.file_name());
                                }
                                Err(e) => {
                                    eprintln!("❌ Compilation failed: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Start pure Rust web server with HTML generation
    async fn start_rust_server(&self) -> Result<()> {
        let app_state = self.server_state.clone();

        // Create Axum router with pure Rust handlers
        let app = Router::new()
            // Main page - generated from .frr
            .route("/", get(generate_main_page))
            // Component endpoints
            .route("/components/:component", get(generate_component_page))
            // API endpoints for monitoring
            .route("/api/status", get(api_status))
            .route("/api/components", get(api_components))
            .route("/api/save", get(api_save))
            .route("/api/render", get(api_render))
            .route("/api/format", get(api_format))
            // Static assets
            .nest_service("/static", ServeDir::new("static"))
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(app_state);

        let addr = ("127.0.0.1", self.port);

        println!(
            "🌐 Pure Rust server ready at: http://localhost:{}",
            self.port
        );

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

/// Debug endpoint: show parsed nodes and rendered body HTML
async fn api_render() -> impl IntoResponse {
    match fs::read_to_string("src/main.frr") {
        Ok(content) => {
            let mut parser = FerrumParser::new();
            match parser.parse(&content) {
                Ok(nodes) => {
                    let body_html = generate_body_html_from_nodes(&nodes).unwrap_or_default();
                    let response = json!({
                        "source": "src/main.frr",
                        "nodes_debug": format!("{:#?}", nodes),
                        "body_html": body_html,
                    });
                    (StatusCode::OK, Json(response)).into_response()
                }
                Err(e) => {
                    let response = json!({
                        "source": "src/main.frr",
                        "error": format!("{}", e),
                    });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
                }
            }
        }
        Err(e) => {
            let response = json!({
                "source": "src/main.frr",
                "error": format!("{}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Format a .frr file
async fn api_format() -> impl IntoResponse {
    match fs::read_to_string("src/main.frr") {
        Ok(content) => {
            let formatter = FerrumFormatter::default();
            match formatter.format(&content) {
                Ok(formatted) => {
                    let response = json!({
                        "original": content,
                        "formatted": formatted,
                    });
                    (StatusCode::OK, Json(response)).into_response()
                }
                Err(e) => {
                    let response = json!({
                        "error": format!("{}", e),
                    });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
                }
            }
        }
        Err(e) => {
            let response = json!({
                "error": format!("{}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Generate main page from main.frr - NO JavaScript!
async fn generate_main_page(State(_state): State<Arc<RwLock<ServerState>>>) -> impl IntoResponse {
    // Try to read and compile main.frr
    match compile_main_frr() {
        Ok(html_content) => Html(html_content).into_response(),
        Err(e) => {
            let error_html = generate_error_page(&format!("Failed to compile main.frr: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Html(error_html)).into_response()
        }
    }
}

/// Generate individual component pages
async fn generate_component_page(
    AxumPath(component): AxumPath<String>,
    State(_state): State<Arc<RwLock<ServerState>>>,
) -> Response {
    let component_path = format!("src/components/{}.frr", component);

    match compile_frr_file(Path::new(&component_path)) {
        Ok(html_content) => Html(html_content).into_response(),
        Err(e) => {
            let error_html =
                generate_error_page(&format!("Failed to compile component {}: {}", component, e));
            (StatusCode::INTERNAL_SERVER_ERROR, Html(error_html)).into_response()
        }
    }
}

/// API endpoint for server status
async fn api_status(State(state): State<Arc<RwLock<ServerState>>>) -> impl IntoResponse {
    let current_state = state.read().await;

    let status = json!({
        "framework": "Ferrum",
        "version": "0.1.0",
        "server": "Pure Rust (No JavaScript)",
        "status": "running",
        "port": 7777,
        "last_reload": current_state.last_reload.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        "compiled_files": current_state.compiled_files.len(),
        "active_routes": current_state.active_routes.len(),
        "features": {
            "auto_format": false,
            "live_reload": false,
            "manual_save": true,
            "hot_reload": false,
        }
    });

    (StatusCode::OK, Json(status))
}

/// API endpoint to list all compiled components
async fn api_components(State(state): State<Arc<RwLock<ServerState>>>) -> impl IntoResponse {
    let current_state = state.read().await;

    let components = json!({
        "components": current_state.compiled_files,
        "total": current_state.compiled_files.len()
    });

    (StatusCode::OK, Json(components))
}

/// API endpoint to manually save and format a file
async fn api_save(State(state): State<Arc<RwLock<ServerState>>>) -> impl IntoResponse {
    // Format and save main.frr file
    let main_frr_path = Path::new("src/main.frr");

    match format_and_save_frr_file(&main_frr_path) {
        Ok((formatted, compiled)) => {
            // Update server state
            let mut current_state = state.write().await;
            current_state.last_reload = SystemTime::now();
            let path_str = main_frr_path.to_string_lossy().to_string();
            current_state
                .compiled_files
                .insert(path_str.clone(), compiled.clone());

            let response = json!({
                "message": "File formatted and saved successfully",
                "file": "src/main.frr",
                "timestamp": current_state.last_reload.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                "formatted": formatted,
                "compiled": compiled
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = json!({
                "error": format!("Failed to format and save file: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// Compile main.frr file
fn compile_main_frr() -> Result<String> {
    let main_frr_path = Path::new("src/main.frr");
    compile_frr_file(&main_frr_path)
}

/// Compile individual .frr file
fn compile_frr_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    let mut parser = FerrumParser::new();

    // Parse .frr content
    let nodes = parser.parse(&content)?;

    // Generate HTML directly from .frr (no JavaScript!)
    let html_content = generate_html_from_nodes(&nodes)?;

    Ok(html_content)
}

/// Format and save a .frr file (manual save)
fn format_and_save_frr_file(path: &Path) -> Result<(String, String)> {
    let content = fs::read_to_string(path)?;
    let formatter = FerrumFormatter::default();

    // Format content
    let formatted = formatter
        .format(&content)
        .map_err(|e| anyhow!("Format error: {}", e))?;

    // Write formatted content back to file
    fs::write(path, &formatted)?;

    // Compile formatted content
    let mut parser = FerrumParser::new();
    let nodes = parser
        .parse(&formatted)
        .map_err(|e| anyhow!("Parse error: {}", e))?;
    let html_content = generate_html_from_nodes(&nodes)?;

    Ok((formatted, html_content))
}

/// Generate inner body HTML for nodes (no <html>/<head>)
fn generate_body_html_from_nodes(nodes: &[ferrum_core::parser::FerrumNode]) -> Result<String> {
    let mut html = String::new();
    for node in nodes {
        html.push_str(&node_to_html(node)?);
    }
    Ok(html)
}

/// Generate pure HTML from parsed .frr nodes (NO JavaScript)
fn generate_html_from_nodes(nodes: &[ferrum_core::parser::FerrumNode]) -> Result<String> {
    let mut html = String::new();

    // Generate full HTML page
    html.push_str("<!DOCTYPE html>");
    html.push_str("<html lang='en'>");
    html.push_str("<head>");
    html.push_str("<meta charset='UTF-8'>");
    html.push_str("<meta name='viewport' content='width=device-width, initial-scale=1.0'>");
    html.push_str("<title>Ferrum App - Pure Rust</title>");

    // Pure CSS styling (no Tailwind JS)
    html.push_str("<style>");
    html.push_str(include_str!("../static/ferrum.css"));
    html.push_str("</style>");

    html.push_str("</head>");
    html.push_str("<body>");
    html.push_str("<div id='ferrum-app'>");

    // Generate HTML from nodes
    for node in nodes {
        html.push_str(&node_to_html(node)?);
    }

    html.push_str("</div>");
    html.push_str("</body>");
    html.push_str("</html>");

    Ok(html)
}

/// Convert Ferrum node to HTML (NO JavaScript)
fn node_to_html(node: &ferrum_core::parser::FerrumNode) -> Result<String> {
    match node {
        ferrum_core::parser::FerrumNode::Element {
            tag,
            props,
            children,
        } => {
            let mut html = format!("<{}", tag);

            // Add props
            for (key, value) in props {
                html.push_str(&format!(" {}='{}'", key, value));
            }

            html.push('>');

            // Add children
            for child in children {
                html.push_str(&node_to_html(child)?);
            }

            html.push_str(&format!("</{}>", tag));
            Ok(html)
        }
        ferrum_core::parser::FerrumNode::Text(text) => Ok(text.clone()),
        ferrum_core::parser::FerrumNode::Component {
            name,
            props,
            children,
        } => {
            // For components, generate div with component name
            let mut html = format!("<div data-component='{}'", name);

            // Add props as data attributes
            for (key, value) in props {
                html.push_str(&format!(" data-{}='{}'", key, value));
            }

            html.push('>');

            for child in children {
                html.push_str(&node_to_html(child)?);
            }

            html.push_str("</div>");
            Ok(html)
        }
        _ => Ok(String::new()),
    }
}

/// Generate error page (pure HTML)
fn generate_error_page(error_message: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <title>Ferrum Error</title>
    <style>
        body {{
            font-family: system-ui, sans-serif;
            background: #1a1a1a;
            color: white;
            margin: 0;
            padding: 2rem;
        }}
        .error-container {{
            max-width: 600px;
            margin: 0 auto;
            background: #2d2d2d;
            border-radius: 8px;
            padding: 2rem;
            border: 1px solid #ef4444;
        }}
        .error-title {{
            color: #ef4444;
            font-size: 1.5rem;
            margin-bottom: 1rem;
        }}
        .error-message {{
            font-size: 1rem;
            line-height: 1.5;
        }}
    </style>
</head>
<body>
    <div class='error-container'>
        <h1 class='error-title'>⚠️ Ferrum Compilation Error</h1>
        <p class='error-message'>{}</p>
    </div>
</body>
</html>
    "#,
        error_message
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    console_log::init_with_level(log::Level::Info).map_err(|e| anyhow!(e.to_string()))?;

    let args: Vec<String> = std::env::args().collect();
    let port = args
        .get(1)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(7777);

    // Find project root
    let current_dir = std::env::current_dir()?;
    let project_path = current_dir.to_string_lossy().to_string();

    // Validate it's a Ferrum project
    if !Path::new(&project_path).join("src/main.frr").exists() {
        eprintln!("❌ Error: Not a Ferrum project directory");
        eprintln!("   Make sure you're in a directory with src/main.frr");
        eprintln!("   No JavaScript, No Single HTML - Pure Rust only!");
        std::process::exit(1);
    }

    // Start pure Rust dev server
    let server = RustDevServer::new(project_path, port)?;
    server.run().await
}
