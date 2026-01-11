use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Trait that all components must implement
pub trait Component: 'static {
    type Props;
    type Msg;
    
    fn create(props: Self::Props) -> Self where Self: Sized;
    fn update(&mut self, msg: Self::Msg) -> bool;
    fn view(&self) -> ComponentView;
}

/// Represents a rendered component view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentView {
    pub tag: String,
    pub props: HashMap<String, PropValue>,
    pub children: Vec<ComponentView>,
}

/// Values that can be assigned to component properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<PropValue>),
    Object(HashMap<String, PropValue>),
    Null,
}

/// Component registry for managing component instances
pub struct ComponentRegistry {
    components: HashMap<String, String>, // Simplified for now - store component names
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, name: String) {
        self.components.insert(name.clone(), "registered".to_string());
        log::debug!("Registered component: {}", name);
    }
    
    pub fn get_component(&self, name: &str) -> Option<&str> {
        self.components.get(name).map(|s| s.as_str())
    }
}

/// Macro for creating components with reduced boilerplate
#[macro_export]
macro_rules! component {
    (
        $name:ident {
            props: { $($prop_name:ident: $prop_type:ty),* $(,)? },
            state: { $($state_name:ident: $state_type:ty),* $(,)? },
            msg: { $($msg_variant:ident $(($msg_data:ty))? ),* $(,)? }
        }
    ) => {
        // This macro would expand to generate the full component implementation
        // For now, it's a placeholder for the macro we'll implement later
        struct $name {
            $($state_name: $state_type,)*
        }
    };
}