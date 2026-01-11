//! Ferrum Core - The heart of the framework
//! 
//! This crate provides the fundamental abstractions and utilities
//! that power the entire Ferrum ecosystem.

use thiserror::Error;

pub mod component;
pub mod state;
pub mod routing;
pub mod css;
pub mod parser;

/// Core error types for the framework
#[derive(Error, Debug)]
pub enum FerrumError {
    #[error("Component error: {0}")]
    Component(String),
    
    #[error("State management error: {0}")]
    State(String),
    
    #[error("Routing error: {0}")]
    Routing(String),
    
    #[error("CSS compilation error: {0}")]
    Css(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type used throughout the framework
pub type Result<T> = std::result::Result<T, FerrumError>;

/// Core configuration for Ferrum applications
#[derive(Debug, Clone)]
pub struct FerrumConfig {
    pub app_name: String,
    pub debug: bool,
    pub hot_reload: bool,
}

impl Default for FerrumConfig {
    fn default() -> Self {
        Self {
            app_name: "Ferrum App".to_string(),
            debug: std::env::var("FERRUM_DEBUG").is_ok(),
            hot_reload: std::env::var("FERRUM_HOT_RELOAD").is_ok(),
        }
    }
}

/// Initialize the Ferrum framework
pub fn init(config: FerrumConfig) -> Result<()> {
    // Initialize logging
    let log_level = if config.debug { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", log_level);
    
    log::info!("Ferrum framework initialized: {}", config.app_name);
    Ok(())
}