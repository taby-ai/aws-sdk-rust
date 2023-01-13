// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`CreateHomeRegionControlInput`](crate::input::CreateHomeRegionControlInput).
pub mod create_home_region_control_input {

    /// A builder for [`CreateHomeRegionControlInput`](crate::input::CreateHomeRegionControlInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) home_region: std::option::Option<std::string::String>,
        pub(crate) target: std::option::Option<crate::model::Target>,
        pub(crate) dry_run: std::option::Option<bool>,
    }
    impl Builder {
        /// <p>The name of the home region of the calling account.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.home_region = Some(input.into());
            self
        }
        /// <p>The name of the home region of the calling account.</p>
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.home_region = input;
            self
        }
        /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.target = Some(input);
            self
        }
        /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.target = input;
            self
        }
        /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
        pub fn dry_run(mut self, input: bool) -> Self {
            self.dry_run = Some(input);
            self
        }
        /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
        pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
            self.dry_run = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateHomeRegionControlInput`](crate::input::CreateHomeRegionControlInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::CreateHomeRegionControlInput,
            aws_smithy_http::operation::error::BuildError,
        > {
            Ok(crate::input::CreateHomeRegionControlInput {
                home_region: self.home_region,
                target: self.target,
                dry_run: self.dry_run.unwrap_or_default(),
            })
        }
    }
}
impl CreateHomeRegionControlInput {
    /// Consumes the builder and constructs an Operation<[`CreateHomeRegionControl`](crate::operation::CreateHomeRegionControl)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::CreateHomeRegionControl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_endpoint(_config.endpoint_url().map(|url| url.to_string()))
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::input::CreateHomeRegionControlInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::CreateHomeRegionControlInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSMigrationHubMultiAccountService.CreateHomeRegionControl",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_create_home_region_control(
                &self,
            )?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::CreateHomeRegionControl::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateHomeRegionControl",
            "migrationhubconfig",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`CreateHomeRegionControlInput`](crate::input::CreateHomeRegionControlInput).
    pub fn builder() -> crate::input::create_home_region_control_input::Builder {
        crate::input::create_home_region_control_input::Builder::default()
    }
}

/// See [`DescribeHomeRegionControlsInput`](crate::input::DescribeHomeRegionControlsInput).
pub mod describe_home_region_controls_input {

    /// A builder for [`DescribeHomeRegionControlsInput`](crate::input::DescribeHomeRegionControlsInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) control_id: std::option::Option<std::string::String>,
        pub(crate) home_region: std::option::Option<std::string::String>,
        pub(crate) target: std::option::Option<crate::model::Target>,
        pub(crate) max_results: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
        pub fn control_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.control_id = Some(input.into());
            self
        }
        /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
        pub fn set_control_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.control_id = input;
            self
        }
        /// <p>The name of the home region you'd like to view.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.home_region = Some(input.into());
            self
        }
        /// <p>The name of the home region you'd like to view.</p>
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.home_region = input;
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
        pub fn target(mut self, input: crate::model::Target) -> Self {
            self.target = Some(input);
            self
        }
        /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
        pub fn set_target(mut self, input: std::option::Option<crate::model::Target>) -> Self {
            self.target = input;
            self
        }
        /// <p>The maximum number of filtering results to display per page. </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        /// <p>The maximum number of filtering results to display per page. </p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeHomeRegionControlsInput`](crate::input::DescribeHomeRegionControlsInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::DescribeHomeRegionControlsInput,
            aws_smithy_http::operation::error::BuildError,
        > {
            Ok(crate::input::DescribeHomeRegionControlsInput {
                control_id: self.control_id,
                home_region: self.home_region,
                target: self.target,
                max_results: self.max_results,
                next_token: self.next_token,
            })
        }
    }
}
impl DescribeHomeRegionControlsInput {
    /// Consumes the builder and constructs an Operation<[`DescribeHomeRegionControls`](crate::operation::DescribeHomeRegionControls)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::DescribeHomeRegionControls,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_endpoint(_config.endpoint_url().map(|url| url.to_string()))
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::input::DescribeHomeRegionControlsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::DescribeHomeRegionControlsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSMigrationHubMultiAccountService.DescribeHomeRegionControls",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_describe_home_region_controls(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::DescribeHomeRegionControls::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DescribeHomeRegionControls",
            "migrationhubconfig",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`DescribeHomeRegionControlsInput`](crate::input::DescribeHomeRegionControlsInput).
    pub fn builder() -> crate::input::describe_home_region_controls_input::Builder {
        crate::input::describe_home_region_controls_input::Builder::default()
    }
}

/// See [`GetHomeRegionInput`](crate::input::GetHomeRegionInput).
pub mod get_home_region_input {

    /// A builder for [`GetHomeRegionInput`](crate::input::GetHomeRegionInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`GetHomeRegionInput`](crate::input::GetHomeRegionInput).
        pub fn build(
            self,
        ) -> Result<crate::input::GetHomeRegionInput, aws_smithy_http::operation::error::BuildError>
        {
            Ok(crate::input::GetHomeRegionInput {})
        }
    }
}
impl GetHomeRegionInput {
    /// Consumes the builder and constructs an Operation<[`GetHomeRegion`](crate::operation::GetHomeRegion)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetHomeRegion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_endpoint(_config.endpoint_url().map(|url| url.to_string()))
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetHomeRegionInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetHomeRegionInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSMigrationHubMultiAccountService.GetHomeRegion",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_get_home_region(&self)?,
        );
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetHomeRegion::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetHomeRegion",
            "migrationhubconfig",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetHomeRegionInput`](crate::input::GetHomeRegionInput).
    pub fn builder() -> crate::input::get_home_region_input::Builder {
        crate::input::get_home_region_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetHomeRegionInput {}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeHomeRegionControlsInput {
    /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    #[doc(hidden)]
    pub control_id: std::option::Option<std::string::String>,
    /// <p>The name of the home region you'd like to view.</p>
    #[doc(hidden)]
    pub home_region: std::option::Option<std::string::String>,
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    #[doc(hidden)]
    pub target: std::option::Option<crate::model::Target>,
    /// <p>The maximum number of filtering results to display per page. </p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeHomeRegionControlsInput {
    /// <p>The <code>ControlID</code> is a unique identifier string of your <code>HomeRegionControl</code> object.</p>
    pub fn control_id(&self) -> std::option::Option<&str> {
        self.control_id.as_deref()
    }
    /// <p>The name of the home region you'd like to view.</p>
    pub fn home_region(&self) -> std::option::Option<&str> {
        self.home_region.as_deref()
    }
    /// <p>The target parameter specifies the identifier to which the home region is applied, which is always of type <code>ACCOUNT</code>. It applies the home region to the current <code>ACCOUNT</code>.</p>
    pub fn target(&self) -> std::option::Option<&crate::model::Target> {
        self.target.as_ref()
    }
    /// <p>The maximum number of filtering results to display per page. </p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateHomeRegionControlInput {
    /// <p>The name of the home region of the calling account.</p>
    #[doc(hidden)]
    pub home_region: std::option::Option<std::string::String>,
    /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
    #[doc(hidden)]
    pub target: std::option::Option<crate::model::Target>,
    /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
    #[doc(hidden)]
    pub dry_run: bool,
}
impl CreateHomeRegionControlInput {
    /// <p>The name of the home region of the calling account.</p>
    pub fn home_region(&self) -> std::option::Option<&str> {
        self.home_region.as_deref()
    }
    /// <p>The account for which this command sets up a home region control. The <code>Target</code> is always of type <code>ACCOUNT</code>.</p>
    pub fn target(&self) -> std::option::Option<&crate::model::Target> {
        self.target.as_ref()
    }
    /// <p>Optional Boolean flag to indicate whether any effect should take place. It tests whether the caller has permission to make the call.</p>
    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}
