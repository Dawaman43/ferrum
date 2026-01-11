use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// State management system for Ferrum applications
pub trait State: Send + Sync {
    type Value: Clone + Send + Sync;
    
    fn get(&self) -> Self::Value;
    fn set(&mut self, value: Self::Value);
    fn subscribe(&mut self, callback: Box<dyn Fn(Self::Value) + Send + Sync>);
}

/// Signal-based state management (inspired by Leptos but simplified)
#[derive(Clone)]
pub struct Signal<T> {
    inner: Arc<RwLock<SignalInner<T>>>,
}

struct SignalInner<T> {
    value: T,
    subscribers: Vec<Box<dyn Fn(&T) + Send + Sync>>,
}

impl<T> Signal<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(initial_value: T) -> Self {
        let inner = SignalInner {
            value: initial_value,
            subscribers: Vec::new(),
        };
        
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }
    
    pub fn get(&self) -> T {
        let inner = self.inner.read().unwrap();
        inner.value.clone()
    }
    
    pub fn set(&self, value: T) {
        let mut inner = self.inner.write().unwrap();
        inner.value = value.clone();
        
        // Notify all subscribers
        for callback in &inner.subscribers {
            callback(&inner.value);
        }
    }
    
    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut inner = self.inner.write().unwrap();
        inner.subscribers.push(Box::new(callback));
    }
}

/// State store for managing application-wide state
pub struct Store {
    signals: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }
    
    pub fn signal<T>(&mut self, key: &str, initial_value: T) -> Signal<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        let signal = Signal::new(initial_value);
        
        // Store the signal for later access
        let signal_clone = signal.clone();
        self.signals.insert(key.to_string(), Box::new(signal_clone));
        
        signal
    }
    
    pub fn get_signal<T>(&self, key: &str) -> Option<Signal<T>>
    where
        T: Clone + Send + Sync + 'static,
    {
        self.signals
            .get(key)?
            .downcast_ref::<Signal<T>>()
            .cloned()
    }
}

/// Hook for creating signals in components
pub fn create_signal<T>(initial_value: T) -> Signal<T>
where
    T: Clone + Send + Sync + 'static,
{
    Signal::new(initial_value)
}

/// Hook for creating derived signals (computed values)
pub fn create_memo<F, T>(compute_fn: F) -> Signal<T>
where
    F: Fn() -> T + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    let initial_value = compute_fn();
    Signal::new(initial_value)
}

/// Action pattern for side effects
pub struct Action<T, R> {
    handler: Box<dyn Fn(T) -> R + Send + Sync>,
}

impl<T, R> Action<T, R>
where
    T: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(T) -> R + Send + Sync + 'static,
    {
        Self {
            handler: Box::new(handler),
        }
    }
    
    pub fn dispatch(&self, value: T) -> R {
        (self.handler)(value)
    }
}

/// Resource pattern for async data fetching
pub struct Resource<T> {
    data: Signal<Option<T>>,
    loading: Signal<bool>,
    error: Signal<Option<String>>,
}

impl<T> Resource<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            data: Signal::new(None),
            loading: Signal::new(false),
            error: Signal::new(None),
        }
    }
    
    pub async fn fetch<F, Fut>(&self, fetcher: F)
    where
        F: FnOnce() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, String>> + Send,
    {
        self.loading.set(true);
        self.error.set(None);
        
        match fetcher().await {
            Ok(result) => {
                self.data.set(Some(result));
                self.loading.set(false);
            }
            Err(err) => {
                self.error.set(Some(err));
                self.loading.set(false);
            }
        }
    }
    
    pub fn data(&self) -> Signal<Option<T>> {
        self.data.clone()
    }
    
    pub fn loading(&self) -> Signal<bool> {
        self.loading.clone()
    }
    
    pub fn error(&self) -> Signal<Option<String>> {
        self.error.clone()
    }
}