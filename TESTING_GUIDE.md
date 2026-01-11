# How to Test the Ferrum Framework

## 🚀 Quick Testing Methods

### 1. **Test CLI Commands**
```bash
cd /home/dave/ferrum

# Test project creation
cargo run --package ferrum-cli -- create my-test-app

# Test help
cargo run --package ferrum-cli -- --help

# Test version
cargo run --package ferrum-cli -- --version
```

### 2. **Test Project Generation**
```bash
# Check generated project structure
ls -la /home/dave/ferrum/my-test-app
cat /home/dave/ferrum/my-test-app/src/main.frr
cat /home/dave/ferrum/my-test-app/Cargo.toml
```

### 3. **Test .frr Parser**
```bash
# Test the parser with sample syntax
cd /home/dave/ferrum
cargo test --package ferrum-core
```

## 🧪 Run Tests

### 1. **Unit Tests**
```bash
cd /home/dave/ferrum
cargo test --package ferrum-core
cargo test --package ferrum-cli
cargo test --package ferrum-shared
```

### 2. **Integration Tests**
```bash
# Test CLI integration
cargo run --package ferrum-cli -- create integration-test
cd integration-test
cargo build  # Test if generated project compiles
```

## 📋 Testing Checklist

### ✅ CLI Tests
- [ ] `ferrum create` creates project
- [ ] `ferrum --help` shows usage
- [ ] `ferrum --version` shows version
- [ ] Templates are generated correctly
- [ ] Error handling works

### ✅ Parser Tests  
- [ ] Basic component syntax parses
- [ ] HTML-like elements parse
- [ ] State bindings recognized
- [ ] Conditional rendering parsed
- [ ] Event handlers processed

### ✅ Project Generation Tests
- [ ] Cargo.toml generated correctly
- [ ] Directory structure created
- [ ] Example .frr files work
- [ ] Component imports work
- [ ] State syntax valid

## 🔧 Manual Testing Steps

### 1. **Create and Test Project**
```bash
# Create test project
cd /home/dave/ferrum
cargo run --package ferrum-cli -- create manual-test

# Verify structure
cd /home/dave/ferrum/manual-test
tree src/

# Check files
cat src/main.frr
cat src/components/Button.frr
```

### 2. **Test .frr Syntax**
```bash
# Edit main.frr with new syntax
echo 'div.new-class.mt-4 "Updated content"' >> src/main.frr

# Try to compile (when implemented)
ferrum build
```

### 3. **Test Component System**
```bash
# Create new component
echo -e 'NewButton(variant: "success")\n    "Success Button"' > src/components/NewButton.frr

# Import and use in main.frr
echo -e 'import { NewButton } from "./components/NewButton.frr"' > src/main.frr
```

## 🐛 Common Issues to Test

### 1. **Parser Edge Cases**
- Empty .frr files
- Malformed syntax
- Deeply nested components
- Invalid props
- Missing imports

### 2. **CLI Error Handling**
- Invalid commands
- File permission errors
- Existing directory conflicts
- Invalid project names

### 3. **Generated Projects**
- Cargo compilation
- Dependency resolution
- File permissions
- Cross-platform compatibility

## 📊 Performance Testing

### 1. **CLI Performance**
```bash
# Time project creation
time cargo run --package ferrum-cli -- create perf-test

# Memory usage during parsing
/usr/bin/time -v cargo run --package ferrum-cli -- create perf-test
```

### 2. **Parser Performance**
```bash
# Test with large .frr files
cargo bench --package ferrum-core
```

## 🌐 Integration Testing

### 1. **End-to-End Workflow**
```bash
# Full workflow test
ferrum create e2e-test
cd e2e-test
# Edit src/main.frr
# Test build when available
# Test dev server when available
# Test deployment when available
```

### 2. **Cross-Platform Testing**
- Test on different OS (Linux, macOS, Windows)
- Test with different Rust versions
- Test with different shell environments

## 📝 Test Data

### Sample .frr Files for Testing
```rust
// Basic component
Button(variant: "primary", onclick: alert("clicked"))
    "Click me"

// Complex nested structure
Card(shadow: "lg")
    CardHeader
        h2 "Card Title"
    CardBody
        div.grid.grid-cols-2.gap-4
            Input(type: "text", placeholder: "Name")
            Input(type: "email", placeholder: "Email")
    CardFooter
        Button(variant: "primary") "Submit"
```

## 🎯 Success Criteria

### Minimal Viable Framework:
- ✅ CLI creates projects
- ✅ .frr files parse basic syntax  
- ✅ Generated projects are valid Rust projects
- ✅ Component system works
- ✅ State management functions

### Production Ready:
- [ ] Hot reload works
- [ ] All .frr features supported
- [ ] Performance benchmarks pass
- [ ] Documentation complete
- [ ] CI/CD pipeline works

## 🚨 Debugging Tests

### 1. **Enable Debug Output**
```bash
RUST_LOG=debug cargo run --package ferrum-cli -- create debug-test
```

### 2. **Test Individual Components**
```bash
# Test parser in isolation
cargo test --package ferrum-core -- parser::tests

# Test CLI commands
cargo test --package ferrum-cli -- cli::tests
```

### 3. **Validate Generated Code**
```bash
# Check generated Rust syntax
rustc --parse src/generated/main.rs
# Check Cargo.toml validity
cargo check --manifest-path Cargo.toml
```

This testing framework ensures Ferrum is production-ready and robust!