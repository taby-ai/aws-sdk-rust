// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>There was an exception validating this data.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ValidationException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for ValidationException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ValidationException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
/// See [`ValidationException`](crate::error::ValidationException).
pub mod validation_exception {
    
    /// A builder for [`ValidationException`](crate::error::ValidationException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ValidationException`](crate::error::ValidationException).
        pub fn build(self) -> crate::error::ValidationException {
            crate::error::ValidationException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::error::ValidationException).
    pub fn builder() -> crate::error::validation_exception::Builder {
        crate::error::validation_exception::Builder::default()
    }
}

/// <p>The requested resource was not found.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ResourceNotFoundException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for ResourceNotFoundException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
pub mod resource_not_found_exception {
    
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>There was an exception with the internal server.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct InternalServerException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for InternalServerException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException).
pub mod internal_server_exception {
    
    /// A builder for [`InternalServerException`](crate::error::InternalServerException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException).
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException).
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// <p>Access was denied for this action.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct AccessDeniedException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for AccessDeniedException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessDeniedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl AccessDeniedException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessDeniedException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for AccessDeniedException {}
/// See [`AccessDeniedException`](crate::error::AccessDeniedException).
pub mod access_denied_exception {
    
    /// A builder for [`AccessDeniedException`](crate::error::AccessDeniedException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`AccessDeniedException`](crate::error::AccessDeniedException).
        pub fn build(self) -> crate::error::AccessDeniedException {
            crate::error::AccessDeniedException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl AccessDeniedException {
    /// Creates a new builder-style object to manufacture [`AccessDeniedException`](crate::error::AccessDeniedException).
    pub fn builder() -> crate::error::access_denied_exception::Builder {
        crate::error::access_denied_exception::Builder::default()
    }
}

/// <p>There was a conflict with this action, and it could not be completed.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ConflictException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for ConflictException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ConflictException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ConflictException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for ConflictException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConflictException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for ConflictException {}
/// See [`ConflictException`](crate::error::ConflictException).
pub mod conflict_exception {
    
    /// A builder for [`ConflictException`](crate::error::ConflictException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ConflictException`](crate::error::ConflictException).
        pub fn build(self) -> crate::error::ConflictException {
            crate::error::ConflictException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ConflictException {
    /// Creates a new builder-style object to manufacture [`ConflictException`](crate::error::ConflictException).
    pub fn builder() -> crate::error::conflict_exception::Builder {
        crate::error::conflict_exception::Builder::default()
    }
}

/// Error type for the `CreateEndpoint` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct CreateEndpointError {
    /// Kind of error that occurred.
                    pub kind: CreateEndpointErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `CreateEndpoint` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum CreateEndpointErrorKind {
    /// <p>Access was denied for this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>There was a conflict with this action, and it could not be completed.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>There was an exception with the internal server.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The requested resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>There was an exception validating this data.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for CreateEndpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CreateEndpointErrorKind::AccessDeniedException(_inner) =>
            _inner.fmt(f)
            ,
            CreateEndpointErrorKind::ConflictException(_inner) =>
            _inner.fmt(f)
            ,
            CreateEndpointErrorKind::InternalServerException(_inner) =>
            _inner.fmt(f)
            ,
            CreateEndpointErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            CreateEndpointErrorKind::ValidationException(_inner) =>
            _inner.fmt(f)
            ,
            CreateEndpointErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateEndpointError {
    fn code(&self) -> Option<&str> {
        CreateEndpointError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateEndpointError {
    /// Creates a new `CreateEndpointError`.
                    pub fn new(kind: CreateEndpointErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `CreateEndpointError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: CreateEndpointErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `CreateEndpointError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: CreateEndpointErrorKind::Unhandled(err.into()),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `CreateEndpointErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(&self.kind, CreateEndpointErrorKind::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `CreateEndpointErrorKind::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(&self.kind, CreateEndpointErrorKind::ConflictException(_))
    }
    /// Returns `true` if the error kind is `CreateEndpointErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(&self.kind, CreateEndpointErrorKind::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `CreateEndpointErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, CreateEndpointErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreateEndpointErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, CreateEndpointErrorKind::ValidationException(_))
    }
}
impl std::error::Error for CreateEndpointError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            CreateEndpointErrorKind::AccessDeniedException(_inner) =>
            Some(_inner)
            ,
            CreateEndpointErrorKind::ConflictException(_inner) =>
            Some(_inner)
            ,
            CreateEndpointErrorKind::InternalServerException(_inner) =>
            Some(_inner)
            ,
            CreateEndpointErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            CreateEndpointErrorKind::ValidationException(_inner) =>
            Some(_inner)
            ,
            CreateEndpointErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `DeleteEndpoint` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct DeleteEndpointError {
    /// Kind of error that occurred.
                    pub kind: DeleteEndpointErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `DeleteEndpoint` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum DeleteEndpointErrorKind {
    /// <p>Access was denied for this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>There was an exception with the internal server.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The requested resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>There was an exception validating this data.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteEndpointErrorKind::AccessDeniedException(_inner) =>
            _inner.fmt(f)
            ,
            DeleteEndpointErrorKind::InternalServerException(_inner) =>
            _inner.fmt(f)
            ,
            DeleteEndpointErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            DeleteEndpointErrorKind::ValidationException(_inner) =>
            _inner.fmt(f)
            ,
            DeleteEndpointErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteEndpointError {
    fn code(&self) -> Option<&str> {
        DeleteEndpointError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteEndpointError {
    /// Creates a new `DeleteEndpointError`.
                    pub fn new(kind: DeleteEndpointErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `DeleteEndpointError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: DeleteEndpointErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `DeleteEndpointError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: DeleteEndpointErrorKind::Unhandled(err.into()),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `DeleteEndpointErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(&self.kind, DeleteEndpointErrorKind::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `DeleteEndpointErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(&self.kind, DeleteEndpointErrorKind::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `DeleteEndpointErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, DeleteEndpointErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `DeleteEndpointErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, DeleteEndpointErrorKind::ValidationException(_))
    }
}
impl std::error::Error for DeleteEndpointError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteEndpointErrorKind::AccessDeniedException(_inner) =>
            Some(_inner)
            ,
            DeleteEndpointErrorKind::InternalServerException(_inner) =>
            Some(_inner)
            ,
            DeleteEndpointErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            DeleteEndpointErrorKind::ValidationException(_inner) =>
            Some(_inner)
            ,
            DeleteEndpointErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `ListEndpoints` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct ListEndpointsError {
    /// Kind of error that occurred.
                    pub kind: ListEndpointsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `ListEndpoints` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum ListEndpointsErrorKind {
    /// <p>Access was denied for this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>There was an exception with the internal server.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The requested resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>There was an exception validating this data.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListEndpointsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListEndpointsErrorKind::AccessDeniedException(_inner) =>
            _inner.fmt(f)
            ,
            ListEndpointsErrorKind::InternalServerException(_inner) =>
            _inner.fmt(f)
            ,
            ListEndpointsErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            ListEndpointsErrorKind::ValidationException(_inner) =>
            _inner.fmt(f)
            ,
            ListEndpointsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListEndpointsError {
    fn code(&self) -> Option<&str> {
        ListEndpointsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListEndpointsError {
    /// Creates a new `ListEndpointsError`.
                    pub fn new(kind: ListEndpointsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `ListEndpointsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: ListEndpointsErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `ListEndpointsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: ListEndpointsErrorKind::Unhandled(err.into()),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `ListEndpointsErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(&self.kind, ListEndpointsErrorKind::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `ListEndpointsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(&self.kind, ListEndpointsErrorKind::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `ListEndpointsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, ListEndpointsErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListEndpointsErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, ListEndpointsErrorKind::ValidationException(_))
    }
}
impl std::error::Error for ListEndpointsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListEndpointsErrorKind::AccessDeniedException(_inner) =>
            Some(_inner)
            ,
            ListEndpointsErrorKind::InternalServerException(_inner) =>
            Some(_inner)
            ,
            ListEndpointsErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            ListEndpointsErrorKind::ValidationException(_inner) =>
            Some(_inner)
            ,
            ListEndpointsErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `ListSharedEndpoints` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct ListSharedEndpointsError {
    /// Kind of error that occurred.
                    pub kind: ListSharedEndpointsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `ListSharedEndpoints` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum ListSharedEndpointsErrorKind {
    /// <p>Access was denied for this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>There was an exception with the internal server.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The requested resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>There was an exception validating this data.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListSharedEndpointsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListSharedEndpointsErrorKind::AccessDeniedException(_inner) =>
            _inner.fmt(f)
            ,
            ListSharedEndpointsErrorKind::InternalServerException(_inner) =>
            _inner.fmt(f)
            ,
            ListSharedEndpointsErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            ListSharedEndpointsErrorKind::ValidationException(_inner) =>
            _inner.fmt(f)
            ,
            ListSharedEndpointsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListSharedEndpointsError {
    fn code(&self) -> Option<&str> {
        ListSharedEndpointsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListSharedEndpointsError {
    /// Creates a new `ListSharedEndpointsError`.
                    pub fn new(kind: ListSharedEndpointsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `ListSharedEndpointsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: ListSharedEndpointsErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `ListSharedEndpointsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: ListSharedEndpointsErrorKind::Unhandled(err.into()),
                        }
                    }
    
                    /// Returns the error message if one is available.
                    pub fn message(&self) -> Option<&str> {
                        self.meta.message()
                    }
    
                    /// Returns error metadata, which includes the error code, message,
                    /// request ID, and potentially additional information.
                    pub fn meta(&self) -> &aws_smithy_types::Error {
                        &self.meta
                    }
    
                    /// Returns the request ID if it's available.
                    pub fn request_id(&self) -> Option<&str> {
                        self.meta.request_id()
                    }
    
                    /// Returns the error code if it's available.
                    pub fn code(&self) -> Option<&str> {
                        self.meta.code()
                    }
    /// Returns `true` if the error kind is `ListSharedEndpointsErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(&self.kind, ListSharedEndpointsErrorKind::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `ListSharedEndpointsErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(&self.kind, ListSharedEndpointsErrorKind::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `ListSharedEndpointsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, ListSharedEndpointsErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListSharedEndpointsErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(&self.kind, ListSharedEndpointsErrorKind::ValidationException(_))
    }
}
impl std::error::Error for ListSharedEndpointsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListSharedEndpointsErrorKind::AccessDeniedException(_inner) =>
            Some(_inner)
            ,
            ListSharedEndpointsErrorKind::InternalServerException(_inner) =>
            Some(_inner)
            ,
            ListSharedEndpointsErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            ListSharedEndpointsErrorKind::ValidationException(_inner) =>
            Some(_inner)
            ,
            ListSharedEndpointsErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

