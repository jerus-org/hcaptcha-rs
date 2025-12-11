<!--
SPDX-FileCopyrightText: 2025 jerusdp

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# hcaptcha-rs Architecture

This document describes the high-level architecture and design of the hcaptcha-rs project.

## Overview

hcaptcha-rs is a Rust library that provides backend verification for hCaptcha responses. The library validates tokens submitted by clients to ensure they completed a valid hCaptcha challenge.

## Project Structure

The project is organized as a Cargo workspace with multiple crates:

```
hcaptcha-rs/
├── hcaptcha/              # Core library
├── hcaptcha_derive/       # Procedural macros for derive functionality
├── hcaptcha-cli/          # Command-line interface
├── mock-verifier/         # Mock hCaptcha API server for testing
├── test-suite-*/          # Feature-specific test suites
└── test-wasm/             # WebAssembly-specific tests
```

## Core Components

### 1. Core Library (`hcaptcha/`)

The main library providing hCaptcha verification functionality.

#### Key Types

- **`Client`**: HTTP client for communicating with hCaptcha API
  - Reusable for multiple requests
  - Configurable with custom reqwest client
  - Supports both native TLS and rustls backends

- **`Request`**: Builder for verification requests
  - Contains secret key and captcha response
  - Optional fields: remoteip, sitekey
  - Validates input data before sending

- **`Captcha`**: Represents the client's captcha response token
  - Primary token from the client-side hCaptcha widget
  - Can include optional remote IP and sitekey

- **`Response`**: Parsed response from hCaptcha API
  - Success/failure status
  - Optional enterprise fields (score, score_reason)
  - Timestamp and challenge information
  - Error codes if verification failed

- **`Error`**: Comprehensive error handling
  - API errors (invalid-input, timeout-or-duplicate, etc.)
  - Network errors
  - Parsing errors
  - Client configuration errors

#### Feature Flags

- **`default`**: `ext` + `rustls-backend`
- **`rustls-backend`**: Use rustls for TLS (default)
- **`nativetls-backend`**: Use native-tls for TLS
- **`ext`**: Extended functionality (hex encoding support)
- **`enterprise`**: Enable hCaptcha Enterprise features
- **`trace`**: Enable tracing support for observability
- **`nightly`**: Unstable features for nightly Rust

### 2. Derive Macros (`hcaptcha_derive/`)

Provides the `#[derive(Hcaptcha)]` procedural macro for easy integration.

#### Functionality

- Generates `valid_response()` method on structs
- Automatically extracts captcha token from marked fields
- Uses `#[captcha]` attribute to identify token field
- Simplifies integration with web frameworks

#### Example Usage

```rust
#[derive(Deserialize, Hcaptcha)]
struct ContactForm {
    name: String,
    email: String,
    #[captcha]
    token: String,
}

// Generated method:
// async fn valid_response(&self, secret: &str, remoteip: Option<String>) -> Result<Response, Error>
```

### 3. CLI Tool (`hcaptcha-cli/`)

Command-line utility for manual testing and verification.

#### Features

- Verify captcha tokens from the command line
- Useful for debugging and integration testing
- Supports all core library features
- Configurable verbosity levels

### 4. Mock Verifier (`mock-verifier/`)

HTTP server that simulates the hCaptcha API for testing.

#### Purpose

- Enables testing without real hCaptcha API calls
- Predictable responses for different test scenarios
- Used extensively in the test suites
- Built with Rocket framework

## Request Flow

### Standard Verification Flow

```
Client Browser                 Backend Server              hCaptcha API
     |                               |                           |
     |---(1) Complete hCaptcha----->|                           |
     |<---(2) Return token----------|                           |
     |                               |                           |
     |---(3) Submit form + token--->|                           |
     |                               |                           |
     |                       [Request Builder]                   |
     |                               |                           |
     |                               |---(4) POST /siteverify-->|
     |                               |     (secret, response)    |
     |                               |                           |
     |                               |<---(5) JSON response-----|
     |                               |     (success, etc.)       |
     |                               |                           |
     |                       [Response Parser]                   |
     |                               |                           |
     |<---(6) Success/Failure--------|                           |
```

### Detailed Request Processing

1. **Client Interaction**: User completes hCaptcha challenge in browser
2. **Token Generation**: hCaptcha widget generates response token
3. **Form Submission**: Client submits form with token to backend
4. **Request Building**:
   - Backend creates `Captcha` with token
   - Builds `Request` with secret key
   - Optionally adds remote IP for additional validation
5. **HTTP Request**: 
   - `Client` sends POST to `https://api.hcaptcha.com/siteverify`
   - Body: `secret`, `response`, optional `remoteip` and `sitekey`
6. **Response Parsing**:
   - Deserialize JSON response
   - Check `success` field
   - Extract error codes if failed
   - Parse enterprise fields if available
7. **Result**: Return `Response` or `Error` to application

## Data Flow

### Request Data Structure

```rust
Request {
    secret: String,           // Secret key from hCaptcha dashboard
    captcha: Captcha {
        response: String,     // Token from client
        remoteip: Option<String>,  // Client IP address
        sitekey: Option<String>,   // Site key (enterprise)
    },
}
```

### Response Data Structure

```rust
Response {
    success: bool,                    // Verification result
    challenge_ts: Option<DateTime>,   // Challenge timestamp
    hostname: Option<String>,         // Site hostname
    credit: Option<bool>,             // Credit consumed
    error_codes: Option<Vec<Code>>,   // Error codes if failed
    score: Option<f64>,               // Enterprise: risk score
    score_reason: Option<Vec<String>>, // Enterprise: score reasons
}
```

## Testing Strategy

### Multi-Suite Approach

The project uses separate test suites to validate different feature combinations:

- **`test-suite-default`**: Default features (rustls backend)
- **`test-suite-enterprise`**: Enterprise features enabled
- **`test-suite-native-only`**: Native TLS backend only
- **`test-suite-no-default`**: No default features
- **`test-suite-rustls-only`**: rustls backend only
- **`test-suite-trace`**: Tracing support
- **`test-suite-cli`**: CLI application tests
- **`test-wasm`**: WebAssembly platform tests

### Testing Tools

- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Use `mock-verifier` for end-to-end scenarios
- **`wiremock`**: HTTP mocking for predictable responses
- **`trybuild`**: Compile-time test failures for derive macros
- **`macrotest`**: Macro expansion testing

### CI/CD Pipeline

CircleCI configuration includes:

- Build verification for all crates
- Test execution across all test suites
- Code coverage reporting (codecov)
- Clippy linting and formatting checks
- WASM platform testing
- Security scanning (OSSF Scorecard)

## Security Considerations

### Secret Handling

- Secrets are never logged or exposed
- Passed as references to minimize copying
- Used only in HTTPS requests to hCaptcha API

### Network Security

- All communication with hCaptcha API over HTTPS
- TLS certificate verification enforced
- Support for both rustls (pure Rust) and native-tls

### Input Validation

- Token format validation
- IP address format validation (if provided)
- Sitekey format validation (enterprise)
- Error handling prevents panic on malformed input

### Dependency Management

- Automated dependency updates via Renovate
- Regular security audits via OSSF Scorecard
- Minimal dependency footprint
- Use of well-maintained, security-focused crates

## Platform Support

### Native Platforms

- **Linux**: Fully supported (primary development platform)
- **macOS**: Fully supported
- **Windows**: Fully supported

### WebAssembly

- Supported via `wasm-bindgen`
- Tested with Node.js runtime
- Uses browser-compatible HTTP client

### Serverless

- AWS Lambda: Examples and testing provided
- Other platforms: Should work with any Rust-compatible serverless platform

### Rust Version

- **MSRV**: 1.88+
- Tested on stable channel
- CI runs on latest stable Rust and select tests on MSRV

## API Communication

### hCaptcha API Endpoint

```
POST https://api.hcaptcha.com/siteverify
Content-Type: application/x-www-form-urlencoded
```

### Request Parameters

- `secret` (required): Your secret key
- `response` (required): Token from client
- `remoteip` (optional): User's IP address
- `sitekey` (optional): Your site key (enterprise)

### Response Format

```json
{
  "success": true|false,
  "challenge_ts": "2025-01-01T00:00:00Z",
  "hostname": "example.com",
  "credit": true|false,
  "error-codes": ["invalid-input-response"],
  "score": 0.5,
  "score_reason": ["reason1", "reason2"]
}
```

## Error Handling

### Error Types

1. **API Errors**: Invalid tokens, duplicate submissions, etc.
2. **Network Errors**: Connection failures, timeouts
3. **Parse Errors**: Malformed responses from API
4. **Configuration Errors**: Invalid secret keys, malformed inputs

### Error Propagation

- Uses `thiserror` for ergonomic error definitions
- Errors implement `std::error::Error`
- Compatible with `?` operator and `Result` types
- Detailed error messages for debugging

## Performance Considerations

### Client Reuse

- `Client::verify_request()` allows client reuse
- Reduces connection overhead for multiple verifications
- Connection pooling handled by reqwest

### Async Design

- Built on `tokio` async runtime
- Non-blocking I/O for high throughput
- Compatible with async web frameworks

### Resource Usage

- Minimal memory footprint
- No persistent state required
- Suitable for high-concurrency scenarios

## Future Architectural Considerations

Potential areas for architectural evolution (see [ROADMAP.md](ROADMAP.md)):

- Alternative async runtime support
- Request batching capabilities
- Built-in caching strategies
- Enhanced metrics and observability
- Plugin system for custom validation logic

## References

- [hCaptcha API Documentation](https://docs.hcaptcha.com/)
- [API Documentation](https://docs.rs/hcaptcha)
- [Contributing Guide](CONTRIBUTING.md)
- [Roadmap](ROADMAP.md)

---

*Last Updated: December 2025*
