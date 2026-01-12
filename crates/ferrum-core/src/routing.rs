use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::component::{ComponentView, PropValue};

/// Routing system for Ferrum applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub component: String,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

/// Router for managing application routes
pub struct Router {
    routes: Vec<Route>,
    current_route: Signal<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            current_route: Signal::new(Route {
                path: "/".to_string(),
                component: "Home".to_string(),
                params: HashMap::new(),
                query: HashMap::new(),
            }),
        }
    }
    
    pub fn add_route(&mut self, path: &str, component: &str) {
        let route = Route {
            path: path.to_string(),
            component: component.to_string(),
            params: HashMap::new(),
            query: HashMap::new(),
        };
        
        self.routes.push(route);
        log::debug!("Added route: {} -> {}", path, component);
    }
    
    pub fn navigate(&self, path: &str) {
        // Parse the path and find matching route
        if let Some(route) = self.find_route(path) {
            self.current_route.set(route);
            
            // Update browser history (only available on client)
            #[cfg(feature = "client")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(history) = window.history() {
                        let _ = history.push_state_with_url(
                            &wasm_bindgen::JsValue::NULL,
                            "",
                            Some(path),
                        );
                    }
                }
            }
        }
    }
    
    fn find_route(&self, path: &str) -> Option<Route> {
        // Simple exact match for now
        // TODO: Implement parameter matching and wildcards
        self.routes
            .iter()
            .find(|route| route.path == path)
            .cloned()
    }
    
    pub fn current_route(&self) -> Signal<Route> {
        self.current_route.clone()
    }
}

/// Navigation hooks
#[cfg(feature = "client")]
pub fn use_navigate() -> impl Fn(&str) {
    |path: &str| {
        if let Some(window) = web_sys::window() {
            let _ = window.location().assign(path);
        }
    }
}

#[cfg(not(feature = "client"))]
pub fn use_navigate() -> impl Fn(&str) {
    |_: &str| {
        // No-op on server
    }
}

/// Hook for accessing current route parameters
pub fn use_params() -> HashMap<String, String> {
    // TODO: Implement parameter extraction from current route
    HashMap::new()
}

/// Hook for accessing query parameters
pub fn use_query() -> HashMap<String, String> {
    // TODO: Implement query parameter extraction
    HashMap::new()
}

/// Link component for navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkProps {
    pub to: String,
    pub class: Option<String>,
    pub children: String,
}

pub fn Link(props: LinkProps) -> ComponentView {
    let onclick = format!("ferrum.navigate('{}')", props.to);
    
    ComponentView {
        tag: "a".to_string(),
        props: {
            let mut map = HashMap::new();
            map.insert("href".to_string(), PropValue::String(props.to.clone()));
            map.insert("onclick".to_string(), PropValue::String(onclick));
            if let Some(class) = props.class {
                map.insert("class".to_string(), PropValue::String(class));
            }
            map
        },
        children: vec![ComponentView {
            tag: "span".to_string(),
            props: HashMap::new(),
            children: vec![],
        }],
    }
}

// Re-export Signal from state module
use crate::state::Signal;