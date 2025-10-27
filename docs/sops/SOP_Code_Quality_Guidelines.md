# SOP: Code Quality Guidelines

## 1. Introduction

This Standard Operating Procedure (SOP) outlines the essential code quality guidelines for all development activities within the project. Adhering to these guidelines ensures maintainable, reliable, and high-performing code.

## 2. General Principles

*   **Readability:** Code should be easy to understand for anyone familiar with the language.
*   **Maintainability:** Code should be easy to modify and extend.
*   **Testability:** Code should be designed to be easily testable.
*   **Performance:** Code should be efficient in its use of resources.
*   **Security:** Code should be free from common vulnerabilities.

## 3. Language-Specific Guidelines

### Rust

*   Follow official Rust style guidelines (`rustfmt`).
*   Utilize Clippy for linting and address all warnings.
*   Write comprehensive unit and integration tests.
*   Ensure proper error handling using Rust's Result and Option types.

### Shell Scripts

*   **Mandatory Shellcheck:** All shell scripts **must** be run through `shellcheck` after any modifications. Refer to `docs/memos/Shellcheck_Always_After_Changes.md` for detailed instructions and policy.
*   Use clear variable names and comments.
*   Avoid unnecessary complexity.
*   Ensure scripts are idempotent where appropriate.

### Python

*   Adhere to PEP 8 style guide.
*   Use a linter (e.g., `flake8`, `pylint`) and address warnings.
*   Write docstrings for modules, classes, and functions.
*   Implement unit tests for all critical logic.

## 4. Version Control Best Practices

*   **Meaningful Commit Messages:** Write clear, concise, and descriptive commit messages. Refer to project-specific guidelines for commit message format.
*   **Small, Focused Commits:** Each commit should ideally address a single logical change.
*   **Branching Strategy:** Follow the project's defined branching strategy (e.g., Git Flow, GitHub Flow).

## 5. Review Process

All code changes must undergo a peer review process. Reviewers will ensure adherence to these code quality guidelines, functional correctness, and architectural consistency.

## 6. Tools and Automation

Leverage automated tools (linters, formatters, static analyzers) as part of your development workflow. These tools are integrated into our Nix development environment and CI/CD pipelines to enforce quality standards automatically.
