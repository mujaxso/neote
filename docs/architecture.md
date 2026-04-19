# Zaroxi Studio Architecture

Zaroxi Studio is an AI-first IDE built as a modular Rust workspace. This document outlines the high-level architecture and component relationships.

## Overview

Zaroxi Studio follows a client-server architecture with a Tauri desktop frontend, multiple background services, and a collection of shared libraries (crates). The system is designed to be extensible, with clear separation between UI, business logic, and AI capabilities.

## Workspace Structure

### Applications (`apps/`)
- **desktop**: The main desktop application built with Tauri + React. Contains UI modules for editor, workspace explorer, AI assistant, and layout management.

### Services (`services/`)
- **workspace-daemon**: Manages workspace state, file system operations, Git integration, terminal sessions, and background tasks.
- **ai-daemon**: Orchestrates AI operations including provider routing, streaming responses, quota management, and request routing.

### Core Crates (`crates/`)
- **core-types**: Shared data structures for IDs, events, commands, and protocol definitions.
- **theme**: Theming system with semantic colors, design tokens, and dark/light mode support.

### Domain (`domain/`)
- **workspace-model**: Models for workspace state with UUID identification and root path management.

### Language (`language/`)
- **editor-core**: Text editing primitives with rope data structure, cursor management, commands, and events.

### AI (`ai/`)
- **ai-agent**: AI agent planning, execution, tool usage, patch generation, and verification.
- **ai-context**: AI context collection, ranking, packing, and prompt construction.

## Frontend Platform Strategy

Zaroxi Studio adopts a modern desktop application architecture built with Tauri, providing a secure, high-performance foundation with web technologies for the UI and Rust for the backend.

**Key decisions:**

1. **Tauri-based desktop** – The main application is built with Tauri, using React for the UI and Rust for the backend, ensuring cross-platform compatibility and security.
2. **Performance** – Critical operations are handled in Rust, providing native performance for text editing, AI processing, and file operations.
3. **Communication** – Frontend and backend communicate via Tauri commands and events, with additional RPC for service communication.
4. **Separation of concerns** – UI-specific code is kept in the frontend, while business logic resides in Rust crates and services.

This architecture provides a responsive, modern IDE experience while leveraging the safety and performance of Rust for core operations.

## Component Relationships

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Desktop App   │◄──►│  Workspace Model │◄──►│  Editor Buffer  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   UI Modules    │    │   LSP Client     │    │   Patch Engine  │
│  (layout, chat, │    │                  │    │                 │
│  editor, etc.)  │    └──────────────────┘    └─────────────────┘
└─────────────────┘            │                        │
         │                     ▼                        ▼
         │            ┌──────────────────┐    ┌─────────────────┐
         └───────────►│   AI Context     │◄──►│     AI Agent    │
                      └──────────────────┘    └─────────────────┘
         ▲                     │                        │
         │                     ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Preview Shell  │    │   AI Daemon      │    │  RPC Framework  │
│   (Tauri/web)   │    └──────────────────┘    └─────────────────┘
└─────────────────┘            │                        │
         ▲                     ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Preview Engine  │◄──►│ Workspace Daemon │◄──►│    Settings     │
│ (simulation,    │    └──────────────────┘    └─────────────────┘
│  device models) │
└─────────────────┘
```

**Preview Subsystem:** The Preview Engine crate (`crates/preview-engine`) contains the logic for simulating mobile devices, desktop frames, and browser environments. It runs a local HTTP server that serves the content to be previewed, which is displayed in the Tauri-based Preview Shell. The Desktop App communicates with the Preview Engine via RPC to update previews in real time as the user edits code or design specifications.

## Anti‑Patterns to Avoid

- **Tight coupling between UI and core logic** – Keep UI‑specific code out of `editor‑core`, `workspace‑model`, and other business‑logic crates.
- **Blocking the main thread with AI operations** – All AI processing and file operations should be asynchronous to keep the editor responsive.
- **Premature abstraction** – Build features for concrete use cases first before attempting generic solutions.
- **Ignoring security** – Leverage Tauri's security model and implement proper permission checks for file operations and AI requests.
- **Over-engineering** – Start with simple, working solutions and refactor as needed based on actual requirements.

## Communication Patterns

1. **Desktop ↔ Services**: RPC over local sockets or message passing
2. **Inter-crate**: Direct Rust dependencies for compile-time safety
3. **AI Operations**: Async streaming via the AI daemon
4. **LSP**: JSON-RPC over stdio or sockets via the lsp-client crate

## Key Design Principles

1. **Modularity**: Each crate has a single, well-defined responsibility
2. **Extensibility**: Plugin system for language support and AI providers
3. **Performance**: Native Rust implementation with async/await where appropriate
4. **Security**: Permission system for sensitive operations
5. **Observability**: Built-in tracing and logging throughout

## Extension Points

- **Language Support**: Via LSP client and extension crates
- **AI Providers**: Pluggable through the AI daemon
- **UI Themes**: Through the desktop application's theming system
- **Tools**: Extensible toolset for the AI agent

## Development Workflow

1. **Build**: `cargo build --workspace`
2. **Test**: `cargo test --workspace`
3. **Format**: `cargo fmt --all`
4. **Lint**: `cargo clippy --workspace --all-targets`
5. **Run**: Individual binaries can be run directly (e.g., `cargo run -p desktop`)

## Development Roadmap

1. **Phase 1 (Current)** – Core Tauri desktop application with workspace explorer, code editor, and AI assistant panels. Basic file operations and AI task execution.
2. **Phase 2** – Enhanced AI capabilities with context collection, better tool integration, and improved code editing features.
3. **Phase 3** – Language Server Protocol (LSP) integration for intelligent code analysis, completion, and diagnostics.
4. **Phase 4** – Preview system for mobile, desktop, and web simulations with real-time synchronization.

This roadmap ensures focused development on core IDE functionality while progressively adding advanced features.

## Future Considerations

- **Multi-platform Support**: Windows, macOS, Linux
- **Cloud Sync**: Optional synchronization of workspace state
- **Collaboration**: Real-time collaborative editing
- **Mobile**: Potential mobile companion applications

This architecture is designed to evolve as Zaroxi Studio grows, maintaining flexibility while providing a solid foundation for an AI-first development experience.
