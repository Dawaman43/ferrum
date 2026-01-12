use super::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

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
    fn test_state_binding_with_operation() {
        let input = r#"
count.value
"#;

        let mut parser = FerrumParser::new();
        let nodes = parser.parse(input).unwrap();
        assert_eq!(nodes.len(), 1);

        match &nodes[0] {
            FerrumNode::StateBinding { signal, operation } => {
                assert_eq!(signal, "count");
                assert_eq!(operation, "value");
            }
            _ => panic!("Expected StateBinding node"),
        }
    }

    #[test]
    fn test_state_binding_signal_access() {
        let input = r#"
count
"#;

        let mut parser = FerrumParser::new();
        let nodes = parser.parse(input).unwrap();
        assert_eq!(nodes.len(), 1);

        match &nodes[0] {
            FerrumNode::StateBinding { signal, operation } => {
                assert_eq!(signal, "count");
                assert_eq!(operation, "");
            }
            _ => panic!("Expected StateBinding node"),
        }
    }

    #[test]
    fn test_import_parsing() {
        let input = r#"
import { create_signal } from "ferrum:state"
"#;

        let mut parser = FerrumParser::new();
        let nodes = parser.parse(input).unwrap();
        assert_eq!(nodes.len(), 1);

        match &nodes[0] {
            FerrumNode::Import { names, from } => {
                assert_eq!(names, &["create_signal"]);
                assert_eq!(from, "ferrum:state");
            }
            _ => panic!("Expected Import node"),
        }
    }
}
