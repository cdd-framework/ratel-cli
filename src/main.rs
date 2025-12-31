use chrono::{DateTime, Utc};
use clap::{Parser as ClapParser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use pest::Parser as PestParser;
use pest_derive::Parser as PestDeriveParser;

extern crate cdd_core; 

// Parser Pest definition
#[derive(PestDeriveParser)]
#[grammar = "ratel.pest"] 
pub struct RatelParser;

#[derive(Serialize, Deserialize, Debug)]
pub struct RatelConfig {
    pub version: String,
    pub project_type: String,
    pub context: String,
    pub initialized_at: DateTime<Utc>,
    pub customized_at: Option<DateTime<Utc>>,
    pub expert_hashes: HashMap<String, String>,
}

// Structures for the audit report consolidated by ratel-cli
#[derive(Serialize, Clone)]
struct ActionResult {
    kind: String,
    value: String,
    target: Option<String>,
    status: String, // "SUCCESS", "FAILED", "ERROR"
    message: String,
}

#[derive(Serialize)]
struct StepResult {
    title: String,
    results: Vec<ActionResult>,
}

#[derive(Serialize)]
struct AuditReport {
    name: String,
    target: String,
    scope: String,
    steps: Vec<StepResult>,
    executed_at: DateTime<Utc>,
}

#[derive(ClapParser)]
#[command(name = "ratel", version, about = "Ratel CLI: Cyberattack-Driven Development Framework")]
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
    // Executes the full audit (Parsing -> cdd-core -> JSON Report)
    Run { path: String },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { context } => {
            println!("🐾 Ratel is sniffing the project structure...");
            let mut hashes = HashMap::new();

            let (p_type, path, content) = if Path::new("package.json").exists() {
                ("Node.js", "tests/ratel/security.ratel".to_string(), generate_default_scenario())
            } else {
                ("Generic", "security.ratel".to_string(), generate_default_scenario())
            };

            setup_security_file(&path, &content);
            hashes.insert(path, calculate_hash(&content));
            save_ratel_config(p_type, context, hashes);
        },
        Commands::Check => { check_integrity(); },
        Commands::Certify => { certify_modifications(); },
        Commands::Run { path } => { execute_full_audit(path); },
    }
}

fn generate_default_scenario() -> String {
    r#"SCENARIO "Access audit"
TARGET "http://localhost:8080"
WITH_SCOPE KERNEL

STEP "Secure transport verification"
    ATTACK secure_headers
    CHECK header "Strict-Transport-Security" EXISTS
    CHECK response.status BE 200"#
        .to_string()
}

// Pivot function: Parses the DSL and calls cdd-core for each action
fn execute_full_audit(path: &str) {
    // 1. Preliminary integrity check
    check_integrity();

    let content = fs::read_to_string(path).expect("Unable to read .ratel file");
    
    // 2. Safe Parsing: avoid thread panic
    let file_parse_result = RatelParser::parse(Rule::file, &content);
    
    let file = match file_parse_result {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            let error_report = serde_json::json!({
                "status": "error",
                "error_type": "PARSING_ERROR",
                "message": format!("Syntax error in .ratel file: {}", e)
            });
            println!("{}", serde_json::to_string_pretty(&error_report).unwrap());
            std::process::exit(1);
        }
    };

    let mut report = AuditReport {
        name: String::new(),
        target: String::new(),
        scope: String::new(),
        steps: Vec::new(),
        executed_at: Utc::now(),
    };

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::scenario => report.name = record.into_inner().as_str().replace("\"", ""),
            Rule::target => report.target = record.into_inner().as_str().replace("\"", ""),
            Rule::with_scope => report.scope = record.into_inner().as_str().to_string(),
            Rule::step => {
                let mut inner = record.into_inner();
                let title = inner.next().unwrap().as_str().replace("\"", "");
                let mut action_results = Vec::new();

                for cmd in inner {
                    // 3. Synchronizing payloads with cdd-core execution
                    let result = match cmd.as_rule() {
                        Rule::attack => {
                            let attack_val = cmd.into_inner().as_str();
                            // Call to external cdd-core library
                            let core_res = cdd_core::execute_attack("attack", attack_val);
                            
                            ActionResult {
                                kind: "ATTACK".into(),
                                value: attack_val.into(),
                                target: None,
                                status: if core_res.success { "SUCCESS".into() } else { "FAILED".into() },
                                message: core_res.message,
                            }
                        },
                        Rule::check => {
                            let check_val = cmd.as_str();
                            // Call to external cdd-core library
                            let core_res = cdd_core::verify_condition(check_val);

                            ActionResult {
                                kind: "CHECK".into(),
                                value: check_val.into(),
                                target: None,
                                status: if core_res.success { "SUCCESS".into() } else { "FAILED".into() },
                                message: core_res.message,
                            }
                        },
                        _ => continue,
                    };
                    action_results.push(result);
                }
                report.steps.push(StepResult { title, results: action_results });
            }
            _ => {}
        }
    }

    // Sends the final JSON report back to cdd-node
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn setup_security_file(path: &str, content: &str) {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).expect("Failed to write .ratel file");
    println!("✅ Expert scenario injected into {}.", path);
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
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write("ratel.yaml", yaml).unwrap();
}

fn check_integrity() {
    let config_content = fs::read_to_string("ratel.yaml").expect("Run init first.");
    let config: RatelConfig = serde_yaml::from_str(&config_content).unwrap();
    for (path, original_hash) in config.expert_hashes {
        let current_content = fs::read_to_string(&path).expect("File missing");
        if calculate_hash(&current_content) != original_hash {
            panic!("❌ ALERT: '{}' modified! Audit aborted.", path);
        }
    }
}

fn certify_modifications() {
    let config_content = fs::read_to_string("ratel.yaml").expect("No ratel.yaml found.");
    let mut config: RatelConfig = serde_yaml::from_str(&config_content).unwrap();
    let mut new_hashes = HashMap::new();
    for (path, _) in &config.expert_hashes {
        if let Ok(content) = fs::read_to_string(path) {
            new_hashes.insert(path.clone(), calculate_hash(&content));
        }
    }
    config.expert_hashes = new_hashes;
    config.customized_at = Some(Utc::now());
    fs::write("ratel.yaml", serde_yaml::to_string(&config).unwrap()).unwrap();
    println!("New baseline established.");
}