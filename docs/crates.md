# Zaroxi Studio Crates

This document provides an overview of the Rust crates that make up the Zaroxi Studio workspace, their responsibilities, and dependencies.

## Overview

Zaroxi Studio is organized as a Cargo workspace with multiple crates, each with a specific responsibility. This modular architecture enables clear separation of concerns, independent development, and efficient compilation.

## Core Crates

### `core-types`
**Purpose**: Shared data structures and type definitions used across the entire system.
**Key Modules**:
- `ids`: Strongly-typed identifiers for documents, users, sessions, etc.
- `events`: Event definitions for system communication and state tracking.
- `commands`: Command structures for editor operations, AI tasks, and workspace actions.
- `protocol`: Communication protocol definitions for inter-component messaging.
- `workspace`: Workspace-related structures like DirectoryEntry, ListDirectoryRequest, etc.
**Dependencies**: `serde`, `uuid`

### `theme`
**Purpose**: Theming system with semantic colors and design tokens.
**Key Modules**:
- `colors`: Color representation with RGB/hex conversions.
- `theme`: Theme variants (Dark, Light, System) with semantic color definitions.
- Design tokens for spacing, typography, and other design system elements.
**Dependencies**: `serde`

## Domain Crates

### `ai-context` (in `domain/`)
**Purpose**: Gathering, organizing, and preparing context for AI operations.
**Key Modules**:
- `collector`: Collects relevant information from various sources.
- `ranking`: Scores and prioritizes collected context based on relevance.
- `packing`: Organizes and compresses context to fit model constraints.
- `prompt`: Constructs structured prompts from context and task descriptions.
**Dependencies**: `serde`, `serde_json`

### `editor-core` (in `domain/`)
**Purpose**: Core text editing functionality for the IDE.
**Key Modules**:
- `document`: Document management with rope data structure.
- `cursor`: Cursor positioning and movement.
- `selection`: Text selection ranges and operations.
- `commands`: Editor command definitions (Insert, Delete, MoveCursor, etc.).
- `events`: Editor event definitions (DocumentChanged, CursorMoved, etc.).
- `viewport`: Viewport management for visible text regions.
**Dependencies**: `ropey` (for rope data structure)

### `workspace-model` (in `domain/`)
**Purpose**: Modeling and managing workspace domain state.
**Key Modules**:
- `workspace`: Workspace domain model with UUID identification and root path management.
- `file_tree`: Hierarchical file and directory representation.
- `open_editors`: Tracking of currently open files and their state.
- `project_graph`: Dependency relationships between projects and modules.
- `snapshots`: Workspace state snapshots for undo/redo and persistence.
**Dependencies**: `serde`, `uuid`

## Language & Editor Crates

### `editor-core` (in `language/`)
**Purpose**: Core text editing functionality for the IDE.
**Key Modules**:
- `document`: Document management with rope data structure.
- `cursor`: Cursor positioning and movement.
- `selection`: Text selection ranges and operations.
- `commands`: Editor command definitions (Insert, Delete, MoveCursor, etc.).
- `events`: Editor event definitions (DocumentChanged, CursorMoved, etc.).
- `viewport`: Viewport management for visible text regions.
**Dependencies**: `ropey` (for rope data structure)

## AI & Intelligence Crates

### `ai-agent` (in `ai/`)
**Purpose**: Orchestrating AI-driven tasks and operations.
**Key Modules**:
- `planner`: Task planning with steps and execution state.
- `executor`: Task execution with AI agent integration.
- `tools`: Tool definitions with name, description, and parameters.
- `patch`: File patch generation and application.
- `verify`: Result verification for AI operations.
**Dependencies**: `serde`, `serde_json`, `anyhow`, `tokio`

## Infrastructure Crates

### `lsp-client`
**Purpose**: Language Server Protocol client implementation.
**Key Modules**:
- `manager`: Orchestrates multiple LSP sessions and lifecycle.
- `session`: Individual LSP session management.
- `transport`: Underlying communication channel handling.
- `capabilities`: Client/server capability negotiation.
- `diagnostics`: Processing and management of LSP diagnostics.
**Dependencies**: `serde`, `serde_json`, `tokio`, `anyhow`

### `patch-engine`
**Purpose**: Generating and applying changes to code and data.
**Key Modules**:
- `diff`: Generating differences between versions.
- `apply`: Applying patches with conflict detection.
- `preview`: Previewing patches before application.
**Dependencies**: `serde`, `anyhow`

### `preview-engine`
**Purpose**: Device‑aware simulation and preview of AI‑generated experiences.
**Key Modules**:
- `simulation`: Core simulation loop and state management.
- `device_models`: Device profiles (screen sizes, input capabilities).
- `web_server`: Lightweight HTTP server to serve preview content.
- `viewport`: Viewport rendering and interaction handling.
**Dependencies**: `serde`, `tokio`, `warp`, `wry` (or `tauri`)

### `rpc`
**Purpose**: Remote Procedure Call framework for inter-process communication.
**Key Modules**:
- `client`: RPC client implementation for outgoing requests.
- `server`: RPC server implementation for incoming requests.
- `framing`: Message framing over byte streams.
- `messages`: RPC message structure definitions.
**Dependencies**: `serde`, `serde_json`, `tokio`, `anyhow`

## Configuration & Security

### `settings`
**Purpose**: Configuration management for user and system settings.
**Key Modules**:
- `model`: Settings structure and validation rules.
- `loader`: Loading settings from various sources.
**Dependencies**: `serde`, `serde_json`, `anyhow`

### `permissions`
**Purpose**: Access control and security policies.
**Key Modules**:
- `policy`: Security policy and rule definitions.
- `grants`: Permission granting and revocation.
**Dependencies**: `serde`, `uuid`

## Services

### `workspace-daemon` (in `services/`)
**Purpose**: Background service for workspace management.
**Responsibilities**: File operations, Git integration, terminal sessions, background tasks.
**Key Modules**: Workspace management, file watching, indexing.
**Dependencies**: `tokio`, `tracing`, `anyhow`

### `ai-daemon` (in `services/`)
**Purpose**: Background service for AI operations.
**Responsibilities**: Provider routing, streaming responses, quota management, request routing.
**Key Modules**: AI task processing, provider management, quota tracking.
**Dependencies**: `tokio`, `tracing`, `anyhow`, `serde`, `serde_json`

## Applications

### `desktop` (in `apps/`)
**Purpose**: Main desktop application built with Tauri + React.
**Key Features**: Workspace explorer, code editor, AI assistant, command palette, keyboard shortcuts.
**Frontend**: React with TypeScript, Tailwind CSS
**Backend**: Tauri with Rust commands for file operations, AI tasks, etc.
**Dependencies**: `tauri`, `tokio`, `tracing`, `serde`, `serde_json`

## Development Guidelines

### Adding a New Crate
1. Create directory under `crates/` with appropriate name
2. Add `Cargo.toml` with workspace inheritance
3. Add minimal dependencies (prefer workspace dependencies)
4. Create `src/lib.rs` with module declarations
5. Add crate to workspace members in root `Cargo.toml`

### Dependency Management
- Use workspace dependencies when possible: `serde = { workspace = true }`
- Keep dependencies minimal and focused
- Document the purpose of each dependency
- Regular security audits via `cargo audit`

### Testing Strategy
- Unit tests within each crate
- Integration tests in `tests/` directory
- Property-based testing for core algorithms
- Mock external services for reliable tests

## Build & Development

```bash
# Build all crates
cargo build --workspace

# Test all crates
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings
```

This crate architecture provides a solid foundation for Zaroxi Studio's development while maintaining flexibility for future growth and specialization.
