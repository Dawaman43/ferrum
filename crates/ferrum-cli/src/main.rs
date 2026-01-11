use clap::{Parser, Subcommand};
use std::path::Path;
use ferrum_core::parser;

fn create_project(name: &str, _template: &str) -> std::io::Result<()> {
    println!("Initializing Ferrum project: {}", name);
    
    // Create project directory structure
    std::fs::create_dir_all(format!("{}/src/components", name))?;
    std::fs::create_dir_all(format!("{}/src/pages", name))?;
    std::fs::create_dir_all(format!("{}/src/api", name))?;
    std::fs::create_dir_all(format!("{}/style", name))?;
    std::fs::create_dir_all(format!("{}/tests", name))?;
    
    // Create main.frr file
    let main_frr = format!(
        r#"// main.frr - Entry point for Ferrum application
import {{ create_signal }} from "ferrum:state"
import {{ css }} from "ferrum:css"

App()
    div.flex.flex-col.items-center.justify-center.min-h-screen.p-8
        h1.text-3xl.font-bold.text-blue-600 "Welcome to {}"
        p.text-gray-600.mt-4 "Your Ferrum app is ready!"
        
        Button(onclick: set_count(count + 1))
            "Click count: {{count}}"
"#,
        name
    );
    
    std::fs::write(format!("{}/src/main.frr", name), main_frr)?;
    
    // Create Button.frr component
    let button_frr = r#"// Button.frr - Reusable button component
Button(onclick: null, variant: "primary", children: "")
    button.rounded.font-medium.py-2.px-4(
        class: variant == "primary" ? css!(bg-blue-500, text-white) :
               variant == "secondary" ? css!(bg-gray-200, text-gray-800) :
               css!(bg-transparent, text-blue-500, border),
        onclick: onclick
    )
        {children}"#;
    
    std::fs::write(format!("{}/src/components/Button.frr", name), button_frr)?;
    
    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
ferrum-core = {{ git = "https://github.com/ferrum-web/ferrum" }}
ferrum-frontend = {{ git = "https://github.com/ferrum-web/ferrum" }}
leptos = "0.6"
leptos_meta = "0.6"
leptos_router = "0.6"
wasm-bindgen = "0.2"
console_log = "1.0"
log = "0.4"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

[dependencies.web-sys]
version = "0.3"
features = [
    "Document",
    "Element", 
    "HtmlElement",
    "Window",
    "console",
]
"#,
        name
    );
    
    std::fs::write(format!("{}/Cargo.toml", name), cargo_toml)?;
    
    // Create index.html
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferrum App</title>
    <style>
        body {
            margin: 0;
            font-family: system-ui, -apple-system, sans-serif;
            background-color: #f9fafb;
        }
    </style>
</head>
<body>
    <div id="root"></div>
</body>
</html>"#;
    
    std::fs::write(format!("{}/index.html", name), index_html)?;
    
    println!("✅ Project '{}' created successfully!", name);
    println!("📁 Next steps:");
    println!("   cd {}", name);
    println!("   ferrum dev");
    
    Ok(())
}

fn start_dev_server() -> std::io::Result<()> {
    // Check if current directory is a Ferrum project
    if !std::path::Path::new("src/main.frr").exists() {
        eprintln!("❌ Error: Not a Ferrum project directory");
        eprintln!("   Make sure you're in a project with src/main.frr");
        eprintln!("   Pure Rust - NO JavaScript, NO Single HTML");
        std::process::exit(1);
    }
    
    println!("🦀 Starting Pure Rust Ferrum Server");
    println!("📁 Project: {}", std::env::current_dir().unwrap().display());
    println!("🌐 Port: 7777");
    println!("🔥 Pure Rust: NO JavaScript, NO Single HTML");
    println!("👀 Watching .frr files...");
    
    // Start the actual dev server
    let dev_server_path = std::env::current_dir()
        .unwrap()
        .join("../target/debug/ferrum-dev-server");
    
    if dev_server_path.exists() {
        println!("🚀 Launching dev server...");
        std::process::Command::new(&dev_server_path)
            .arg("7777")
            .spawn()
            .expect("Failed to start dev server");
        
        println!("✨ Server started at: http://localhost:7777");
        println!("📝 Features:");
        println!("   • Pure Rust server (NO JavaScript)");
        println!("   • Hot reload for .frr files");
        println!("   • HTML generation from .frr");
        println!("   • CSS-in-Rust styling");
        println!("   • Component compilation");
    } else {
        println!("⚠️  Dev server not built yet. Run: cargo build --package ferrum-dev-server");
        println!("📝 Features (when built):");
        println!("   • Pure Rust server (NO JavaScript)");
        println!("   • Hot reload for .frr files");
        println!("   • HTML generation from .frr");
        println!("   • CSS-in-Rust styling");
        println!("   • Component compilation");
    }
    
    Ok(())
}

#[derive(Parser)]
#[command(name = "ferrum")]
#[command(about = "The Rust full-stack framework that will revolutionize web development")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Ferrum project
    Create {
        /// Project name
        name: String,
        #[arg(long, short, default_value = "fullstack")]
        template: String,
    },
    /// Start development server with hot reload
    Dev,
    /// Build for production
    Build,
    /// Run tests
    Test,
    /// Deploy application
    Deploy {
        #[arg(long, short, default_value = "local")]
        provider: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Create { name, template } => {
            println!("Creating new Ferrum project: {}", name);
            println!("Template: {}", template);
            create_project(&name, &template)?;
            Ok(())
        }
        Commands::Dev => {
            println!("Starting Ferrum development server...");
Ok(start_dev_server()?)
        }
        Commands::Build => {
            println!("Building Ferrum application for production...");
            // TODO: Implement build process
            Ok(())
        }
        Commands::Test => {
            println!("Running tests...");
            // TODO: Implement test runner
            Ok(())
        }
        Commands::Deploy { provider } => {
            println!("Deploying to: {}", provider);
            // TODO: Implement deployment
            Ok(())
        }
    }
}