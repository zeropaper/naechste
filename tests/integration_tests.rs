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

    create_file(
        &project_dir,
        "app/page.tsx",
        "export default function Page() {}",
    );
    create_file(
        &project_dir,
        "app/layout.tsx",
        "export default function Layout() {}",
    );

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
        "'use client'\nexport async function getServerSideProps() {}",
    );

    create_file(
        &project_dir,
        "naechste.json",
        r#"{"rules":{"server_side_exports":{"severity":"error"}}}"#,
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
fn test_cli_yaml_config_detected_by_default() {
    let project_dir = create_temp_project("yaml-default");

    create_file(
        &project_dir,
        "app/MyComponent.tsx",
        "'use client'\nexport async function getServerSideProps() {}",
    );

    create_file(
        &project_dir,
        "naechste.yaml",
        r#"
rules:
  server_side_exports:
    severity: error
"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_cli_json_output() {
    let project_dir = create_temp_project("json");

    create_file(
        &project_dir,
        "app/MyComponent.tsx",
        "'use client'\nexport const getServerSideProps = () => {}",
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
        "export function Component() {}",
    );

    create_file(
        &project_dir,
        "naechste.json",
        r#"{"rules":{"filename_style_consistency":{"severity":"warn","options":{"filename_style":"kebab-case"}}}}"#,
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

    create_file(
        &project_dir,
        "app/Component.tsx",
        "export function Component() {}",
    );
    create_file(
        &project_dir,
        "custom.json",
        r#"{"rules":{"filename_style_consistency":{"severity":"error","options":{"filename_style":"kebab-case"}}}}"#,
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
        "'use client'\nexport const getServerSideProps = () => {}",
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
        "'use client'\nexport const getServerSideProps = () => {}",
    );
    create_file(
        &project_dir,
        "app/BadName2.tsx",
        "'use client'\nexport const getStaticProps = () => {}",
    );

    create_file(
        &project_dir,
        "naechste.json",
        r#"{"rules":{"server_side_exports":{"severity":"error"}}}"#,
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
        "export function Deep() {}",
    );

    create_file(
        &project_dir,
        "naechste.json",
        r#"{"rules":{"component_nesting_depth":{"severity":"error","options":{"max_nesting_depth":3}}}}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(1));

    fs::remove_dir_all(project_dir).ok();
}

// ==================== BASSIST PRESET TESTS ====================

#[test]
fn test_bassist_preset_cli_flag() {
    let project_dir = create_temp_project("bassist-cli");

    // Create route group without [locale] directory (violation)
    create_file(
        &project_dir,
        "app/(admin)/page.tsx",
        "export default function Page() {}",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect domain structure violation
    assert!(stdout.contains("bassist-domain-structure") || stderr.contains("bassist-domain-structure"));
    assert_eq!(output.status.code(), Some(1)); // Error severity

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_preset_config_file() {
    let project_dir = create_temp_project("bassist-config");

    create_file(
        &project_dir,
        "naechste.json",
        r#"{"preset":"bassist"}"#,
    );

    // Create route group without [locale] directory
    create_file(
        &project_dir,
        "app/(auth)/page.tsx",
        "export default function Page() {}",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect domain structure violation
    assert!(stdout.contains("bassist-domain-structure"));
    assert_eq!(output.status.code(), Some(1));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_locale_nesting() {
    let project_dir = create_temp_project("bassist-locale-nesting");

    // Create page.tsx outside [locale] directory
    create_file(
        &project_dir,
        "app/(admin)/page.tsx",
        "export default function Page() {}",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect locale nesting violation
    assert!(stdout.contains("bassist-locale-nesting"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_service_client_restriction() {
    let project_dir = create_temp_project("bassist-service-client");

    // Service client in production code (violation)
    create_file(
        &project_dir,
        "app/lib/profiles.ts",
        r#"import { createTestServiceClient } from '@/tests/utils';
export function getProfiles() {
    const supabase = createTestServiceClient();
    return supabase.from('profiles').select();
}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect service client in production
    assert!(stdout.contains("bassist-service-client-restriction"));
    assert_eq!(output.status.code(), Some(1)); // Error severity

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_supabase_client_imports() {
    let project_dir = create_temp_project("bassist-supabase-imports");

    // Client component importing server client (violation)
    create_file(
        &project_dir,
        "app/components/Form.tsx",
        r#"'use client'
import { createClient } from '@/lib/supabase/server';
export function Form() {
    return <div>Form</div>;
}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect wrong client import
    assert!(stdout.contains("bassist-supabase-client-imports"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_i18n_hook_usage() {
    let project_dir = create_temp_project("bassist-i18n-hooks");

    // Client component using server function (violation)
    create_file(
        &project_dir,
        "app/components/Button.tsx",
        r#"'use client'
import { getExtracted } from '@/i18n/server';
export function Button() {
    const t = getExtracted('common.actions');
    return <button>{t('Click me')}</button>;
}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect wrong i18n hook
    assert!(stdout.contains("bassist-i18n-hook-usage"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_test_colocation() {
    let project_dir = create_temp_project("bassist-test-colocation");

    // Test file in root tests directory (violation)
    create_file(
        &project_dir,
        "tests/admin.test.ts",
        "import { describe, it } from 'vitest';\ndescribe('Admin', () => {});",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect test colocation issue
    assert!(stdout.contains("bassist-test-colocation"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_domain_isolation() {
    let project_dir = create_temp_project("bassist-domain-isolation");

    // Cross-domain import from lib (violation)
    create_file(
        &project_dir,
        "app/(auth)/lib/profiles.ts",
        r#"import { getUsers } from '@/app/(admin)/lib/users';
export function getProfiles() {
    return getUsers();
}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect cross-domain import
    assert!(stdout.contains("bassist-domain-isolation"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_i18n_namespaces() {
    let project_dir = create_temp_project("bassist-i18n-namespaces");

    // i18n key without namespace (violation)
    create_file(
        &project_dir,
        "app/components/Header.tsx",
        r#"'use client'
import { useExtracted } from '@/i18n/client';
export function Header() {
    const t = useExtracted('actions');
    return <div>{t('Click')}</div>;
}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should detect missing namespace
    assert!(stdout.contains("bassist-i18n-namespaces"));

    fs::remove_dir_all(project_dir).ok();
}

#[test]
fn test_bassist_valid_structure() {
    let project_dir = create_temp_project("bassist-valid");

    // Create correct structure
    create_file(
        &project_dir,
        "app/(admin)/[locale]/layout.tsx",
        r#"import { getExtracted } from '@/i18n/server';
export default async function Layout({ children }) {
    const t = await getExtracted('admin.layout');
    return <div>{children}</div>;
}"#,
    );

    create_file(
        &project_dir,
        "app/(admin)/[locale]/page.tsx",
        r#"import { createClient } from '@/lib/supabase/server';
export default async function Page() {
    const supabase = await createClient();
    return <div>Admin</div>;
}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_naechste"))
        .arg(&project_dir)
        .arg("--preset")
        .arg("bassist")
        .output()
        .expect("Failed to execute command");

    let _stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should pass with no Bassist-specific errors
    // (might have warnings about unknown route group if not configured)
    assert_eq!(output.status.code(), Some(0));

    fs::remove_dir_all(project_dir).ok();
}

