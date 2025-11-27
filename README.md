# 🦀 Rust Practice Projects
A collection of small, focused Rust learning projects — each organized as its own crate inside this repository.

This project uses a Cargo workspace so all crates can share dependencies, tooling, and a single target/ folder in the root.

## 🚀 Running a Specific Crate
Each crate is fully independent.

### To run one:
cd leapyear_kata  
cargo run

### To test one:
cd leapyear_kata  
cargo test

## 🧠 Why a Workspace?
- Workspaces help keep multi-project repos clean:
- One shared target/ directory → faster builds, less clutter
- One shared Cargo.lock → consistent dependency versions
- Each crate stays lightweight and separate
- Easy to add new learning projects

To add a new crate:  
cargo new new_project_name  
Then add it to the workspace members in the root Cargo.toml.

## 🧹 Best Practices Followed
This repo uses clean professional Rust standards:
- Workspaces for structure and maintainability
- Each crate is isolated for clarity
- No duplicate target folders
- No unnecessary lockfiles in member crates
- Clear, consistent naming
- Simple and readable project organization
