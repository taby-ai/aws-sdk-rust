// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Service config.
/// 
/// 
/// Service configuration allows for customization of endpoints, region, credentials providers,
/// and retry configuration. Generally, it is constructed automatically for you from a shared
/// configuration loaded by the `aws-config` crate. For example:
/// 
/// ```ignore
/// // Load a shared config from the environment
/// let shared_config = aws_config::from_env().load().await;
/// // The client constructor automatically converts the shared config into the service config
/// let client = Client::new(&shared_config);
/// ```
/// 
/// The service config can also be constructed manually using its builder.
/// 
pub struct Config {
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
                    sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
                    timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
    app_name: Option<aws_types::app_name::AppName>,
    pub (crate) endpoint_resolver: std::sync::Arc<dyn aws_smithy_http::endpoint::ResolveEndpoint<aws_endpoint::Params>>,
    pub(crate) region: Option<aws_types::region::Region>,
    pub(crate) credentials_provider: aws_types::credentials::SharedCredentialsProvider,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let mut config = f.debug_struct("Config");
                        config.finish()
                    }
}
impl Config {
    /// Constructs a config builder.
                    pub fn builder() -> Builder { Builder::default() }
    /// Return a reference to the retry configuration contained in this config, if any.
                        pub fn retry_config(&self) -> Option<&aws_smithy_types::retry::RetryConfig> {
                            self.retry_config.as_ref()
                        }
    
                        /// Return a cloned Arc containing the async sleep implementation from this config, if any.
                        pub fn sleep_impl(&self) -> Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>> {
                            self.sleep_impl.clone()
                        }
    
                        /// Return a reference to the timeout configuration contained in this config, if any.
                        pub fn timeout_config(&self) -> Option<&aws_smithy_types::timeout::TimeoutConfig> {
                            self.timeout_config.as_ref()
                        }
    /// Returns the name of the app that is using the client, if it was provided.
                        ///
                        /// This _optional_ name is used to identify the application in the user agent that
                        /// gets sent along with requests.
                        pub fn app_name(&self) -> Option<&aws_types::app_name::AppName> {
                            self.app_name.as_ref()
                        }
    /// Creates a new [service config](crate::Config) from a [shared `config`](aws_types::sdk_config::SdkConfig).
                        pub fn new(config: &aws_types::sdk_config::SdkConfig) -> Self {
                            Builder::from(config).build()
                        }
    /// The signature version 4 service signing name to use in the credential scope when signing requests.
                    ///
                    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
                    /// [`SigningService`](aws_types::SigningService) during operation construction
                    pub fn signing_service(&self) -> &'static str {
                        "databrew"
                    }
    /// Returns the AWS region, if it was provided.
                    pub fn region(&self) -> Option<&aws_types::region::Region> {
                        self.region.as_ref()
                    }
    /// Returns the credentials provider.
                    pub fn credentials_provider(&self) -> aws_types::credentials::SharedCredentialsProvider {
                        self.credentials_provider.clone()
                    }
}
/// Builder for creating a `Config`.
#[derive(Default)]pub struct Builder {
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
                        sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
                        timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
    app_name: Option<aws_types::app_name::AppName>,
    endpoint_resolver: Option<std::sync::Arc<dyn aws_smithy_http::endpoint::ResolveEndpoint<aws_endpoint::Params>>>,
    region: Option<aws_types::region::Region>,
    credentials_provider: Option<aws_types::credentials::SharedCredentialsProvider>,
}
impl Builder {
    /// Constructs a config builder.
    pub fn new() -> Self { Self::default() }
    /// Set the retry_config for the builder
                        ///
                        /// # Examples
                        /// ```no_run
                        /// use aws_sdk_databrew::config::Config;
                        /// use aws_sdk_databrew::config::retry::RetryConfig;
                        ///
                        /// let retry_config = RetryConfig::standard().with_max_attempts(5);
                        /// let config = Config::builder().retry_config(retry_config).build();
                        /// ```
                        pub fn retry_config(mut self, retry_config: aws_smithy_types::retry::RetryConfig) -> Self {
                            self.set_retry_config(Some(retry_config));
                            self
                        }
    
                        /// Set the retry_config for the builder
                        ///
                        /// # Examples
                        /// ```no_run
                        /// use aws_sdk_databrew::config::{Builder, Config};
                        /// use aws_sdk_databrew::config::retry::RetryConfig;
                        ///
                        /// fn disable_retries(builder: &mut Builder) {
                        ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
                        ///     builder.set_retry_config(Some(retry_config));
                        /// }
                        ///
                        /// let mut builder = Config::builder();
                        /// disable_retries(&mut builder);
                        /// let config = builder.build();
                        /// ```
                        pub fn set_retry_config(&mut self, retry_config: Option<aws_smithy_types::retry::RetryConfig>) -> &mut Self {
                            self.retry_config = retry_config;
                            self
                        }
    
                        /// Set the sleep_impl for the builder
                        ///
                        /// # Examples
                        ///
                        /// ```no_run
                        /// use aws_sdk_databrew::config::{AsyncSleep, Sleep, Config};
                        ///
                        /// #[derive(Debug)]
                        /// pub struct ForeverSleep;
                        ///
                        /// impl AsyncSleep for ForeverSleep {
                        ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
                        ///         Sleep::new(std::future::pending())
                        ///     }
                        /// }
                        ///
                        /// let sleep_impl = std::sync::Arc::new(ForeverSleep);
                        /// let config = Config::builder().sleep_impl(sleep_impl).build();
                        /// ```
                        pub fn sleep_impl(mut self, sleep_impl: std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>) -> Self {
                            self.set_sleep_impl(Some(sleep_impl));
                            self
                        }
    
                        /// Set the sleep_impl for the builder
                        ///
                        /// # Examples
                        ///
                        /// ```no_run
                        /// use aws_sdk_databrew::config::{AsyncSleep, Sleep, Builder, Config};
                        ///
                        /// #[derive(Debug)]
                        /// pub struct ForeverSleep;
                        ///
                        /// impl AsyncSleep for ForeverSleep {
                        ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
                        ///         Sleep::new(std::future::pending())
                        ///     }
                        /// }
                        ///
                        /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
                        ///     let sleep_impl = std::sync::Arc::new(ForeverSleep);
                        ///     builder.set_sleep_impl(Some(sleep_impl));
                        /// }
                        ///
                        /// let mut builder = Config::builder();
                        /// set_never_ending_sleep_impl(&mut builder);
                        /// let config = builder.build();
                        /// ```
                        pub fn set_sleep_impl(&mut self, sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>) -> &mut Self {
                            self.sleep_impl = sleep_impl;
                            self
                        }
    
                        /// Set the timeout_config for the builder
                        ///
                        /// # Examples
                        ///
                        /// ```no_run
                        /// # use std::time::Duration;
                        /// use aws_sdk_databrew::config::Config;
                        /// use aws_sdk_databrew::config::timeout::TimeoutConfig;
                        ///
                        /// let timeout_config = TimeoutConfig::builder()
                        ///     .operation_attempt_timeout(Duration::from_secs(1))
                        ///     .build();
                        /// let config = Config::builder().timeout_config(timeout_config).build();
                        /// ```
                        pub fn timeout_config(mut self, timeout_config: aws_smithy_types::timeout::TimeoutConfig) -> Self {
                            self.set_timeout_config(Some(timeout_config));
                            self
                        }
    
                        /// Set the timeout_config for the builder
                        ///
                        /// # Examples
                        ///
                        /// ```no_run
                        /// # use std::time::Duration;
                        /// use aws_sdk_databrew::config::{Builder, Config};
                        /// use aws_sdk_databrew::config::timeout::TimeoutConfig;
                        ///
                        /// fn set_request_timeout(builder: &mut Builder) {
                        ///     let timeout_config = TimeoutConfig::builder()
                        ///         .operation_attempt_timeout(Duration::from_secs(1))
                        ///         .build();
                        ///     builder.set_timeout_config(Some(timeout_config));
                        /// }
                        ///
                        /// let mut builder = Config::builder();
                        /// set_request_timeout(&mut builder);
                        /// let config = builder.build();
                        /// ```
                        pub fn set_timeout_config(&mut self, timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>) -> &mut Self {
                            self.timeout_config = timeout_config;
                            self
                        }
    /// Sets the name of the app that is using the client.
                        ///
                        /// This _optional_ name is used to identify the application in the user agent that
                        /// gets sent along with requests.
                        pub fn app_name(mut self, app_name: aws_types::app_name::AppName) -> Self {
                            self.set_app_name(Some(app_name));
                            self
                        }
    
                        /// Sets the name of the app that is using the client.
                        ///
                        /// This _optional_ name is used to identify the application in the user agent that
                        /// gets sent along with requests.
                        pub fn set_app_name(&mut self, app_name: Option<aws_types::app_name::AppName>) -> &mut Self {
                            self.app_name = app_name;
                            self
                        }
    /// Overrides the endpoint resolver to use when making requests.
                        ///
                        /// When unset, the client will used a generated endpoint resolver based on the endpoint metadata
                        /// for `aws_sdk_databrew`.
                        ///
                        /// # Examples
                        /// ```no_run
                        /// use aws_types::region::Region;
                        /// use aws_sdk_databrew::config::{Builder, Config};
                        /// use aws_sdk_databrew::Endpoint;
                        ///
                        /// let config = aws_sdk_databrew::Config::builder()
                        ///     .endpoint_resolver(
                        ///         Endpoint::immutable("http://localhost:8080".parse().expect("valid URI"))
                        ///     ).build();
                        /// ```
                        pub fn endpoint_resolver(mut self, endpoint_resolver: impl aws_endpoint::ResolveAwsEndpoint + 'static) -> Self {
                            self.endpoint_resolver = Some(std::sync::Arc::new(aws_endpoint::EndpointShim::from_resolver(endpoint_resolver)) as _);
                            self
                        }
    
                        /// Sets the endpoint resolver to use when making requests.
                        pub fn set_endpoint_resolver(&mut self, endpoint_resolver: Option<std::sync::Arc<dyn aws_endpoint::ResolveAwsEndpoint>>) -> &mut Self {
                            self.endpoint_resolver = endpoint_resolver.map(|res|std::sync::Arc::new(aws_endpoint::EndpointShim::from_arc(res) ) as _);
                            self
                        }
    /// Sets the AWS region to use when making requests.
                        ///
                        /// # Examples
                        /// ```no_run
                        /// use aws_types::region::Region;
                        /// use aws_sdk_databrew::config::{Builder, Config};
                        ///
                        /// let config = aws_sdk_databrew::Config::builder()
                        ///     .region(Region::new("us-east-1"))
                        ///     .build();
                        /// ```
                        pub fn region(mut self, region: impl Into<Option<aws_types::region::Region>>) -> Self {
                            self.region = region.into();
                            self
                        }
    /// Sets the credentials provider for this service
                        pub fn credentials_provider(mut self, credentials_provider: impl aws_types::credentials::ProvideCredentials + 'static) -> Self {
                            self.credentials_provider = Some(aws_types::credentials::SharedCredentialsProvider::new(credentials_provider));
                            self
                        }
    
                        /// Sets the credentials provider for this service
                        pub fn set_credentials_provider(&mut self, credentials_provider: Option<aws_types::credentials::SharedCredentialsProvider>) -> &mut Self {
                            self.credentials_provider = credentials_provider;
                            self
                        }
    /// Builds a [`Config`].
    pub fn build(self) -> Config {
        Config {
            retry_config: self.retry_config,
                            sleep_impl: self.sleep_impl,
                            timeout_config: self.timeout_config,
            app_name: self.app_name,
            endpoint_resolver: self.endpoint_resolver.unwrap_or_else(||
                                    std::sync::Arc::new(aws_endpoint::EndpointShim::from_resolver(crate::aws_endpoint::endpoint_resolver()))
                                ),
            region: self.region,
            credentials_provider: self.credentials_provider.unwrap_or_else(|| aws_types::credentials::SharedCredentialsProvider::new(crate::no_credentials::NoCredentials)),
        }
    }
}

impl From<&aws_types::sdk_config::SdkConfig> for Builder {
                    fn from(input: &aws_types::sdk_config::SdkConfig) -> Self {
                        let mut builder = Builder::default();
                        builder = builder.region(input.region().cloned());
                        builder.set_endpoint_resolver(input.endpoint_resolver().clone());
                        builder.set_retry_config(input.retry_config().cloned());
                        builder.set_timeout_config(input.timeout_config().cloned());
                        builder.set_sleep_impl(input.sleep_impl());
                        builder.set_credentials_provider(input.credentials_provider().cloned());
                        builder.set_app_name(input.app_name().cloned());
                        builder
                    }
                }

                impl From<&aws_types::sdk_config::SdkConfig> for Config {
                    fn from(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Builder::from(sdk_config).build()
                    }
                }

pub use aws_smithy_async::rt::sleep::{AsyncSleep, Sleep};

                /// Retry configuration
                ///
                /// These are re-exported from `aws-smithy-types` for convenience.
                pub mod retry {
                    pub use aws_smithy_types::retry::{RetryConfig, RetryConfigBuilder, RetryMode};
                }
                /// Timeout configuration
                ///
                /// These are re-exported from `aws-smithy-types` for convenience.
                pub mod timeout {
                    pub use aws_smithy_types::timeout::{TimeoutConfig, TimeoutConfigBuilder};
                }

