# 🐾 Ratel CLI (v0.5.4)
The Cyberattack-Driven Development (CDD) Orchestrator. Because security isn't a feature, it's a foundation.

Ratel is the official command-line tool (CLI) for the CDD methodology. It allows developers to inject, verify, and certify expert-level security tests directly into their projects (Java, Node.js, Python), requiring zero prior cybersecurity knowledge.

## Quick Start
Download the binary for your operating system from the Releases page or install it globally:

```Bash

# Via Cargo (for Rust developers)
cargo install ratel-cli

# Verify installation
ratel --version

# Installation using install.sh
curl -fsSL https://raw.githubusercontent.com/cdd-framework/ratel-cli/main/install.sh | sh
```

## Fundamental Commands
1. ratel init
Initializes the CDD ecosystem in your project. Ratel automatically detects your environment (Maven/Spring Boot, Node.js, etc.) and injects an expert test suite into a dedicated directory.

```Bash
ratel init --context banking
```

Action: Creates src/test/java/.../RatelSecurityTest.java and a ratel.yaml traceability file.

Why?: To start with a robust security foundation without spending time writing complex security tests.

2. ratel check
Verifies the integrity of your security foundations. This command compares the current state of your tests with the original expert digital fingerprints (SHA-256 hashes).

```Bash
ratel check
```

Green (✅): Your tests comply with the original expert standards.

Red (❌): Modifications (tweaks) detected. Traceability has been broken.

3. ratel certify
Modified a test to fit a specific business need? Use certify to sign your changes and establish a new certified baseline.

```Bash
ratel certify
```

Action: Updates the hashes in ratel.yaml and adds a customized_at timestamp.

## Traceability (ratel.yaml)
Every Ratel project is accompanied by a ratel.yaml file. This file acts as the guarantor of your security posture for auditors and production teams:

```YAML
version: "0.4.0-alpha.2"
project_type: "Java"
context: "banking"
initialized_at: 2025-12-30T17:45:00Z
customized_at: null # Becomes a timestamp after running 'ratel certify'
expert_hashes:
  "src/test/java/.../RatelSecurityTest.java": "a1b2c3d4e5f6..."
```

## CDD Philosophy (Cyberattack-Driven Development)
Unlike traditional TDD, CDD doesn't test if your code works, but whether it resists. Ratel organizes tests into two critical scopes:

- KERNEL: Protection of the core. (HTTP Headers, encryption, CORS policies).
- TERRITORY: Reduction of the exposure surface. (Sensitive files, configuration leaks).

## Best Practices: Enterprise & Banking Integration
To maintain a maximum security posture, follow these expert recommendations when integrating Ratel into your continuous deployment pipelines.

### 1. CI/CD Gatekeeping
Don't just run the tests. Use ratel check as an impenetrable security gate in your pipeline.

- Build Stage: Run ratel check immediately after compilation.
- Failure Policy: If ratel check detects an uncertified modification, the pipeline must fail (Exit Code != 0).
- GitLab CI Example:


```YAML

security_audit:
  stage: test
  script:
    - ratel check
  allow_failure: false # Mandatory for compliance
```

### 2. Certification Management (ratel certify)
In a banking environment, certifying a "tweak" must not be an isolated act.

- Peer-Review: Any test modification requiring ratel certify must be handled through a dedicated Pull Request.

- Justification: The certification commit should include a link to a ticket (Jira/GitHub) explaining why the expert standard was modified.

- Audit Trail: Ratel logs the modification date in ratel.yaml. Use this timestamp for your quarterly audit reports.

## Contribute
Join our 8 early-adopter followers and help us lay the cornerstone of the CDD standard.