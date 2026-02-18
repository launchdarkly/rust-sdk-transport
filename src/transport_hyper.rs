//! Hyper v1 transport implementation.
//!
//! This crate provides a production-ready [`HyperTransport`] implementation that
//! integrates hyper v1 with any LaunchDarkly [`HttpTransport`] clients.
//!
//! # Example
//!
//! ```no_run
//! use launchdarkly_sdk_transport::HyperTransport;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let transport = HyperTransport::new()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! This module is only available when the appropriate feature flags are enabled in your `Cargo.toml`.
//!
//! ## Available Features
//!
//! ### `hyper`
//!
//! Enables the hyper v1 transport implementation with HTTP-only support.
//!
//! **Use this when:**
//! - You only need plain HTTP connections
//! - You're handling TLS termination elsewhere (e.g., behind a load balancer)
//! - You want to minimize dependencies
//!
//! **Cargo.toml:**
//! ```toml
//! [dependencies]
//! launchdarkly-sdk-transport = { version = "0.0.1", features = ["hyper"] }
//! ```
//!
//! ### `native-tls`
//!
//! Enables HTTPS support using the native TLS library. This feature automatically includes the
//! `hyper` feature and adds TLS capabilities. Certificate validation relies on the operating
//! system's native certificate store.
//!
//! **Cargo.toml:**
//! ```toml
//! [dependencies]
//! launchdarkly-sdk-transport = { version = "0.0.1", features = ["native-tls"] }
//! ```
//!
//! ### `hyper-rustls-native-roots`
//!
//! Use the operating system's native certificate store for TLS certificate validation, relying on
//! the hyper-rustls crate.
//!
//! **Cargo.toml:**
//! ```toml
//! [dependencies]
//! launchdarkly-sdk-transport = { version = "0.0.1", features = ["hyper-rustls-native-roots"] }
//! ```
//!
//! ### `hyper-rustls-webpki-roots`
//!
//! Use Mozilla's curated WebPKI certificate bundle, compiled into the binary.
//!
//! **Cargo.toml:**
//! ```toml
//! [dependencies]
//! launchdarkly-sdk-transport = { version = "0.0.1", features = ["hyper-rustls-webpki-roots"] }
//! ```
//!
//! # Timeout Configuration
//!
//! ```no_run
//! use launchdarkly_sdk_transport::HyperTransport;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let transport = HyperTransport::builder()
//!     .connect_timeout(Duration::from_secs(10))
//!     .read_timeout(Duration::from_secs(30))
//!     .build_http()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! # TLS/HTTPS Configuration
//!
//! The HTTPS transport requires one root certificate provider to validate server
//! certificates. The behavior of `build_https()` depends on which features are enabled:
//!
//! - **`hyper-rustls-native-roots`**: Uses the OS certificate store.
//!
//! - **`hyper-rustls-webpki-roots`**: Uses Mozilla's curated certificate bundle compiled
//!   into the binary.
//!
//! - **`native-tls`**: Uses the platform's native TLS implementation.
//!
//! ## TLS Customization
//!
//! For custom TLS settings (custom CA certificates, client certificates, cipher suites, etc.),
//! you can create a custom connector and pass it to [`HyperTransportBuilder::build_with_connector`].
//!
//! ```no_run
//! # #[cfg(feature = "hyper-rustls-webpki-roots")]
//! # {
//! use launchdarkly_sdk_transport::HyperTransport;
//! use hyper_rustls::HttpsConnectorBuilder;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a custom HTTPS connector with specific settings
//! let https_connector = HttpsConnectorBuilder::new()
//!     .with_webpki_roots()  // Use WebPKI roots instead of native
//!     .https_only()         // Reject plain HTTP connections
//!     .enable_http1()
//!     .enable_http2()
//!     .build();
//!
//! let transport = HyperTransport::builder()
//!     .build_with_connector(https_connector)?;
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! # Proxy Configuration
//!
//! By default, the transport automatically detects proxy settings from environment variables
//! (`HTTP_PROXY`, `HTTPS_PROXY`, `NO_PROXY`). You can customize this behavior:
//!
//! ```no_run
//! use launchdarkly_sdk_transport::HyperTransport;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Disable proxy completely
//! let transport = HyperTransport::builder()
//!     .disable_proxy()
//!     .build_http()?;
//!
//! // Use a custom proxy URL
//! let transport = HyperTransport::builder()
//!     .proxy_url("http://proxy.example.com:8080".to_string())
//!     .build_http()?;
//!
//! // Explicitly enable auto-detection (default behavior)
//! let transport = HyperTransport::builder()
//!     .auto_proxy()
//!     .build_http()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Proxy Environment Variables
//!
//! When using automatic proxy detection (the default), the transport checks these environment
//! variables in order of precedence:
//!
//! - `http_proxy` / `HTTP_PROXY`: Proxy URL for HTTP requests
//! - `https_proxy` / `HTTPS_PROXY`: Proxy URL for HTTPS requests
//! - `no_proxy` / `NO_PROXY`: Comma-separated list of hosts to bypass the proxy
//!
//! **Note**: Lowercase variants take precedence over uppercase.
//!
//! ### Proxy Routing Behavior
//!
//! - **Both proxies set**: HTTP requests use `HTTP_PROXY`, HTTPS requests use `HTTPS_PROXY`
//! - **Only HTTP_PROXY set**: All requests (HTTP and HTTPS) route through `HTTP_PROXY`
//! - **Only HTTPS_PROXY set**: Only HTTPS requests use the proxy, HTTP requests connect directly
//! - **Neither set**: All requests connect directly (no proxy)
//!
//! ### NO_PROXY Format
//!
//! The `NO_PROXY` variable accepts a comma-separated list of patterns:
//!
//! - Domain names: `example.com` (matches example.com and all subdomains)
//! - IP addresses: `192.168.1.1`
//! - CIDR blocks: `10.0.0.0/8`
//! - Wildcard: `*` (disables proxy for all hosts)
//!
//! ### Proxy Authentication
//!
//! Proxy URLs can include authentication credentials:
//!
//! ```no_run
//! use launchdarkly_sdk_transport::HyperTransport;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let transport = HyperTransport::builder()
//!     .proxy_url("http://username:password@proxy.example.com:8080".to_string())
//!     .build_http()?;
//! # Ok(())
//! # }
//! ```
//!
//! Or via environment variables:
//!
//! ```bash
//! export HTTP_PROXY="http://username:password@proxy.example.com:8080"
//! ```

use crate::{ByteStream, HttpTransport, TransportError};
use bytes::Bytes;
use http::Uri;
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::body::Incoming;
use hyper_http_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_timeout::TimeoutConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use no_proxy::NoProxy;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// A transport implementation using hyper v1.x
///
/// This struct wraps a hyper client and implements the [`HttpTransport`] trait.
///
/// # Timeout Support
///
/// All three timeout types are fully supported via `hyper-timeout`:
/// - `connect_timeout` - Timeout for establishing the TCP connection
/// - `read_timeout` - Timeout for reading data from the connection
/// - `write_timeout` - Timeout for writing data to the connection
///
/// Timeouts are configured using the builder pattern. See [`HyperTransportBuilder`] for details.
///
/// # Example
///
/// ```no_run
/// use launchdarkly_sdk_transport::HyperTransport;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create transport with default HTTP connector
/// let _transport = HyperTransport::new()?;
///
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct HyperTransport<
    C = ProxyConnector<TimeoutConnector<hyper_util::client::legacy::connect::HttpConnector>>,
> {
    client: HyperClient<C, BoxBody<Bytes, Box<dyn std::error::Error + Send + Sync>>>,
}

impl HyperTransport {
    /// Create a new HyperTransport with default HTTP connector and no timeouts
    ///
    /// This creates a basic HTTP-only client that supports both HTTP/1 and HTTP/2.
    /// For HTTPS support or timeout configuration, use [`HyperTransport::builder()`].
    pub fn new() -> Result<Self, std::io::Error> {
        HyperTransport::builder().build_http()
    }

    /// Create a new HyperTransport with HTTPS support using rustls
    ///
    /// This creates an HTTPS client that supports both HTTP/1 and HTTP/2 protocols using
    /// rustls for TLS. The transport can handle both HTTP and HTTPS connections.
    ///
    /// This method is only available when the `hyper-rustls-native-roots` or
    /// `hyper-rustls-webpki-roots` feature is enabled.
    ///
    /// For timeout configuration or custom TLS settings, use [`HyperTransport::builder()`] instead.
    #[cfg(any(
        feature = "hyper-rustls-native-roots",
        feature = "hyper-rustls-webpki-roots"
    ))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(
            feature = "hyper-rustls-native-roots",
            feature = "hyper-rustls-webpki-roots"
        )))
    )]
    pub fn new_https() -> Result<
        HyperTransport<
            ProxyConnector<
                TimeoutConnector<
                    hyper_rustls::HttpsConnector<
                        hyper_util::client::legacy::connect::HttpConnector,
                    >,
                >,
            >,
        >,
        std::io::Error,
    > {
        HyperTransport::builder().build_https()
    }

    /// Create a new HyperTransport with HTTPS support using hyper-tls
    ///
    /// This creates an HTTPS client that supports both HTTP/1 and HTTP/2 protocols using the
    /// native TLS implementation. The transport can handle both HTTP and HTTPS connections.
    ///
    /// This method is only available when the `native-tls` feature is enabled.
    ///
    /// For timeout configuration or custom TLS settings, use [`HyperTransport::builder()`] instead.
    #[cfg(feature = "native-tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
    pub fn new_https() -> Result<
        HyperTransport<
            ProxyConnector<
                TimeoutConnector<
                    hyper_tls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
                >,
            >,
        >,
        std::io::Error,
    > {
        HyperTransport::builder().build_https()
    }

    /// Create a new builder for configuring HyperTransport
    ///
    /// The builder allows you to configure timeouts and choose between HTTP and HTTPS connectors.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use launchdarkly_sdk_transport::HyperTransport;
    /// use std::time::Duration;
    ///
    /// let transport = HyperTransport::builder()
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .read_timeout(Duration::from_secs(30))
    ///     .build_http();
    /// ```
    pub fn builder() -> HyperTransportBuilder {
        HyperTransportBuilder::default()
    }
}

impl<C> HttpTransport for HyperTransport<C>
where
    C: hyper_util::client::legacy::connect::Connect + Clone + Send + Sync + 'static,
{
    fn request(
        &self,
        request: http::Request<Option<Bytes>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<http::Response<ByteStream>, TransportError>>
                + Send
                + Sync
                + 'static,
        >,
    > {
        // Convert http::Request<Option<Bytes>> to hyper::Request<BoxBody>
        let (parts, body_opt) = request.into_parts();

        // Convert Option<Bytes> to BoxBody
        let body: BoxBody<Bytes, Box<dyn std::error::Error + Send + Sync>> = match body_opt {
            Some(bytes) => {
                // Use Full for non-empty bodies
                Full::new(bytes)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                    .boxed()
            }
            None => {
                // Use Empty for no body
                Empty::<Bytes>::new()
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                    .boxed()
            }
        };

        let hyper_req = hyper::Request::from_parts(parts, body);

        let client = self.client.clone();

        Box::pin(async move {
            // Make the request - timeouts are handled by TimeoutConnector
            let resp = client
                .request(hyper_req)
                .await
                .map_err(TransportError::new)?;

            let (parts, body) = resp.into_parts();

            // Convert hyper's Incoming body to ByteStream
            let byte_stream: ByteStream = Box::pin(body_to_stream(body));

            Ok(http::Response::from_parts(parts, byte_stream))
        })
    }
}

/// Builder for configuring a [`HyperTransport`].
///
/// This builder allows you to configure timeouts and choose between HTTP and HTTPS connectors.
///
/// # Example
///
/// ```no_run
/// use launchdarkly_sdk_transport::HyperTransport;
/// use std::time::Duration;
///
/// let transport = HyperTransport::builder()
///     .connect_timeout(Duration::from_secs(10))
///     .read_timeout(Duration::from_secs(30))
///     .build_http();
/// ```
#[derive(Default)]
pub struct HyperTransportBuilder {
    connect_timeout: Option<Duration>,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    proxy_config: Option<ProxyConfig>,
}

impl HyperTransportBuilder {
    /// Disable proxy support completely.
    ///
    /// When this is set, the transport will connect directly to all hosts,
    /// ignoring any proxy environment variables (`HTTP_PROXY`, `HTTPS_PROXY`, etc.).
    ///
    /// Use this when you need to ensure no proxy is used, even if proxy environment
    /// variables are set in the environment.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use launchdarkly_sdk_transport::HyperTransport;
    ///
    /// let transport = HyperTransport::builder()
    ///     .disable_proxy()
    ///     .build_http();
    /// ```
    pub fn disable_proxy(mut self) -> Self {
        self.proxy_config = Some(ProxyConfig::Disabled);
        self
    }

    /// Configure the transport to automatically detect proxy settings from environment variables.
    ///
    /// This is the default behavior if no proxy configuration method is called.
    ///
    /// # Environment Variables
    ///
    /// The transport checks these variables (lowercase variants take precedence):
    /// - `http_proxy` / `HTTP_PROXY`: Proxy for HTTP requests
    /// - `https_proxy` / `HTTPS_PROXY`: Proxy for HTTPS requests
    /// - `no_proxy` / `NO_PROXY`: Comma-separated bypass list
    ///
    /// # Routing Logic
    ///
    /// - If both HTTP_PROXY and HTTPS_PROXY are set, requests are routed based on their scheme
    /// - If only HTTP_PROXY is set, **all requests** (HTTP and HTTPS) use that proxy
    /// - If only HTTPS_PROXY is set, only HTTPS requests use the proxy
    /// - NO_PROXY hosts bypass the proxy regardless of scheme
    ///
    /// # Example
    ///
    /// ```no_run
    /// use launchdarkly_sdk_transport::HyperTransport;
    ///
    /// // Explicitly enable auto-detection
    /// let transport = HyperTransport::builder()
    ///     .auto_proxy()
    ///     .build_http();
    /// ```
    ///
    /// With environment variables:
    /// ```bash
    /// export HTTP_PROXY=http://proxy.corp.example.com:8080
    /// export NO_PROXY=localhost,127.0.0.1,.internal.example.com
    /// ```
    pub fn auto_proxy(mut self) -> Self {
        self.proxy_config = Some(ProxyConfig::Auto);
        self
    }

    /// Configure the transport to use a custom proxy URL for all requests.
    ///
    /// When this is set, **all requests** will route through the specified proxy,
    /// regardless of environment variables or request scheme. This overrides any
    /// `HTTP_PROXY` or `HTTPS_PROXY` environment variables.
    ///
    /// # URL Format
    ///
    /// The proxy URL must include the scheme (`http://` or `https://`) and can optionally
    /// include authentication credentials:
    ///
    /// - Without auth: `http://proxy.example.com:8080`
    /// - With auth: `http://username:password@proxy.example.com:8080`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use launchdarkly_sdk_transport::HyperTransport;
    ///
    /// // Basic proxy
    /// let transport = HyperTransport::builder()
    ///     .proxy_url("http://proxy.example.com:8080".to_string())
    ///     .build_http();
    ///
    /// // Proxy with authentication
    /// let transport = HyperTransport::builder()
    ///     .proxy_url("http://user:pass@proxy.example.com:8080".to_string())
    ///     .build_http();
    /// ```
    ///
    /// # Note
    ///
    /// Unlike `auto_proxy()`, this method does **not** respect the `NO_PROXY` environment
    /// variable. All requests will use the specified proxy.
    pub fn proxy_url(mut self, proxy_url: String) -> Self {
        self.proxy_config = Some(ProxyConfig::Custom(proxy_url));
        self
    }

    /// Set a connect timeout for establishing connections
    ///
    /// This timeout applies when establishing the TCP connection to the server.
    /// There is no connect timeout by default.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Set a read timeout for reading from connections
    ///
    /// This timeout applies when reading data from the connection.
    /// There is no read timeout by default.
    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self
    }

    /// Set a write timeout for writing to connections
    ///
    /// This timeout applies when writing data to the connection.
    /// There is no write timeout by default.
    pub fn write_timeout(mut self, timeout: Duration) -> Self {
        self.write_timeout = Some(timeout);
        self
    }

    /// Build with an HTTP connector
    ///
    /// Creates a transport that supports HTTP/1 and HTTP/2 over plain HTTP.
    pub fn build_http(self) -> Result<HyperTransport, std::io::Error> {
        let connector = hyper_util::client::legacy::connect::HttpConnector::new();
        self.build_with_connector(connector)
    }

    /// Build with an HTTPS connector using rustls
    ///
    /// Creates a transport that supports HTTP/1 and HTTP/2 over HTTPS using rustls for TLS.
    /// The transport can handle both plain HTTP and HTTPS connections.
    ///
    /// This method is only available when the `hyper-rustls-native-roots` or
    /// `hyper-rustls-webpki-roots` feature is enabled.
    ///
    /// For custom TLS configuration (custom CAs, client certificates, etc.), use
    /// [`build_with_connector`](Self::build_with_connector) with a custom rustls connector.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(any(feature = "hyper-rustls-native-roots", feature = "hyper-rustls-webpki-roots"))]
    /// # {
    /// use launchdarkly_sdk_transport::HyperTransport;
    /// use std::time::Duration;
    ///
    /// let transport = HyperTransport::builder()
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .build_https()
    ///     .expect("failed to build HTTPS transport");
    /// # }
    /// ```
    #[cfg(any(
        feature = "hyper-rustls-native-roots",
        feature = "hyper-rustls-webpki-roots"
    ))]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(
            feature = "hyper-rustls-native-roots",
            feature = "hyper-rustls-webpki-roots"
        )))
    )]
    pub fn build_https(
        self,
    ) -> Result<
        HyperTransport<
            ProxyConnector<
                TimeoutConnector<
                    hyper_rustls::HttpsConnector<
                        hyper_util::client::legacy::connect::HttpConnector,
                    >,
                >,
            >,
        >,
        std::io::Error,
    > {
        // When only native roots are enabled, use them and propagate errors.
        #[cfg(feature = "hyper-rustls-native-roots")]
        let builder = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        // When only webpki roots are enabled, use them (infallible).
        #[cfg(feature = "hyper-rustls-webpki-roots")]
        let builder = hyper_rustls::HttpsConnectorBuilder::new().with_webpki_roots();

        let connector = builder
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        self.build_with_connector(connector)
    }

    /// Build with an HTTPS connector using native TLS
    ///
    /// Creates a transport that supports HTTP/1 and HTTP/2 over HTTPS using native TLS.
    /// The transport can handle both plain HTTP and HTTPS connections.
    ///
    /// This method is only available when the `native-tls` feature is enabled.
    ///
    /// For custom TLS configuration (custom CAs, client certificates, etc.), use
    /// [`build_with_connector`](Self::build_with_connector) with a custom native TLS connector.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(feature = "native-tls")]
    /// # {
    /// use launchdarkly_sdk_transport::HyperTransport;
    /// use std::time::Duration;
    ///
    /// let transport = HyperTransport::builder()
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .build_https()
    ///     .expect("failed to build HTTPS transport");
    /// # }
    /// ```
    #[cfg(feature = "native-tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
    pub fn build_https(
        self,
    ) -> Result<
        HyperTransport<
            ProxyConnector<
                TimeoutConnector<
                    hyper_tls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
                >,
            >,
        >,
        std::io::Error,
    > {
        let connector = hyper_tls::HttpsConnector::new();

        self.build_with_connector(connector)
    }

    /// Build with a custom connector
    ///
    /// This allows you to provide your own connector implementation, which is useful for:
    /// - Custom TLS configuration
    /// - Proxy support
    /// - Connection pooling customization
    /// - Custom DNS resolution
    ///
    /// The connector will be automatically wrapped with a `TimeoutConnector` that applies
    /// the configured timeout settings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use launchdarkly_sdk_transport::HyperTransport;
    /// use hyper_util::client::legacy::connect::HttpConnector;
    /// use std::time::Duration;
    ///
    /// let mut connector = HttpConnector::new();
    /// // Configure the connector as needed
    /// connector.set_nodelay(true);
    ///
    /// let transport = HyperTransport::builder()
    ///     .read_timeout(Duration::from_secs(30))
    ///     .build_with_connector(connector);
    /// ```
    pub fn build_with_connector<C>(
        self,
        connector: C,
    ) -> Result<HyperTransport<ProxyConnector<TimeoutConnector<C>>>, std::io::Error>
    where
        C: tower::Service<http::Uri> + Clone + Send + Sync + 'static,
        C::Response: hyper_util::client::legacy::connect::Connection
            + hyper::rt::Read
            + hyper::rt::Write
            + Send
            + Unpin,
        C::Future: Send + 'static,
        C::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let mut timeout_connector = TimeoutConnector::new(connector);
        timeout_connector.set_connect_timeout(self.connect_timeout);
        timeout_connector.set_read_timeout(self.read_timeout);
        timeout_connector.set_write_timeout(self.write_timeout);

        // ProxyConnector::new() sets up TLS for connecting to HTTPS proxies.
        // Use it when we have a TLS-backed proxy (native-tls or rustls with native roots).
        // Otherwise use unsecured() which still supports HTTP proxies; TLS to target servers
        // is handled by the inner connector.
        #[cfg(any(
            feature = "hyper-rustls-native-roots",
            feature = "hyper-rustls-webpki-roots",
            feature = "native-tls"
        ))]
        let mut proxy_connector = ProxyConnector::new(timeout_connector)?;
        #[cfg(not(any(
            feature = "hyper-rustls-native-roots",
            feature = "hyper-rustls-webpki-roots",
            feature = "native-tls"
        )))]
        let mut proxy_connector = ProxyConnector::unsecured(timeout_connector);

        match self.proxy_config {
            Some(ProxyConfig::Auto) | None => {
                let http_proxy = std::env::var("http_proxy")
                    .or_else(|_| std::env::var("HTTP_PROXY"))
                    .unwrap_or_default();
                let https_proxy = std::env::var("https_proxy")
                    .or_else(|_| std::env::var("HTTPS_PROXY"))
                    .unwrap_or_default();
                let no_proxy = std::env::var("no_proxy")
                    .or_else(|_| std::env::var("NO_PROXY"))
                    .unwrap_or_default();
                let no_proxy = NoProxy::from(no_proxy);

                if !https_proxy.is_empty() {
                    let https_uri = https_proxy
                        .parse::<Uri>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                    let no_proxy = no_proxy.clone();
                    let custom: Intercept = Intercept::Custom(
                        (move |schema: Option<&str>,
                               host: Option<&str>,
                               _port: Option<u16>|
                              -> bool {
                            // This function should only enforce validation when it matches
                            // the schema of the proxy.
                            if !matches!(schema, Some("https")) {
                                return false;
                            }

                            match host {
                                None => false,
                                Some(h) => !no_proxy.matches(h),
                            }
                        })
                        .into(),
                    );
                    let proxy = Proxy::new(custom, https_uri);
                    proxy_connector.add_proxy(proxy);
                }

                if !http_proxy.is_empty() {
                    let http_uri = http_proxy
                        .parse::<Uri>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                    // If http_proxy is set but https_proxy is not, then all hosts are eligible to
                    // route through the http_proxy.
                    let proxy_all = https_proxy.is_empty();
                    let custom: Intercept = Intercept::Custom(
                        (move |schema: Option<&str>,
                               host: Option<&str>,
                               _port: Option<u16>|
                              -> bool {
                            if !proxy_all && matches!(schema, Some("https")) {
                                return false;
                            }

                            match host {
                                None => false,
                                Some(h) => !no_proxy.matches(h),
                            }
                        })
                        .into(),
                    );
                    let proxy = Proxy::new(custom, http_uri);
                    proxy_connector.add_proxy(proxy);
                }
            }
            Some(ProxyConfig::Disabled) => {
                // No proxies will be added, so the client will connect directly
            }
            Some(ProxyConfig::Custom(url)) => {
                let uri = url
                    .parse::<Uri>()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                proxy_connector.add_proxy(Proxy::new(Intercept::All, uri));
            }
        };

        let client = HyperClient::builder(TokioExecutor::new()).build(proxy_connector);

        Ok(HyperTransport { client })
    }
}

/// Proxy configuration for HyperTransport.
///
/// This determines whether and how the transport uses an HTTP/HTTPS proxy.
#[derive(Default, Debug, Clone)]
enum ProxyConfig {
    /// Automatically detect proxy from environment variables (default).
    ///
    /// Checks `HTTP_PROXY`, `HTTPS_PROXY`, and `NO_PROXY` environment variables.
    /// Lowercase variants take precedence over uppercase.
    #[default]
    Auto,

    /// Explicitly disable proxy support.
    ///
    /// No proxy will be used even if environment variables are set.
    Disabled,

    /// Use a custom proxy URL.
    ///
    /// Format: `http://[user:pass@]host:port`
    Custom(String),
}

/// Convert hyper's Incoming body to a Stream of Bytes
fn body_to_stream(
    body: Incoming,
) -> impl futures::Stream<Item = Result<Bytes, TransportError>> + Send + Sync {
    futures::stream::unfold(body, |mut body| async move {
        loop {
            match body.frame().await {
                Some(Ok(frame)) => {
                    if let Ok(data) = frame.into_data() {
                        return Some((Ok(data), body));
                    }
                    // Non-data frame (e.g., trailers) - skip and continue to next frame
                }
                Some(Err(e)) => return Some((Err(TransportError::new(e)), body)),
                None => return None,
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use http::{Method, Request};
    use std::time::Duration;

    #[test]
    fn test_hyper_transport_new() {
        let transport = HyperTransport::new();
        // If we can create it without panic, the test passes
        // This verifies the default HTTP connector is set up correctly
        drop(transport);
    }

    #[cfg(any(
        feature = "hyper-rustls-native-roots",
        feature = "hyper-rustls-webpki-roots"
    ))]
    #[test]
    fn test_hyper_transport_new_https() {
        let transport = HyperTransport::new_https().expect("transport failed to build");
        // If we can create it without panic, the test passes
        // This verifies the HTTPS connector with rustls is set up correctly
        drop(transport);
    }

    #[cfg(feature = "native-tls")]
    #[test]
    fn test_hyper_transport_new_https() {
        let transport = HyperTransport::new_https().expect("transport failed to build");
        // If we can create it without panic, the test passes
        // This verifies the HTTPS connector with native TLS is set up correctly
        drop(transport);
    }

    #[test]
    fn test_builder_default() {
        let builder = HyperTransport::builder();
        let transport = builder.build_http().expect("failed to build transport");
        // Verify we can build with default settings
        drop(transport);
    }

    #[test]
    fn test_builder_with_all_timeouts() {
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(30))
            .write_timeout(Duration::from_secs(10))
            .build_http()
            .expect("failed to build transport");
        // Verify we can build with all timeouts configured
        drop(transport);
    }

    #[cfg(any(
        feature = "hyper-rustls-native-roots",
        feature = "hyper-rustls-webpki-roots"
    ))]
    #[test]
    fn test_builder_https() {
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(30))
            .write_timeout(Duration::from_secs(10))
            .build_https()
            .expect("failed to build HTTPS transport");
        // Verify we can build HTTPS transport with timeouts
        drop(transport);
    }

    #[cfg(feature = "native-tls")]
    #[test]
    fn test_builder_https() {
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(30))
            .write_timeout(Duration::from_secs(10))
            .build_https()
            .expect("failed to build HTTPS transport");
        drop(transport);
    }

    #[test]
    fn test_builder_with_custom_connector() {
        let mut connector = hyper_util::client::legacy::connect::HttpConnector::new();
        connector.set_nodelay(true);

        let transport = HyperTransport::builder()
            .read_timeout(Duration::from_secs(30))
            .build_with_connector(connector);
        // Verify we can build with a custom connector
        drop(transport);
    }

    #[test]
    fn test_transport_is_clone() {
        let transport = HyperTransport::new().expect("failed to build transport");
        let _cloned = transport.clone();
    }

    #[tokio::test]
    async fn test_http_transport_trait_implemented() {
        let transport = HyperTransport::new().expect("failed to build transport");

        // Create a basic request
        let request = Request::builder()
            .method(Method::GET)
            .uri("http://httpbin.org/get")
            .body(None)
            .expect("failed to build request");

        // Verify the trait is implemented by attempting to call it
        // We're not actually making the request here, just verifying the types work
        let _future = transport.request(request);
        // The future exists and has the correct type signature
    }

    #[tokio::test]
    async fn test_request_with_empty_body() {
        // This test verifies that we can construct a request with no body
        let transport = HyperTransport::new().expect("failed to build transport");

        let request = Request::builder()
            .method(Method::GET)
            .uri("http://httpbin.org/get")
            .body(None)
            .expect("failed to build request");

        // Just verify we can create the future - not actually making network call
        let _future = transport.request(request);
    }

    #[tokio::test]
    async fn test_request_with_string_body() {
        // This test verifies that we can construct a request with a string body
        let transport = HyperTransport::new().expect("failed to build transport");

        let request = Request::builder()
            .method(Method::POST)
            .uri("http://httpbin.org/post")
            .body(Some(Bytes::from("test body")))
            .expect("failed to build request");

        // Just verify we can create the future - not actually making network call
        let _future = transport.request(request);
    }

    // Integration tests that actually make HTTP requests
    // These require a running HTTP server, so they're marked as ignored by default

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_integration_http_request() {
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(30))
            .build_http()
            .expect("failed to build transport");

        let request = Request::builder()
            .method(Method::GET)
            .uri("http://httpbin.org/get")
            .body(None)
            .expect("failed to build request");

        let response = transport.request(request).await;
        assert!(response.is_ok(), "Request should succeed");

        let response = response.unwrap();
        assert!(response.status().is_success(), "Status should be success");

        // Verify we can read from the stream
        let mut stream = response.into_body();
        let mut received_data = false;
        while let Some(result) = stream.next().await {
            assert!(result.is_ok(), "Stream chunk should not error");
            received_data = true;
        }
        assert!(received_data, "Should have received some data");
    }

    #[cfg(any(
        feature = "hyper-rustls-native-roots",
        feature = "hyper-rustls-webpki-roots"
    ))]
    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_integration_https_request() {
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(30))
            .build_https()
            .expect("failed to build HTTPS transport");

        // Using example.com as it's highly reliable and well-maintained
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://example.com/")
            .body(None)
            .expect("failed to build request");

        let response = transport.request(request).await;
        assert!(
            response.is_ok(),
            "HTTPS request should succeed: {:?}",
            response.as_ref().err()
        );

        let response = response.unwrap();
        assert!(
            response.status().is_success(),
            "Status should be success: {}",
            response.status()
        );
    }

    #[cfg(feature = "native-tls")]
    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_integration_https_request() {
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_secs(10))
            .read_timeout(Duration::from_secs(30))
            .build_https()
            .expect("failed to build HTTPS transport");

        // Using example.com as it's highly reliable and well-maintained
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://example.com/")
            .body(None)
            .expect("failed to build request");

        let response: Result<http::Response<crate::ByteStream>, crate::TransportError> =
            transport.request(request).await;
        assert!(
            response.is_ok(),
            "HTTPS request should succeed: {:?}",
            response.as_ref().err()
        );

        let response = response.unwrap();
        assert!(
            response.status().is_success(),
            "Status should be success: {}",
            response.status()
        );
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_integration_request_with_body() {
        let transport = HyperTransport::new().expect("failed to build transport");

        let body_content = r#"{"test": "data"}"#;
        let request = Request::builder()
            .method(Method::POST)
            .uri("http://httpbin.org/post")
            .header("Content-Type", "application/json")
            .body(Some(Bytes::from(body_content)))
            .expect("failed to build request");

        let response = transport.request(request).await;
        assert!(response.is_ok(), "POST request should succeed");

        let response = response.unwrap();
        assert!(response.status().is_success(), "Status should be success");
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_integration_streaming_response() {
        let transport = HyperTransport::new().expect("failed to build transport");

        let request = Request::builder()
            .method(Method::GET)
            .uri("http://httpbin.org/stream/10")
            .body(None)
            .expect("failed to build request");

        let response = transport.request(request).await;
        assert!(response.is_ok(), "Streaming request should succeed");

        let response = response.unwrap();
        assert!(response.status().is_success(), "Status should be success");

        // Verify we receive multiple chunks
        let mut stream = response.into_body();
        let mut chunk_count = 0;
        while let Some(result) = stream.next().await {
            assert!(result.is_ok(), "Stream chunk should not error");
            let chunk = result.unwrap();
            assert!(!chunk.is_empty(), "Chunk should not be empty");
            chunk_count += 1;
        }
        assert!(chunk_count > 0, "Should have received multiple chunks");
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_integration_connect_timeout() {
        // Use a non-routable IP to test connect timeout
        let transport = HyperTransport::builder()
            .connect_timeout(Duration::from_millis(100))
            .build_http()
            .expect("failed to build transport");

        let request = Request::builder()
            .method(Method::GET)
            .uri("http://10.255.255.1/")
            .body(None)
            .expect("failed to build request");

        let response = transport.request(request).await;
        assert!(response.is_err(), "Request should timeout");
    }
}
