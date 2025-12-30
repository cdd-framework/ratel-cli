use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path; // used for numeric fingerprinting calculation

#[derive(Serialize, Deserialize, Debug)]
pub struct RatelConfig {
    pub version: String,
    pub project_type: String,
    pub context: String,
    pub initialized_at: DateTime<Utc>,
    pub customized_at: Option<DateTime<Utc>>, // Nouveau champ
    pub expert_hashes: HashMap<String, String>,
}

#[derive(Parser)]
#[command(
    name = "ratel",
    version,
    about = "Ratel CLI: Cyberattack-Driven Development Framework"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, default_value = "generic")]
        context: String,
    },
    Check,
    Certify,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { context } => {
            println!("🐾 Ratel is sniffing the project structure...");
            let mut hashes = HashMap::new();

            if Path::new("pom.xml").exists() {
                println!("Java project detected");
                let (path, content) = setup_java_project(context);
                hashes.insert(path, calculate_hash(&content));
                save_ratel_config("Java", context, hashes);
            } else if Path::new("package.json").exists() {
                println!("Node.js project detected");
                let (path, content) = setup_node_project(context);
                hashes.insert(path, calculate_hash(&content));
                save_ratel_config("Node.js", context, hashes);
            } else {
                println!("No supported project structure found.");
            }
        },
        Commands::Check => {
            println!("Ratel is checking expert tests integrity...");
            check_integrity();
        },
        Commands::Certify => { certify_modifications(); },
    }
}

fn check_integrity() {
    // 1. Load the yaml configuration
    let config_content = fs::read_to_string("ratel.yaml")
        .expect("No ratel.yaml found. Please run 'ratel init' first.");

    let config: RatelConfig = serde_yaml::from_str(&config_content).unwrap();
    let mut modified_count = 0;

    // 2. Compare hashes
    for (path, original_hash) in config.expert_hashes {
        if let Ok(current_content) = fs::read_to_string(&path) {
            let current_hash = calculate_hash(&current_content);

            if current_hash != original_hash {
                println!("ALERT: File '{}' has been modified (Tweak detected).", path);
                modified_count += 1;
            } else {
                println!("'{}' is authentic (Expert version).", path);
            }
        } else {
            println!("WARNING: File '{}' is missing!", path);
        }
    }

    if modified_count == 0 {
        println!("\nYour CDD foundation is intact. All tests are certified 'Expert'.");
    } else {
        println!(
            "\nFound {} modification(s). Your security gate is no longer original.",
            modified_count
        );
    }
}

fn certify_modifications() {
    let config_content = fs::read_to_string("ratel.yaml").expect("No ratel.yaml found.");

    let mut config: RatelConfig = serde_yaml::from_str(&config_content).unwrap();
    let mut new_hashes = HashMap::new();

    println!("Certifying new baseline...");

    for (path, _) in &config.expert_hashes {
        if let Ok(content) = fs::read_to_string(path) {
            new_hashes.insert(path.clone(), calculate_hash(&content));
            println!("Certified: {}", path);
        }
    }

    config.expert_hashes = new_hashes;
    config.customized_at = Some(Utc::now()); // On marque la date de signature

    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write("ratel.yaml", yaml).unwrap();

    println!("\nNew baseline established. These tests are now your official certified version.");
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn save_ratel_config(p_type: &str, context: &str, hashes: HashMap<String, String>) {
    let config = RatelConfig {
        version: env!("CARGO_PKG_VERSION").to_string(),
        project_type: p_type.to_string(),
        context: context.to_string(),
        initialized_at: Utc::now(),
        customized_at: None,
        expert_hashes: hashes,
    };
    let yaml = serde_yaml::to_string(&config).expect("Failed to serialize config");
    fs::write("ratel.yaml", yaml).expect("Failed to write ratel.yaml");
    println!("Traceability file 'ratel.yaml' created with expert hashes.");
}

fn setup_java_project(_context: &str) -> (String, String) {
    let dir = "src/test/java/io/github/cddframework/ratel";
    let file_path = format!("{}/RatelSecurityTest.java", dir);
    fs::create_dir_all(dir).unwrap();

    let content = r#"
package io.github.cddframework.ratel;
import io.github.cddframework.Ratel;
import io.github.cddframework.ScanScope;
import org.junit.jupiter.api.Test;

class RatelSecurityTest {
    @Test
    void security_gate_kernel() {
        Ratel.scan("http://localhost:8080").withScope(ScanScope.KERNEL).run();
    }
}"#;
    fs::write(&file_path, content.trim()).unwrap();
    println!("Expert tests injected into {}.", file_path);
    (file_path, content.trim().to_string())
}

fn setup_node_project(_context: &str) -> (String, String) {
    let dir = "tests/ratel";
    let file_path = format!("{}/ratelSecurity.test.js", dir);
    fs::create_dir_all(dir).unwrap();

    let content = r#"
const { Ratel, ScanScope } = require('cdd-ratel');
test('Ratel Security Test', async () => {
    await Ratel.scan('http://localhost:8080').withScope(ScanScope.KERNEL).run();
});"#;
    fs::write(&file_path, content.trim()).unwrap();
    println!("✅ Expert tests injected into {}.", file_path);
    (file_path, content.trim().to_string())
}
