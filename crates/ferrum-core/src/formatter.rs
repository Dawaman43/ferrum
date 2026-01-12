use crate::parser::{BinaryOperator, Expression, FerrumNode, FerrumParser};
use std::fmt::Write;

/// Auto-formatter for .frr files
/// Provides consistent indentation and spacing
pub struct FerrumFormatter {
    indent_size: usize,
    indent_char: char,
}

impl Default for FerrumFormatter {
    fn default() -> Self {
        Self {
            indent_size: 4,
            indent_char: ' ',
        }
    }
}

impl FerrumFormatter {
    pub fn new(indent_size: usize, indent_char: char) -> Self {
        Self {
            indent_size,
            indent_char,
        }
    }

    /// Format .frr source code
    pub fn format(&self, input: &str) -> Result<String, String> {
        let mut parser = FerrumParser::new();
        let nodes = parser.parse(input).map_err(|e| e.to_string())?;

        let mut output = String::new();
        for node in &nodes {
            self.format_node(node, 0, &mut output)?;
        }

        Ok(output)
    }

    fn format_node(
        &self,
        node: &FerrumNode,
        depth: usize,
        output: &mut String,
    ) -> Result<(), String> {
        let indent = self.indent_string(depth);

        match node {
            FerrumNode::Element {
                tag,
                props,
                children,
            } => {
                // Format element with classes and props
                write!(output, "{}{}", indent, tag).map_err(|e| e.to_string())?;

                // Add id
                if let Some(id) = props.get("id") {
                    write!(output, "#{}", id).map_err(|e| e.to_string())?;
                }

                // Add classes
                if let Some(classes) = props.get("class") {
                    for class in classes.split_whitespace() {
                        write!(output, ".{}", class).map_err(|e| e.to_string())?;
                    }
                }

                // Add other props
                for (key, value) in props {
                    if key != "id" && key != "class" {
                        write!(output, " {}=\"{}\"", key, value).map_err(|e| e.to_string())?;
                    }
                }

                // Add children
                if children.is_empty() {
                    writeln!(output).map_err(|e| e.to_string())?;
                } else {
                    writeln!(output).map_err(|e| e.to_string())?;
                    for child in children {
                        self.format_node(child, depth + 1, output)?;
                    }
                }
            }

            FerrumNode::Text(text) => {
                // Format text node - quote if it contains spaces
                if text.contains(' ') {
                    writeln!(output, "{}\"{}\"", indent, text).map_err(|e| e.to_string())?;
                } else {
                    writeln!(output, "{}{}", indent, text).map_err(|e| e.to_string())?;
                }
            }

            FerrumNode::Component {
                name,
                props,
                children,
            } => {
                // Format component
                write!(output, "{}{}(", indent, name).map_err(|e| e.to_string())?;

                // Add props
                let props_str: Vec<String> =
                    props.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(output, "{}", props_str.join(", ")).map_err(|e| e.to_string())?;

                if children.is_empty() {
                    writeln!(output, ")").map_err(|e| e.to_string())?;
                } else {
                    writeln!(output, ")").map_err(|e| e.to_string())?;
                    for child in children {
                        self.format_node(child, depth + 1, output)?;
                    }
                }
            }

            FerrumNode::StateBinding { signal, operation } => {
                if operation.is_empty() {
                    writeln!(output, "{}{}", indent, signal).map_err(|e| e.to_string())?;
                } else {
                    writeln!(output, "{}{}.{}", indent, signal, operation)
                        .map_err(|e| e.to_string())?;
                }
            }
            FerrumNode::Import { names, from } => {
                let names_str = names.join(", ");
                writeln!(output, "import {{ {} }} from \"{}\"", names_str, from)
                    .map_err(|e| e.to_string())?;
            }
            FerrumNode::Expression(expr) => {
                self.format_expression(expr, &indent, output)?;
            }
        }

        Ok(())
    }

    fn format_expression(
        &self,
        expr: &Expression,
        indent: &str,
        output: &mut String,
    ) -> Result<(), String> {
        match expr {
            Expression::StringLiteral(s) => {
                writeln!(output, "{}\"{}\"", indent, s).map_err(|e| e.to_string())?;
            }
            Expression::Number(n) => {
                writeln!(output, "{}{}", indent, n).map_err(|e| e.to_string())?;
            }
            Expression::SignalAccess(s) => {
                writeln!(output, "{}{}", indent, s).map_err(|e| e.to_string())?;
            }
            Expression::PropertyAccess { signal, property } => {
                writeln!(output, "{}{}.{}", indent, signal, property).map_err(|e| e.to_string())?;
            }
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let op_str = match operator {
                    BinaryOperator::Add => " + ",
                    BinaryOperator::Subtract => " - ",
                    BinaryOperator::Multiply => " * ",
                    BinaryOperator::Divide => " / ",
                    BinaryOperator::Equals => " == ",
                    BinaryOperator::NotEquals => " != ",
                    BinaryOperator::GreaterThan => " > ",
                    BinaryOperator::LessThan => " < ",
                    BinaryOperator::And => " && ",
                    BinaryOperator::Or => " || ",
                };
                writeln!(
                    output,
                    "{}{}{}{}",
                    indent,
                    self.expression_to_string(left),
                    op_str,
                    self.expression_to_string(right)
                )
                .map_err(|e| e.to_string())?;
            }
            Expression::FunctionCall { function, args } => {
                let args_str: Vec<String> = args
                    .iter()
                    .map(|arg| self.expression_to_string(arg))
                    .collect();
                writeln!(output, "{}{}({})", indent, function, args_str.join(", "))
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    fn expression_to_string(&self, expr: &Expression) -> String {
        match expr {
            Expression::StringLiteral(s) => format!("\"{}\"", s),
            Expression::Number(n) => n.to_string(),
            Expression::SignalAccess(s) => s.clone(),
            Expression::PropertyAccess { signal, property } => format!("{}.{}", signal, property),
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let op_str = match operator {
                    BinaryOperator::Add => " + ",
                    BinaryOperator::Subtract => " - ",
                    BinaryOperator::Multiply => " * ",
                    BinaryOperator::Divide => " / ",
                    BinaryOperator::Equals => " == ",
                    BinaryOperator::NotEquals => " != ",
                    BinaryOperator::GreaterThan => " > ",
                    BinaryOperator::LessThan => " < ",
                    BinaryOperator::And => " && ",
                    BinaryOperator::Or => " || ",
                };
                format!(
                    "{}{}{}",
                    self.expression_to_string(left),
                    op_str,
                    self.expression_to_string(right)
                )
            }
            Expression::FunctionCall { function, args } => {
                let args_str: Vec<String> = args
                    .iter()
                    .map(|arg| self.expression_to_string(arg))
                    .collect();
                format!("{}({})", function, args_str.join(", "))
            }
        }
    }

    fn indent_string(&self, depth: usize) -> String {
        (0..depth * self.indent_size)
            .map(|_| self.indent_char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_simple_element() {
        let input = r#"
div#app.container
    h1.title "Hello World"
    p.text-gray-600 "Welcome to Ferrum"
"#;

        let formatter = FerrumFormatter::default();
        let formatted = formatter.format(input).unwrap();

        assert!(formatted.contains("div#app.container"));
        assert!(formatted.contains("h1.title"));
        assert!(formatted.contains("p.text-gray-600"));
    }

    #[test]
    fn test_format_preserves_structure() {
        let input = r#"
div
    h1 "Title"
        p "Text"
"#;

        let formatter = FerrumFormatter::default();
        let formatted = formatter.format(input).unwrap();

        let lines: Vec<&str> = formatted.lines().collect();
        assert!(lines[0].trim().starts_with("div"));
        assert!(lines[1].trim().starts_with("h1"));
        assert!(lines[2].trim().starts_with("p"));
    }
}
