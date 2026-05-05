# Contributing to Stellar Oracle

First off, thank you for taking the time to contribute! It’s people like you that make **Stellar Oracle** a robust and reliable piece of infrastructure for the entire Soroban ecosystem.

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
- [Styleguides](#styleguides)
  - [Commit Messages](#commit-messages)
  - [Rust Styleguide](#rust-styleguide)
  - [TypeScript Styleguide](#typescript-styleguide)
- [Setting Up Your Development Environment](#setting-up-your-development-environment)
- [Testing](#testing)

---

## 🤝 Code of Conduct

This project and everyone participating in it is governed by the [Stellar Oracle Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## 🚀 How Can I Contribute?

### Reporting Bugs
If you find a bug, please create an issue. A great bug report includes:
- A clear, descriptive title.
- Steps to reproduce the problem.
- Observed vs. Expected behavior.
- Screenshots or logs if applicable.
- Environment details (OS, Node version, Rust version).

### Suggesting Enhancements
We love new ideas! When suggesting an enhancement:
- Check if it’s already been suggested in the [Issues](../../issues) or [Discussions](../../discussions).
- Explain **why** this enhancement would be useful to the most users.
- Provide a step-by-step description of the suggested enhancement.

### Your First Code Contribution
1. **Find an Issue**: Browse our [Issues](../../issues) and look for the `good first issue` label.
2. **Claim It**: Comment on the issue so others know you are working on it.
3. **Setup Environment**: Follow the [Setup](#setting-up-your-development-environment) instructions below.
4. **Fork and Branch**: Fork the repo and create a branch with a descriptive name (e.g., `feat/add-new-view-function` or `fix/sdk-timeout-bug`).
5. **Develop**: Write your code and accompanying tests.
6. **Lint & Format**: Ensure your code follows our [Styleguides](#styleguides).
7. **Submit PR**: Open a Pull Request against the `main` branch.

---

## 💻 Setting Up Your Development Environment

### Prerequisites
- **Rust Toolchain**: [Install Rust](https://rustup.rs/) and the Wasm target.
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **Stellar CLI**: Essential for contract deployment and interaction.
  ```bash
  cargo install --locked stellar-cli --all-features
  ```
- **Node.js**: Version 18 or higher for SDK development.

### Workspace Setup
1. **Clone and Install**:
   ```bash
   git clone https://github.com/Escelit/stellar-oracle.git
   cd stellar-oracle
   cd sdk && npm install && cd ..
   ```
2. **Build the Contract**:
   ```bash
   cargo build --release --target wasm32v1-none
   ```

---

## 🧪 Testing

We take testing seriously. No PR will be merged without accompanying tests.

### Contract Testing (Rust)
Run the core logic tests:
```bash
cargo test
```
To see output for debugging:
```bash
cargo test -- --nocapture
```

### SDK Testing (TypeScript)
Ensure the SDK and publisher bot logic are sound:
```bash
cd sdk
npm test
```

---

## 📏 Styleguides

### Commit Messages
We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. This helps us generate clear changelogs and maintain a clean history.
- `feat: ...` for new features.
- `fix: ...` for bug fixes.
- `docs: ...` for documentation changes.
- `style: ...` for formatting, missing semi-colons, etc.
- `refactor: ...` for code changes that neither fix a bug nor add a feature.

### Rust Styleguide
- Run `cargo fmt` to ensure standard formatting.
- Run `cargo clippy` to catch common mistakes and unoptimized code.
- Avoid using `panic!` in contract code where an `Error` enum variant can be returned instead.

### TypeScript Styleguide
- Use **Strict Mode** (defined in `tsconfig.json`).
- Avoid `any` types; prefer specific interfaces or generics.
- Document all public classes and methods using JSDoc.

---

## 📁 Project Structure

For a deep dive into the directory layout, please refer to the [Project Structure section of the README](README.md#-project-structure).

---

## ❓ Questions?
If you have questions, please use [GitHub Discussions](../../discussions) instead of opening an issue. This keeps the issue tracker clean for actionable tasks.
