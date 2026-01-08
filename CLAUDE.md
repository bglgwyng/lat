# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`lat` is a CLI tool that displays file contents in an LLM-friendly way. It dispatches to specialized handlers based on file patterns configured via `.lat.kdl`. Think of it as `cat` but with configurable per-filetype processing.

## Build Commands

```bash
cargo build           # Build the project
cargo run -- <FILE>   # Run with arguments
cargo test            # Run all tests
cargo test <test_name> # Run a specific test
```

Nix users can use `nix develop` for the dev shell or `nix build` to build the package.

## Architecture

The codebase has two main modules:

- **main.rs**: CLI argument parsing (using clap) and `RuleRunner` which handles variable substitution (`$FILE`, `$UPTO`, `$FOCUS`) and command execution
- **config.rs**: KDL config parsing, `Rule` and `Config` structs, glob matching, and config file discovery (searches up directory tree for `.lat.kdl`)

Flow: CLI args → load config → find matching rule by filename glob → substitute variables in rule args → execute external command

## Configuration Format

Rules are defined in `.lat.kdl` using KDL syntax. Each rule has patterns, a command, args with variable substitution, and optional defaults. Rules are matched in order; first match wins.
