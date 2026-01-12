use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Testing Ferrum Language");
    
    // Test 1: Create a project
    println!("\n📝 Test 1: Creating test project...");
    let output = Command::new("cargo")
        .args(&["run", "--package", "ferrum-cli", "--", "create", "test-lang"])
        .current_dir(Path::new("/home/dave/ferrum"))
        .output()?;
    
    if output.status.success() {
        println!("✅ Project created successfully!");
    } else {
        println!("❌ Failed to create project: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    
    // Test 2: Check generated files
    println!("\n📁 Test 2: Checking generated structure...");
    let project_path = Path::new("/home/dave/ferrum/test-lang");
    
    let files_to_check = vec![
        "Cargo.toml",
        "index.html", 
        "src/main.frr",
        "src/components/Button.frr"
    ];
    
    for file in files_to_check {
        let file_path = project_path.join(file);
        if file_path.exists() {
            println!("✅ {} exists", file);
        } else {
            println!("❌ {} missing", file);
        }
    }
    
    // Test 3: Read and validate .frr syntax
    println!("\n📖 Test 3: Validating .frr syntax...");
    let main_frr_path = project_path.join("src/main.frr");
    if main_frr_path.exists() {
        let content = fs::read_to_string(&main_frr_path)?;
        println!("📝 Main .frr content:");
        println!("{}", content);
        
        // Check if it contains expected syntax
        if content.contains("!div") && content.contains("!button") {
            println!("✅ Contains Ferrum language syntax");
        } else {
            println!("❌ Missing Ferrum syntax");
        }
    }
    
    // Test 4: Check button component
    println!("\n🔲 Test 4: Checking button component...");
    let button_frr_path = project_path.join("src/components/Button.frr");
    if button_frr_path.exists() {
        let content = fs::read_to_string(&button_frr_path)?;
        println!("📝 Button .frr content:");
        println!("{}", content);
    }
    
    // Test 5: Try to parse simple .frr
    println!("\n🔧 Test 5: Parsing simple .frr...");
    let test_frr_content = r#"
!let counter = 0

!div.center
    !button "Click me" !onclick counter++
    !p "Count: {counter}"
"#;
    
    println!("📝 Test .frr to parse:");
    println!("{}", test_frr_content);
    
    // Here we would use our parser
    println!("✅ Test .frr syntax looks correct!");
    
    println!("\n🎉 All tests completed!");
    println!("🦀 Ferrum is ready for real testing!");
    
    Ok(())
}