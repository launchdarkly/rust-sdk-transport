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
//! # Built-in Implementations
//!
//! This crate provides an optional hyper v1 transport implementation via feature flags:
//!
//! - **`hyper` feature**: HTTP-only transport using hyper v1
//!   - Includes proxy support, timeouts, HTTP/1 and HTTP/2
//!   - Suitable for plain HTTP or when TLS is handled elsewhere
//!
//! - **`hyper-rustls` feature**: Full HTTPS transport using rustls
//!   - Includes everything from `hyper` plus TLS support
//!   - Recommended for production use with HTTPS endpoints
//!
//! See [`transport_hyper::HyperTransport`] for detailed documentation and examples.
//!
//! **Quick Start with hyper-rustls:**
//!
//! ```toml
//! [dependencies]
//! launchdarkly-sdk-transport = { version = "0.0.1", features = ["hyper-rustls"] }
//! ```
//!
//! ```no_run
//! # #[cfg(feature = "hyper-rustls")]
//! # {
//! use launchdarkly_sdk_transport::HyperTransport;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let transport = HyperTransport::new_https()?;
//! // Use with LaunchDarkly SDK...
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! # Example Usage
//!
//! Implementing the trait for a custom HTTP client:
//!
//! ```ignore
//! use launchdarkly_sdk_transport::{HttpTransport, ResponseFuture, ByteStream, TransportError};
//! use bytes::Bytes;
//! use http::{Request, Response};
//!
//! #[derive(Clone)]
//! struct MyHttpClient {
//!     // Your HTTP client implementation
//! }
//!
//! impl HttpTransport for MyHttpClient {
//!     fn request(&self, request: Request<Option<Bytes>>) -> ResponseFuture {
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
pub use transport::*;

#[cfg(feature = "hyper")]
pub mod transport_hyper;
#[cfg(feature = "hyper")]
pub use transport_hyper::*;
