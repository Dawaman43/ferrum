use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};

/// Ferrum Language - Ultra-simple web language
/// Compiles ! syntax to WebAssembly
/// File extension: .frr (Ferrum Resource)

#[derive(Debug, Clone)]
pub struct FerrumParser {
    components: HashMap<String, String>,
    current_indent: usize,
}

#[derive(Debug, Clone)]
pub enum FerrumNode {
    Element {
        tag: String,
        props: HashMap<String, String>,
        children: Vec<FerrumNode>,
    },
    Text(String),
    Component {
        name: String,
        props: HashMap<String, String>,
        children: Vec<FerrumNode>,
    },
    StateBinding {
        signal: String,
        operation: String,
    },
}

impl FerrumParser {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            current_indent: 0,
        }
    }
    
    /// Parse Ferrum DSL syntax
    pub fn parse(&mut self, input: &str) -> Result<Vec<FerrumNode>> {
        let lines: Vec<&str> = input.lines().collect();
        let mut nodes = Vec::new();
        let mut stack = Vec::new();
        
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            
            let indent = line.len() - trimmed.len();
            
            // Close nodes that are deeper than current indent
            while stack.len() > (indent / 2) + 1 {
                stack.pop();
            }
            
let node = self.parse_line(trimmed)?;
            
            let indent = line.len() - trimmed.len();
            
            // Close nodes that are deeper than current indent
            while stack.len() > (indent / 2) + 1 {
                stack.pop();
            }
            
            match stack.last_mut() {
                Some(parent) => {
                    if let Some(children) = self.get_children_mut(parent) {
                        children.push(node.clone());
                    }
                }
                None => nodes.push(node.clone()),
            }
            
            if self.has_children(&node) {
                stack.push(node);
            }
        }
        
        Ok(nodes)
    }
    
    fn parse_line(&self, line: &str) -> Result<FerrumNode> {
        // Handle component syntax: ComponentName(prop: value, prop: value)
        if let Some(component_end) = line.find('(') {
            if line.ends_with(')') {
                let component_name = line[..component_end].trim().to_string();
                let props_str = &line[component_end + 1..line.len() - 1];
                let props = self.parse_props(props_str)?;
                
                return Ok(FerrumNode::Component {
                    name: component_name,
                    props,
                    children: Vec::new(),
                });
            }
        }
        
        // Handle HTML-like syntax: tag#id.class prop="value"
        if line.contains('<') {
            return self.parse_html_element(line);
        }
        
        // Handle simple text
        if !line.contains([' ', '#', '.', '(']) {
            return Ok(FerrumNode::Text(line.to_string()));
        }
        
        // Handle tag shorthand syntax
        self.parse_tag_shorthand(line)
    }
    
    fn parse_props(&self, props_str: &str) -> Result<HashMap<String, String>> {
        let mut props = HashMap::new();
        let pairs = props_str.split(',');
        
        for pair in pairs {
            let pair = pair.trim();
            if let Some(colon_pos) = pair.find(':') {
                let key = pair[..colon_pos].trim().to_string();
                let value = pair[colon_pos + 1..].trim().to_string();
                props.insert(key, value);
            }
        }
        
        Ok(props)
    }
    
    fn parse_html_element(&self, line: &str) -> Result<FerrumNode> {
        // Extract tag from <tag> syntax
        if let Some(start) = line.find('<') {
            if let Some(end) = line.find('>') {
                let tag_content = &line[start + 1..end];
                let (tag, rest) = tag_content.split_once(' ').unwrap_or((tag_content, ""));
                
                let mut props = HashMap::new();
                
                // Parse remaining props
                if !rest.is_empty() {
                    for part in rest.split_whitespace() {
                        if let Some(eq_pos) = part.find('=') {
                            let key = &part[..eq_pos];
                            let value = &part[eq_pos + 1..];
                            if value.starts_with('"') && value.ends_with('"') {
                                props.insert(key.to_string(), value[1..value.len() - 1].to_string());
                            }
                        }
                    }
                }
                
                return Ok(FerrumNode::Element {
                    tag: tag.to_string(),
                    props,
                    children: Vec::new(),
                });
            }
        }
        
        Err(anyhow!("Invalid HTML element syntax: {}", line))
    }
    
fn parse_tag_shorthand(&self, line: &str) -> Result<FerrumNode> {
        let mut parts = line.split_whitespace();
        let first_part = parts.next().ok_or_else(|| anyhow!("Empty line"))?;
        
        // Parse tag with #id and .class syntax
        let (tag_part, rest) = first_part.split_once('#').unwrap_or((first_part, ""));
        let (id_part, class_part) = if !rest.is_empty() {
            rest.split_once('.').unwrap_or((rest, ""))
        } else {
            ("", "")
        };
        
        let mut props = HashMap::new();
        if !id_part.is_empty() {
            props.insert("id".to_string(), id_part.to_string());
        }
        
        let mut classes = Vec::new();
        if !class_part.is_empty() {
            classes.push(class_part.to_string());
        }
        
        // Parse remaining classes and props
        for part in parts {
            if part.starts_with('.') {
                classes.push(part[1..].to_string());
            } else if let Some(eq_pos) = part.find('=') {
                let key = &part[..eq_pos];
                let value = &part[eq_pos + 1..];
                if value.starts_with('"') && value.ends_with('"') {
                    props.insert(key.to_string(), value[1..value.len() - 1].to_string());
                }
            }
        }
        
        if !classes.is_empty() {
            props.insert("class".to_string(), classes.join(" "));
        }
        
        Ok(FerrumNode::Element {
            tag: tag_part.to_string(),
            props,
            children: Vec::new(),
        })
    }
    
    fn has_children(&self, node: &FerrumNode) -> bool {
        match node {
            FerrumNode::Element { tag, .. } => {
                !["input", "img", "br", "hr", "meta", "link"].contains(&tag.as_str())
            }
            FerrumNode::Component { .. } => true,
            FerrumNode::Text(_) => false,
            FerrumNode::StateBinding { .. } => false,
        }
    }
    
    fn get_children_mut<'a>(&self, node: &'a mut FerrumNode) -> Option<&'a mut Vec<FerrumNode>> {
        match node {
            FerrumNode::Element { children, .. } => Some(children),
            FerrumNode::Component { children, .. } => Some(children),
            FerrumNode::Text(_) => None,
            FerrumNode::StateBinding { .. } => None,
        }
    }
    
    /// Generate Rust code from parsed nodes
    pub fn generate_rust(&self, nodes: &[FerrumNode]) -> Result<String> {
        let mut code = String::new();
        code.push_str("use leptos::*;\n\n");
        
        for node in nodes {
            code.push_str(&self.node_to_rust(node)?);
            code.push('\n');
        }
        
        Ok(code)
    }
    
    fn node_to_rust(&self, node: &FerrumNode) -> Result<String> {
        match node {
            FerrumNode::Element { tag, props, children } => {
                let mut rust = format!("view! {{\n    <{} ", tag);
                
                // Add props
                for (key, value) in props {
                    if key == "class" {
                        rust.push_str(&format!("class=\"{}\" ", value));
                    } else if key == "id" {
                        rust.push_str(&format!("id=\"{}\" ", value));
                    } else {
                        rust.push_str(&format!("{}=\"{}\" ", key, value));
                    }
                }
                
                if children.is_empty() {
                    rust.push_str("/>\n}}");
                } else {
                    rust.push_str(">\n");
                    for child in children {
                        rust.push_str(&format!("        {}\n", self.child_to_rust(child)?));
                    }
                    rust.push_str(&format!("    </{}>\n}}", tag));
                }
                
                Ok(rust)
            }
            FerrumNode::Text(text) => Ok(format!("\"{}\"", text)),
            FerrumNode::Component { name, props, children } => {
                let mut rust = format!("view! {{\n    <{} ", name);
                
                for (key, value) in props {
                    rust.push_str(&format!("{}={} ", key, value));
                }
                
                if children.is_empty() {
                    rust.push_str("/>\n}}");
                } else {
                    rust.push_str(">\n");
                    for child in children {
                        rust.push_str(&format!("        {}\n", self.child_to_rust(child)?));
                    }
                    rust.push_str(&format!("    </{}>\n}}", name));
                }
                
                Ok(rust)
            }
            FerrumNode::StateBinding { signal, operation } => {
                Ok(format!("{{move || {}.{}}}", signal, operation))
            }
        }
    }
    
    fn child_to_rust(&self, node: &FerrumNode) -> Result<String> {
        match node {
            FerrumNode::Text(text) => Ok(format!("\"{}\"", text)),
            _ => self.node_to_rust(node),
        }
    }
}

/// Compile a .frr file to Rust
pub fn compile_frr_to_rust(input_path: &Path, output_path: &Path) -> Result<()> {
    let input = fs::read_to_string(input_path)?;
    let mut parser = FerrumParser::new();
    
    let nodes = parser.parse(&input)?;
    let rust_code = parser.generate_rust(&nodes)?;
    
    fs::write(output_path, rust_code)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_component() {
        let input = r#"
Button(onclick: set_count(-1))
    "-"
"#;
        
        let mut parser = FerrumParser::new();
        let nodes = parser.parse(input).unwrap();
        assert_eq!(nodes.len(), 1);
        
        match &nodes[0] {
            FerrumNode::Component { name, props, .. } => {
                assert_eq!(name, "Button");
                assert_eq!(props.get("onclick"), Some(&"set_count(-1)".to_string()));
            }
            _ => panic!("Expected component node"),
        }
    }
    
    #[test]
    fn test_html_syntax() {
        let input = r#"
div#app.container
    h1 "Hello World"
    p.text-gray-600 "Welcome to Ferrum"
"#;
        
        let mut parser = FerrumParser::new();
        let nodes = parser.parse(input).unwrap();
        assert_eq!(nodes.len(), 1);
    }
}