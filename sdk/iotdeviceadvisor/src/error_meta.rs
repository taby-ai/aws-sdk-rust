// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>Sends a Conflict Exception.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Sends an Internal Failure exception.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>Sends a Resource Not Found exception.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Sends a validation exception.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSuiteDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSuiteDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateSuiteDefinitionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateSuiteDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateSuiteDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSuiteDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSuiteDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteSuiteDefinitionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteSuiteDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteSuiteDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEndpointError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetEndpointError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEndpointErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEndpointErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetEndpointErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetEndpointErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSuiteDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSuiteDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSuiteDefinitionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetSuiteDefinitionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetSuiteDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetSuiteDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSuiteRunError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSuiteRunError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSuiteRunErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetSuiteRunErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetSuiteRunErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetSuiteRunErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSuiteRunReportError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSuiteRunReportError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSuiteRunReportErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetSuiteRunReportErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetSuiteRunReportErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetSuiteRunReportErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSuiteDefinitionsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSuiteDefinitionsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListSuiteDefinitionsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListSuiteDefinitionsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListSuiteDefinitionsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSuiteRunsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSuiteRunsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListSuiteRunsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListSuiteRunsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListSuiteRunsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartSuiteRunError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartSuiteRunError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartSuiteRunErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::StartSuiteRunErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::StartSuiteRunErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::StartSuiteRunErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopSuiteRunError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopSuiteRunError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StopSuiteRunErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::StopSuiteRunErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::StopSuiteRunErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::StopSuiteRunErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::TagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::TagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UntagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UntagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSuiteDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateSuiteDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateSuiteDefinitionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateSuiteDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateSuiteDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

