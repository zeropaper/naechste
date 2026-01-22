use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn create_temp_project(name: &str) -> std::path::PathBuf {
    let temp_dir = std::env::temp_dir().join(format!("naechste-integration-{}", name));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).ok();
    }
    fs::create_dir_all(&temp_dir).unwrap();
    temp_dir
}

fn create_file(base: &Path, path: &str, content: &str) {
    let file_path = base.join(path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("naechste"));
    assert!(stdout.contains("Next.js file-structure"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_cli_clean_project_exits_zero() {
    let project_dir = create_temp_project("clean");
    
    create_file(&project_dir, "app/page.tsx", "export default function Page() {}");
    create_file(&project_dir, "app/layout.tsx", "export default function Layout() {}");

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_error_exits_one() {
    let project_dir = create_temp_project("error");
    
    create_file(
        &project_dir,
        "app/MyComponent.tsx",
        "'use client'\nexport async function getServerSideProps() {}"
    );

    create_file(
        &project_dir,
        ".naechste.config.json",
        r#"{"rules":{"server_side_exports":{"severity":"error"}}}"#
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_json_output() {
    let project_dir = create_temp_project("json");
    
    create_file(
        &project_dir,
        "app/MyComponent.tsx",
        "'use client'\nexport const getServerSideProps = () => {}"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"diagnostics\""));
    assert!(stdout.contains("\"severity\""));
    assert!(stdout.contains("\"rule\""));
    assert!(stdout.contains("\"message\""));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_human_output() {
    let project_dir = create_temp_project("human");
    
    create_file(
        &project_dir,
        "app/BadName.tsx",
        "export function Component() {}"
    );

    create_file(
        &project_dir,
        ".naechste.config.json",
        r#"{"rules":{"filename_style_consistency":{"severity":"warn","options":{"filename_style":"kebab-case"}}}}"#
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--format")
        .arg("human")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("warn") || stdout.contains("error"));
    assert!(stdout.contains("filename-style-consistency") || stdout.contains("BadName"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_custom_config_file() {
    let project_dir = create_temp_project("custom-config");
    
    create_file(&project_dir, "app/Component.tsx", "export function Component() {}");
    create_file(
        &project_dir,
        "custom.json",
        r#"{"rules":{"filename_style_consistency":{"severity":"error","options":{"filename_style":"kebab-case"}}}}"#
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--config")
        .arg(project_dir.join("custom.json"))
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_ignores_node_modules() {
    let project_dir = create_temp_project("node-modules");
    
    create_file(
        &project_dir,
        "node_modules/package/index.tsx",
        "'use client'\nexport const getServerSideProps = () => {}"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_multiple_errors() {
    let project_dir = create_temp_project("multiple");
    
    create_file(
        &project_dir,
        "app/BadName1.tsx",
        "'use client'\nexport const getServerSideProps = () => {}"
    );
    create_file(
        &project_dir,
        "app/BadName2.tsx",
        "'use client'\nexport const getStaticProps = () => {}"
    );

    create_file(
        &project_dir,
        ".naechste.config.json",
        r#"{"rules":{"server_side_exports":{"severity":"error"}}}"#
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse JSON and verify multiple diagnostics
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let diagnostics = json["diagnostics"].as_array().unwrap();
    assert!(diagnostics.len() >= 2);

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_nesting_depth() {
    let project_dir = create_temp_project("nesting");
    
    create_file(
        &project_dir,
        "app/a/b/c/d/deep.tsx",
        "export function Deep() {}"
    );

    create_file(
        &project_dir,
        ".naechste.config.json",
        r#"{"rules":{"component_nesting_depth":{"severity":"error","options":{"max_nesting_depth":3}}}}"#
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_companion_files() {
    let project_dir = create_temp_project("companion");
    
    create_file(&project_dir, "app/Button.tsx", "export function Button() {}");

    create_file(
        &project_dir,
        ".naechste.config.json",
        r#"{"rules":{"missing_companion_files":{"severity":"error","options":{"require_test_files":true}}}}"#
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    // Now add the test file
    create_file(&project_dir, "app/Button.test.tsx", "test('Button', () => {})");

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(0));

    fs::remove_dir_all(project_dir).ok();
}
