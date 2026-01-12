use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CSS-in-Rust styling system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub properties: HashMap<String, String>,
    pub pseudo_classes: HashMap<String, Style>,
    pub media_queries: Vec<(String, Style)>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            pseudo_classes: HashMap::new(),
            media_queries: Vec::new(),
        }
    }

    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }

    pub fn hover(mut self, style: Style) -> Self {
        self.pseudo_classes.insert(":hover".to_string(), style);
        self
    }

    pub fn focus(mut self, style: Style) -> Self {
        self.pseudo_classes.insert(":focus".to_string(), style);
        self
    }

    pub fn media(mut self, query: &str, style: Style) -> Self {
        self.media_queries.push((query.to_string(), style));
        self
    }
}

/// CSS units and values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CssUnit {
    Px(f64),
    Rem(f64),
    Em(f64),
    Percent(f64),
    Vw(f64),
    Vh(f64),
    Auto,
}

impl std::fmt::Display for CssUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssUnit::Px(val) => write!(f, "{}px", val),
            CssUnit::Rem(val) => write!(f, "{}rem", val),
            CssUnit::Em(val) => write!(f, "{}em", val),
            CssUnit::Percent(val) => write!(f, "{}%", val),
            CssUnit::Vw(val) => write!(f, "{}vw", val),
            CssUnit::Vh(val) => write!(f, "{}vh", val),
            CssUnit::Auto => write!(f, "auto"),
        }
    }
}

/// Tailwind-like utility classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UtilityClass {
    // Layout
    Flex,
    Grid,
    Block,
    Inline,
    Hidden,

    // Flexbox
    FlexRow,
    FlexCol,
    JustifyCenter,
    JustifyBetween,
    ItemsCenter,
    ItemsStart,

    // Spacing
    P(u8), // padding, 0-8 mapped to values
    M(u8), // margin, 0-8 mapped to values

    // Typography
    TextSm,
    TextBase,
    TextLg,
    TextXl,
    FontBold,
    FontMedium,

    // Colors
    BgRed500,
    BgBlue500,
    BgGreen500,
    TextWhite,
    TextGray800,

    // Sizing
    WAuto,
    WFull,
    HAuto,
    HFull,

    // Border
    Border,
    Border2,
    Rounded,
    RoundedLg,

    // Effects
    Shadow,
    ShadowLg,
    Opacity50,
}

impl UtilityClass {
    pub fn to_css(&self) -> String {
        match self {
            // Layout
            UtilityClass::Flex => "display: flex;".to_string(),
            UtilityClass::Grid => "display: grid;".to_string(),
            UtilityClass::Block => "display: block;".to_string(),
            UtilityClass::Inline => "display: inline;".to_string(),
            UtilityClass::Hidden => "display: none;".to_string(),

            // Flexbox
            UtilityClass::FlexRow => "flex-direction: row;".to_string(),
            UtilityClass::FlexCol => "flex-direction: column;".to_string(),
            UtilityClass::JustifyCenter => "justify-content: center;".to_string(),
            UtilityClass::JustifyBetween => "justify-content: space-between;".to_string(),
            UtilityClass::ItemsCenter => "align-items: center;".to_string(),
            UtilityClass::ItemsStart => "align-items: flex-start;".to_string(),

            // Spacing
            UtilityClass::P(n) => format!("padding: {}rem;", *n as f64 * 0.25),
            UtilityClass::M(n) => format!("margin: {}rem;", *n as f64 * 0.25),

            // Typography
            UtilityClass::TextSm => "font-size: 0.875rem;".to_string(),
            UtilityClass::TextBase => "font-size: 1rem;".to_string(),
            UtilityClass::TextLg => "font-size: 1.125rem;".to_string(),
            UtilityClass::TextXl => "font-size: 1.25rem;".to_string(),
            UtilityClass::FontBold => "font-weight: bold;".to_string(),
            UtilityClass::FontMedium => "font-weight: 500;".to_string(),

            // Colors
            UtilityClass::BgRed500 => "background-color: #ef4444;".to_string(),
            UtilityClass::BgBlue500 => "background-color: #3b82f6;".to_string(),
            UtilityClass::BgGreen500 => "background-color: #10b981;".to_string(),
            UtilityClass::TextWhite => "color: white;".to_string(),
            UtilityClass::TextGray800 => "color: #1f2937;".to_string(),

            // Sizing
            UtilityClass::WAuto => "width: auto;".to_string(),
            UtilityClass::WFull => "width: 100%;".to_string(),
            UtilityClass::HAuto => "height: auto;".to_string(),
            UtilityClass::HFull => "height: 100%;".to_string(),

            // Border
            UtilityClass::Border => "border: 1px solid #e5e7eb;".to_string(),
            UtilityClass::Border2 => "border: 2px solid #e5e7eb;".to_string(),
            UtilityClass::Rounded => "border-radius: 0.25rem;".to_string(),
            UtilityClass::RoundedLg => "border-radius: 0.5rem;".to_string(),

            // Effects
            UtilityClass::Shadow => "box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);".to_string(),
            UtilityClass::ShadowLg => {
                "box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);".to_string()
            }
            UtilityClass::Opacity50 => "opacity: 0.5;".to_string(),
        }
    }
}

/// CSS builder for combining utility classes
pub struct CssBuilder {
    classes: Vec<UtilityClass>,
    custom_styles: Vec<String>,
}

impl CssBuilder {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
            custom_styles: Vec::new(),
        }
    }

    pub fn add(mut self, class: UtilityClass) -> Self {
        self.classes.push(class);
        self
    }

    pub fn custom(mut self, css: &str) -> Self {
        self.custom_styles.push(css.to_string());
        self
    }

    pub fn build(self) -> String {
        let mut css_string = String::new();

        // Add utility class CSS
        for class in &self.classes {
            css_string.push_str(&class.to_css());
            css_string.push(' ');
        }

        // Add custom CSS
        for style in &self.custom_styles {
            css_string.push_str(style);
            css_string.push(' ');
        }

        css_string.trim().to_string()
    }
}

/// Macro for CSS utility classes (similar to Tailwind)
#[macro_export]
macro_rules! css {
    ($($class:ident$(($param:expr))?),+ $(,)?) => {
        {
            let mut builder = $crate::css::CssBuilder::new();
            $(
                builder = builder.add($crate::css::UtilityClass::$class$(($param))?);
            )+
            builder.build()
        }
    };
}

/// Hook for using styles in components
pub fn use_style() -> CssBuilder {
    CssBuilder::new()
}

/// Theme system for consistent design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub colors: Colors,
    pub spacing: Spacing,
    pub typography: Typography,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Colors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text_primary: String,
    pub text_secondary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub sizes: HashMap<String, String>,
    pub weights: HashMap<String, String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: Colors {
                primary: "#3b82f6".to_string(),
                secondary: "#6b7280".to_string(),
                accent: "#10b981".to_string(),
                background: "#ffffff".to_string(),
                surface: "#f9fafb".to_string(),
                text_primary: "#111827".to_string(),
                text_secondary: "#6b7280".to_string(),
            },
            spacing: Spacing {
                xs: "0.25rem".to_string(),
                sm: "0.5rem".to_string(),
                md: "1rem".to_string(),
                lg: "1.5rem".to_string(),
                xl: "2rem".to_string(),
            },
            typography: Typography {
                font_family: "system-ui, -apple-system, sans-serif".to_string(),
                sizes: {
                    let mut sizes = HashMap::new();
                    sizes.insert("xs".to_string(), "0.75rem".to_string());
                    sizes.insert("sm".to_string(), "0.875rem".to_string());
                    sizes.insert("base".to_string(), "1rem".to_string());
                    sizes.insert("lg".to_string(), "1.125rem".to_string());
                    sizes.insert("xl".to_string(), "1.25rem".to_string());
                    sizes.insert("2xl".to_string(), "1.5rem".to_string());
                    sizes
                },
                weights: {
                    let mut weights = HashMap::new();
                    weights.insert("normal".to_string(), "400".to_string());
                    weights.insert("medium".to_string(), "500".to_string());
                    weights.insert("bold".to_string(), "700".to_string());
                    weights
                },
            },
        }
    }
}
