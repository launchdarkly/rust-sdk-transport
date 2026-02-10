//! Generic HTTP transport trait for LaunchDarkly Rust libraries.
//!
//! This crate provides the [`transport::HttpTransport`] trait, which abstracts over HTTP client
//! implementations. This allows LaunchDarkly SDKs to work with different HTTP clients (hyper,
//! reqwest, or custom implementations) without being tightly coupled to any specific one.
//!
//! # Why Use This Crate?
//!
//! LaunchDarkly SDKs need to make HTTP requests for various purposes:
//! - Streaming feature flag updates via Server-Sent Events (SSE)
//! - Polling for flag data
//! - Sending analytics events
//!
//! Rather than forcing users to depend on a specific HTTP client, this crate provides a trait
//! that users can implement with their preferred HTTP library. This is especially useful when:
//! - You already use a specific HTTP client in your application
//! - You need custom networking behavior (proxies, custom TLS, etc.)
//! - You want to minimize dependencies
//!
//! # Core Components
//!
//! - [`transport::HttpTransport`] - The main trait to implement for custom HTTP clients
//! - [`transport::TransportError`] - Error type for transport operations
//! - [`transport::ByteStream`] - Type alias for streaming response bodies
//! - Re-exported types from the `http` crate for convenience
//!
//! # Example Usage
//!
//! Implementing the trait for a custom HTTP client:
//!
//! ```ignore
//! use launchdarkly_sdk_transport::{HttpTransport, ResponseFuture, ByteStream, TransportError};
//! use http::{Request, Response};
//!
//! #[derive(Clone)]
//! struct MyHttpClient {
//!     // Your HTTP client implementation
//! }
//!
//! impl HttpTransport for MyHttpClient {
//!     fn request(&self, request: Request<Option<String>>) -> ResponseFuture {
//!         Box::pin(async move {
//!             // Convert the http::Request to your client's format
//!             // Execute the request
//!             // Return http::Response<ByteStream>
//!             todo!("Implement your HTTP logic here")
//!         })
//!     }
//! }
//! ```
//!
//! Then pass your transport implementation to LaunchDarkly SDK configuration.

pub mod transport;
