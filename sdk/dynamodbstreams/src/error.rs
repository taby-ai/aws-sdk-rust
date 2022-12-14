// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ResourceNotFoundException  {
    /// <p>The resource which is being requested does not exist.</p>
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
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
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
        /// <p>The resource which is being requested does not exist.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>The resource which is being requested does not exist.</p>
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

/// <p>An error occurred on the server side.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct InternalServerError  {
    /// <p>The server encountered an internal error trying to fulfill the request.</p>
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for InternalServerError  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerError {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerError")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerError {}
/// See [`InternalServerError`](crate::error::InternalServerError).
pub mod internal_server_error {
    
    /// A builder for [`InternalServerError`](crate::error::InternalServerError).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The server encountered an internal error trying to fulfill the request.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>The server encountered an internal error trying to fulfill the request.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InternalServerError`](crate::error::InternalServerError).
        pub fn build(self) -> crate::error::InternalServerError {
            crate::error::InternalServerError {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InternalServerError {
    /// Creates a new builder-style object to manufacture [`InternalServerError`](crate::error::InternalServerError).
    pub fn builder() -> crate::error::internal_server_error::Builder {
        crate::error::internal_server_error::Builder::default()
    }
}

/// <p>The operation attempted to read past the oldest stream record in a shard.</p> 
/// <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> 
/// <ul> 
/// <li> <p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> 
/// <li> <p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> 
/// </ul>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct TrimmedDataAccessException  {
    /// <p>"The data you are trying to access has been trimmed.</p>
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for TrimmedDataAccessException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TrimmedDataAccessException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl TrimmedDataAccessException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for TrimmedDataAccessException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrimmedDataAccessException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for TrimmedDataAccessException {}
/// See [`TrimmedDataAccessException`](crate::error::TrimmedDataAccessException).
pub mod trimmed_data_access_exception {
    
    /// A builder for [`TrimmedDataAccessException`](crate::error::TrimmedDataAccessException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>"The data you are trying to access has been trimmed.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>"The data you are trying to access has been trimmed.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`TrimmedDataAccessException`](crate::error::TrimmedDataAccessException).
        pub fn build(self) -> crate::error::TrimmedDataAccessException {
            crate::error::TrimmedDataAccessException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl TrimmedDataAccessException {
    /// Creates a new builder-style object to manufacture [`TrimmedDataAccessException`](crate::error::TrimmedDataAccessException).
    pub fn builder() -> crate::error::trimmed_data_access_exception::Builder {
        crate::error::trimmed_data_access_exception::Builder::default()
    }
}

/// <p>There is no limit to the number of daily on-demand backups that can be taken. </p> 
/// <p>Up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> 
/// <p>The only exception is when you are creating a table with one or more secondary indexes. You can have up to 250 such requests running at a time; however, if the table or index specifications are complex, DynamoDB might temporarily reduce the number of concurrent operations.</p> 
/// <p>There is a soft account quota of 2,500 tables.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct LimitExceededException  {
    /// <p>Too many operations for a given subscriber.</p>
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for LimitExceededException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("LimitExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl LimitExceededException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for LimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LimitExceededException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for LimitExceededException {}
/// See [`LimitExceededException`](crate::error::LimitExceededException).
pub mod limit_exceeded_exception {
    
    /// A builder for [`LimitExceededException`](crate::error::LimitExceededException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Too many operations for a given subscriber.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>Too many operations for a given subscriber.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`LimitExceededException`](crate::error::LimitExceededException).
        pub fn build(self) -> crate::error::LimitExceededException {
            crate::error::LimitExceededException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl LimitExceededException {
    /// Creates a new builder-style object to manufacture [`LimitExceededException`](crate::error::LimitExceededException).
    pub fn builder() -> crate::error::limit_exceeded_exception::Builder {
        crate::error::limit_exceeded_exception::Builder::default()
    }
}

/// <p>The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the <code>GetShardIterator</code> action.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ExpiredIteratorException  {
    /// <p>The provided iterator exceeds the maximum age allowed.</p>
    #[doc(hidden)]pub message: std::option::Option<std::string::String>,
}
impl  std::fmt::Debug for ExpiredIteratorException  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ExpiredIteratorException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ExpiredIteratorException {
    /// Returns the error message.
                        pub fn message(&self) -> Option<&str> { self.message.as_deref() }
}
impl std::fmt::Display for ExpiredIteratorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExpiredIteratorException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for ExpiredIteratorException {}
/// See [`ExpiredIteratorException`](crate::error::ExpiredIteratorException).
pub mod expired_iterator_exception {
    
    /// A builder for [`ExpiredIteratorException`](crate::error::ExpiredIteratorException).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The provided iterator exceeds the maximum age allowed.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        /// <p>The provided iterator exceeds the maximum age allowed.</p>
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ExpiredIteratorException`](crate::error::ExpiredIteratorException).
        pub fn build(self) -> crate::error::ExpiredIteratorException {
            crate::error::ExpiredIteratorException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl ExpiredIteratorException {
    /// Creates a new builder-style object to manufacture [`ExpiredIteratorException`](crate::error::ExpiredIteratorException).
    pub fn builder() -> crate::error::expired_iterator_exception::Builder {
        crate::error::expired_iterator_exception::Builder::default()
    }
}

/// Error type for the `DescribeStream` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct DescribeStreamError {
    /// Kind of error that occurred.
                    pub kind: DescribeStreamErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `DescribeStream` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum DescribeStreamErrorKind {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeStreamErrorKind::InternalServerError(_inner) =>
            _inner.fmt(f)
            ,
            DescribeStreamErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            DescribeStreamErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeStreamError {
    fn code(&self) -> Option<&str> {
        DescribeStreamError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeStreamError {
    /// Creates a new `DescribeStreamError`.
                    pub fn new(kind: DescribeStreamErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `DescribeStreamError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: DescribeStreamErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `DescribeStreamError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: DescribeStreamErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `DescribeStreamErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, DescribeStreamErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `DescribeStreamErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, DescribeStreamErrorKind::ResourceNotFoundException(_))
    }
}
impl std::error::Error for DescribeStreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeStreamErrorKind::InternalServerError(_inner) =>
            Some(_inner)
            ,
            DescribeStreamErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            DescribeStreamErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `GetRecords` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct GetRecordsError {
    /// Kind of error that occurred.
                    pub kind: GetRecordsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `GetRecords` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum GetRecordsErrorKind {
    /// <p>The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the <code>GetShardIterator</code> action.</p>
    ExpiredIteratorException(crate::error::ExpiredIteratorException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>There is no limit to the number of daily on-demand backups that can be taken. </p> 
    /// <p>Up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p> 
    /// <p>The only exception is when you are creating a table with one or more secondary indexes. You can have up to 250 such requests running at a time; however, if the table or index specifications are complex, DynamoDB might temporarily reduce the number of concurrent operations.</p> 
    /// <p>There is a soft account quota of 2,500 tables.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation attempted to read past the oldest stream record in a shard.</p> 
    /// <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> 
    /// <ul> 
    /// <li> <p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> 
    /// <li> <p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> 
    /// </ul>
    TrimmedDataAccessException(crate::error::TrimmedDataAccessException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetRecordsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetRecordsErrorKind::ExpiredIteratorException(_inner) =>
            _inner.fmt(f)
            ,
            GetRecordsErrorKind::InternalServerError(_inner) =>
            _inner.fmt(f)
            ,
            GetRecordsErrorKind::LimitExceededException(_inner) =>
            _inner.fmt(f)
            ,
            GetRecordsErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            GetRecordsErrorKind::TrimmedDataAccessException(_inner) =>
            _inner.fmt(f)
            ,
            GetRecordsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetRecordsError {
    fn code(&self) -> Option<&str> {
        GetRecordsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetRecordsError {
    /// Creates a new `GetRecordsError`.
                    pub fn new(kind: GetRecordsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `GetRecordsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: GetRecordsErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `GetRecordsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: GetRecordsErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `GetRecordsErrorKind::ExpiredIteratorException`.
    pub fn is_expired_iterator_exception(&self) -> bool {
        matches!(&self.kind, GetRecordsErrorKind::ExpiredIteratorException(_))
    }
    /// Returns `true` if the error kind is `GetRecordsErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, GetRecordsErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `GetRecordsErrorKind::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(&self.kind, GetRecordsErrorKind::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `GetRecordsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, GetRecordsErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `GetRecordsErrorKind::TrimmedDataAccessException`.
    pub fn is_trimmed_data_access_exception(&self) -> bool {
        matches!(&self.kind, GetRecordsErrorKind::TrimmedDataAccessException(_))
    }
}
impl std::error::Error for GetRecordsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetRecordsErrorKind::ExpiredIteratorException(_inner) =>
            Some(_inner)
            ,
            GetRecordsErrorKind::InternalServerError(_inner) =>
            Some(_inner)
            ,
            GetRecordsErrorKind::LimitExceededException(_inner) =>
            Some(_inner)
            ,
            GetRecordsErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            GetRecordsErrorKind::TrimmedDataAccessException(_inner) =>
            Some(_inner)
            ,
            GetRecordsErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `GetShardIterator` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct GetShardIteratorError {
    /// Kind of error that occurred.
                    pub kind: GetShardIteratorErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `GetShardIterator` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum GetShardIteratorErrorKind {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation attempted to read past the oldest stream record in a shard.</p> 
    /// <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p> 
    /// <ul> 
    /// <li> <p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li> 
    /// <li> <p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li> 
    /// </ul>
    TrimmedDataAccessException(crate::error::TrimmedDataAccessException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetShardIteratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetShardIteratorErrorKind::InternalServerError(_inner) =>
            _inner.fmt(f)
            ,
            GetShardIteratorErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            GetShardIteratorErrorKind::TrimmedDataAccessException(_inner) =>
            _inner.fmt(f)
            ,
            GetShardIteratorErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetShardIteratorError {
    fn code(&self) -> Option<&str> {
        GetShardIteratorError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetShardIteratorError {
    /// Creates a new `GetShardIteratorError`.
                    pub fn new(kind: GetShardIteratorErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `GetShardIteratorError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: GetShardIteratorErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `GetShardIteratorError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: GetShardIteratorErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `GetShardIteratorErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, GetShardIteratorErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `GetShardIteratorErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, GetShardIteratorErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `GetShardIteratorErrorKind::TrimmedDataAccessException`.
    pub fn is_trimmed_data_access_exception(&self) -> bool {
        matches!(&self.kind, GetShardIteratorErrorKind::TrimmedDataAccessException(_))
    }
}
impl std::error::Error for GetShardIteratorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetShardIteratorErrorKind::InternalServerError(_inner) =>
            Some(_inner)
            ,
            GetShardIteratorErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            GetShardIteratorErrorKind::TrimmedDataAccessException(_inner) =>
            Some(_inner)
            ,
            GetShardIteratorErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `ListStreams` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct ListStreamsError {
    /// Kind of error that occurred.
                    pub kind: ListStreamsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `ListStreams` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum ListStreamsErrorKind {
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListStreamsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListStreamsErrorKind::InternalServerError(_inner) =>
            _inner.fmt(f)
            ,
            ListStreamsErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            ListStreamsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListStreamsError {
    fn code(&self) -> Option<&str> {
        ListStreamsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListStreamsError {
    /// Creates a new `ListStreamsError`.
                    pub fn new(kind: ListStreamsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `ListStreamsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: ListStreamsErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `ListStreamsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: ListStreamsErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `ListStreamsErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, ListStreamsErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `ListStreamsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, ListStreamsErrorKind::ResourceNotFoundException(_))
    }
}
impl std::error::Error for ListStreamsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListStreamsErrorKind::InternalServerError(_inner) =>
            Some(_inner)
            ,
            ListStreamsErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            ListStreamsErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

