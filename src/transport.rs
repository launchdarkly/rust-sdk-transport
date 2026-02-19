//! HTTP transport abstraction for LaunchDarkly SDKs.
//!
//! This module defines the [`HttpTransport`] trait which allows users to plug in
//! their own HTTP client implementation (hyper, reqwest, or custom).

use bytes::Bytes;
use futures::Stream;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

// Re-export http crate types for convenience
pub use http::{HeaderMap, HeaderValue, Request, Response, StatusCode, Uri};

/// A pinned, boxed stream of bytes returned by HTTP transports.
///
/// This represents the streaming response body from an HTTP request.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, TransportError>> + Send + Sync>>;

/// A pinned, boxed future for an HTTP response.
///
/// This represents the future returned by [`HttpTransport::request`].
pub type ResponseFuture =
    Pin<Box<dyn Future<Output = Result<Response<ByteStream>, TransportError>> + Send + Sync>>;

/// Error type for HTTP transport operations.
///
/// This wraps transport-specific errors (network failures, timeouts, etc.) in a
/// common error type that the SDKs can handle uniformly.
#[derive(Debug)]
pub struct TransportError {
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl TransportError {
    /// Create a new transport error from any error type.
    ///
    /// This is used by transport implementations to wrap their specific error types
    /// into the common `TransportError` type.
    ///
    /// # Example
    ///
    /// ```
    /// use launchdarkly_sdk_transport::TransportError;
    /// use std::io;
    ///
    /// let io_err = io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused");
    /// let transport_err = TransportError::new(io_err);
    /// ```
    pub fn new(err: impl StdError + Send + Sync + 'static) -> Self {
        Self {
            inner: Box::new(err),
        }
    }

    /// Get a reference to the inner error.
    ///
    /// Use this to access the underlying error for detailed inspection, logging,
    /// or to check for specific error types.
    ///
    /// # Example
    ///
    /// ```
    /// use launchdarkly_sdk_transport::TransportError;
    /// use std::io;
    ///
    /// fn diagnose_error(err: TransportError) {
    ///     let inner = err.inner();
    ///
    ///     // Check if it's an I/O error
    ///     if let Some(io_err) = inner.downcast_ref::<io::Error>() {
    ///         match io_err.kind() {
    ///             io::ErrorKind::TimedOut => println!("Operation timed out"),
    ///             io::ErrorKind::ConnectionRefused => println!("Connection refused"),
    ///             _ => println!("I/O error: {}", io_err),
    ///         }
    ///     } else {
    ///         println!("Other error: {}", inner);
    ///     }
    /// }
    /// ```
    pub fn inner(&self) -> &(dyn StdError + Send + Sync + 'static) {
        &*self.inner
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transport error: {}", self.inner)
    }
}

impl StdError for TransportError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.inner)
    }
}

/// Trait for pluggable HTTP transport implementations.
///
/// Implement this trait to provide HTTP request/response functionality. The transport is
/// responsible for:
///
/// - Establishing HTTP connections (with TLS if needed)
/// - Sending HTTP requests
/// - Returning streaming HTTP responses
/// - Handling timeouts (if desired)
///
/// # Example
///
/// ```no_run
/// use launchdarkly_sdk_transport::{HttpTransport, ByteStream, TransportError, Request,
/// ResponseFuture};
/// use bytes::Bytes;
///
/// #[derive(Clone)]
/// struct MyTransport {
///     // Your HTTP client here
/// }
///
/// impl HttpTransport for MyTransport {
///     fn request(&self, request: Request<Option<Bytes>>) -> ResponseFuture {
///         // Extract body from request
///         // Convert request to your HTTP client's format
///         // Make the request
///         // Return streaming response
///         todo!()
///     }
/// }
/// ```
pub trait HttpTransport: Send + Sync + Clone + 'static {
    /// Execute an HTTP request and return a streaming response.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to execute. The body type is `Option<Bytes>`
    ///   to support methods like REPORT that may include a request body. Use `None` for
    ///   requests without a body (like GET). `Bytes` can contain either text or binary data.
    ///
    /// # Returns
    ///
    /// A future that resolves to an HTTP response with a streaming body, or a
    /// transport error if the request fails.
    ///
    /// The response should include:
    /// - Status code
    /// - Response headers
    /// - A stream of body bytes
    ///
    /// # Notes
    ///
    /// - The transport should NOT follow redirects. This should be left to the consumer.
    /// - The transport should NOT retry requests. This should also be left to the consumer.
    /// - The transport MAY implement timeouts as desired
    fn request(&self, request: Request<Option<Bytes>>) -> ResponseFuture;
}
