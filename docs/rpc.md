# Remote Procedure Call (RPC) Framework

Zaroxi Studio uses a custom RPC framework for communication between different components of the system, particularly between the desktop application and background services (workspace-daemon, ai-daemon).

## Overview

The RPC framework (`crates/rpc`) provides a lightweight, type-safe communication layer that enables:
- Inter-process communication between desktop app and services
- Async request/response patterns
- Streaming capabilities for long-running operations
- Error propagation and handling
- Connection management and reconnection

## Architecture

### Components

1. **Client** (`rpc::client`): Initiates RPC requests and handles responses
2. **Server** (`rpc::server`): Listens for incoming requests and dispatches to handlers
3. **Framing** (`rpc::framing`): Message serialization and transport over byte streams
4. **Messages** (`rpc::messages`): Type definitions for requests, responses, and notifications

### Transport Layer

The framework supports multiple transport mechanisms:
- **Unix domain sockets** (primary for local communication)
- **TCP sockets** (for remote or cross-machine communication)
- **In-memory channels** (for testing and same-process communication)

## Message Protocol

### Message Types

```rust
enum RpcMessage {
    // Request with unique ID for response matching
    Request {
        id: u64,
        method: String,
        params: serde_json::Value,
    },
    // Response to a previous request
    Response {
        id: u64,
        result: Result<serde_json::Value, RpcError>,
    },
    // Notification (no response expected)
    Notification {
        method: String,
        params: serde_json::Value,
    },
}
```

### Framing Format

Messages are framed using length-prefixed JSON:
```
[4-byte length][JSON message]
```

Where:
- Length is little-endian u32 indicating JSON byte count
- JSON message follows the `RpcMessage` structure

## Usage Examples

### Server Implementation

```rust
use rpc::server::Server;
use rpc::messages::{Request, Response};

let mut server = Server::new("/tmp/zaroxi.sock").await?;

server.register_handler("workspace.openFile", |params| async move {
    let path: String = serde_json::from_value(params)?;
    // Open file logic...
    Ok(serde_json::json!({ "success": true }))
}).await;

server.run().await?;
```

### Client Implementation

```rust
use rpc::client::Client;

let client = Client::connect("/tmp/zaroxi.sock").await?;

// Simple request/response
let response: serde_json::Value = client
    .request("workspace.openFile", serde_json::json!({ "path": "/home/user/file.rs" }))
    .await?;

// Notification (no response expected)
client.notify("editor.didChange", serde_json::json!({ "content": "new text" })).await;
```

## Error Handling

The RPC framework defines a comprehensive error type:

```rust
enum RpcError {
    // Transport errors
    ConnectionLost,
    Timeout,
    
    // Protocol errors
    InvalidMessage,
    MethodNotFound,
    
    // Application errors
    ApplicationError {
        code: i32,
        message: String,
        data: Option<serde_json::Value>,
    },
    
    // Serialization errors
    Serialization(serde_json::Error),
}
```

## Streaming Support

For operations that produce multiple results over time (e.g., AI streaming responses, file watching):

```rust
// Server-side streaming
server.register_stream("ai.complete", |params, mut stream| async move {
    let prompt: String = serde_json::from_value(params)?;
    
    for chunk in ai_model.stream_completion(&prompt).await {
        stream.send(chunk).await?;
    }
    
    Ok(())
}).await;

// Client-side consumption
let mut stream = client.stream("ai.complete", serde_json::json!({ "prompt": "Write a function" })).await?;

while let Some(chunk) = stream.next().await {
    println!("Received: {}", chunk);
}
```

## Security Considerations

### Authentication
- Unix domain sockets use filesystem permissions
- TCP connections support TLS encryption
- Token-based authentication for remote connections

### Authorization
- All RPC methods require appropriate permissions
- Permission checks are enforced at the server level
- Audit logging of all RPC calls

### Validation
- Input validation for all parameters
- Rate limiting to prevent abuse
- Size limits on messages to prevent DoS

## Performance Optimizations

1. **Connection pooling**: Reuse connections for multiple requests
2. **Batching**: Combine multiple requests into single messages
3. **Compression**: Optional gzip compression for large payloads
4. **Binary serialization**: Option to use MessagePack instead of JSON

## Testing

The RPC framework includes comprehensive testing utilities:

```rust
#[cfg(test)]
mod tests {
    use rpc::test_utils::TestServer;
    
    #[tokio::test]
    async fn test_basic_rpc() {
        let server = TestServer::new().await;
        let client = server.client().await;
        
        let response = client.request("test.ping", serde_json::json!({})).await;
        assert!(response.is_ok());
    }
}
```

## Integration with Other Crates

### Workspace Daemon
The workspace-daemon exposes RPC methods for:
- File system operations
- Git commands
- Terminal session management
- Task execution

### AI Daemon
The ai-daemon exposes RPC methods for:
- AI model inference
- Context management
- Streaming responses
- Quota tracking

### Desktop Application
The desktop app connects to both daemons and provides:
- UI event forwarding to services
- Service status monitoring
- Error reporting and recovery

## Future Enhancements

1. **Protocol versioning**: Backward-compatible protocol evolution
2. **Service discovery**: Automatic discovery of available services
3. **Load balancing**: Distribute requests across multiple service instances
4. **Metrics collection**: Performance monitoring and telemetry
5. **Protocol buffers**: Alternative serialization format for performance

The RPC framework is designed to be robust, performant, and extensible, serving as the communication backbone for Zaroxi Studio's distributed architecture.
