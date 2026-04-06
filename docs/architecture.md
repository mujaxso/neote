# Neote Architecture

Neote is an AI-first IDE built as a modular Rust workspace. This document outlines the high-level architecture and component relationships.

## Overview

Neote follows a client-server architecture with a desktop frontend, multiple background services, and a collection of shared libraries (crates). The system is designed to be extensible, with clear separation between UI, business logic, and AI capabilities.

## Workspace Structure

### Applications (`apps/`)
- **desktop**: The main desktop application built with a native UI framework (to be chosen). Contains UI modules for editor, sidebar, chat, terminal, and layout management.

### Services (`services/`)
- **workspace-daemon**: Manages workspace state, file system operations, Git integration, terminal sessions, and background tasks.
- **ai-daemon**: Orchestrates AI operations including provider routing, streaming responses, quota management, and request routing.

### Core Crates (`crates/`)
- **core-types**: Shared data structures for IDs, events, commands, and protocol definitions.
- **editor-buffer**: Text editing primitives: buffer management, edits, positions, selections, and undo/redo.
- **workspace-model**: Models for workspace state, file trees, open editors, project graphs, and snapshots.
- **lsp-client**: Language Server Protocol client with session management, transport, capabilities, and diagnostics.
- **ai-context**: AI context collection, ranking, packing, and prompt construction.
- **ai-agent**: AI agent planning, execution, tool usage, patch generation, and verification.
- **patch-engine**: Diff generation, patch application, and preview functionality.
- **rpc**: Remote Procedure Call framework with client/server, framing, and message handling.
- **settings**: Configuration model and loader.
- **permissions**: Policy and grants for security and access control.

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
                               │                        │
                               ▼                        ▼
                      ┌──────────────────┐    ┌─────────────────┐
                      │   AI Daemon      │    │  RPC Framework  │
                      └──────────────────┘    └─────────────────┘
                               │                        │
                               ▼                        ▼
                      ┌──────────────────┐    ┌─────────────────┐
                      │ Workspace Daemon │◄──►│    Settings     │
                      └──────────────────┘    └─────────────────┘
```

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

## Future Considerations

- **Multi-platform Support**: Windows, macOS, Linux
- **Cloud Sync**: Optional synchronization of workspace state
- **Collaboration**: Real-time collaborative editing
- **Mobile**: Potential mobile companion applications

This architecture is designed to evolve as Neote grows, maintaining flexibility while providing a solid foundation for an AI-first development experience.
