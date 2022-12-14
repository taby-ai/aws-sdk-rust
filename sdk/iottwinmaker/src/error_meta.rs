// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>Access is denied.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>A conflict occurred.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>The connector failed.</p>
    ConnectorFailureException(crate::error::ConnectorFailureException),
    /// <p>The connector timed out.</p>
    ConnectorTimeoutException(crate::error::ConnectorTimeoutException),
    /// <p>An unexpected error has occurred.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The resource wasn't found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The service quota was exceeded.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>The rate exceeds the limit.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The number of tags exceeds the limit.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// <p>Failed</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::ConnectorFailureException(inner) => inner.fmt(f),
            Error::ConnectorTimeoutException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchPutPropertyValuesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchPutPropertyValuesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::BatchPutPropertyValuesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::BatchPutPropertyValuesErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::BatchPutPropertyValuesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::BatchPutPropertyValuesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::BatchPutPropertyValuesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateComponentTypeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateComponentTypeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateComponentTypeErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateComponentTypeErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateComponentTypeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateComponentTypeErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateComponentTypeErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateComponentTypeErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateComponentTypeErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateEntityError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateEntityError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateEntityErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateEntityErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateEntityErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateEntityErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateEntityErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateEntityErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateEntityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSceneError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSceneError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateSceneErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateSceneErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateSceneErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateSceneErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateSceneErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateSceneErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateSceneErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateWorkspaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateWorkspaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateWorkspaceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateWorkspaceErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateWorkspaceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateWorkspaceErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateWorkspaceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateWorkspaceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateWorkspaceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteComponentTypeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteComponentTypeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteComponentTypeErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteComponentTypeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteComponentTypeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteComponentTypeErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteComponentTypeErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteComponentTypeErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteEntityError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteEntityError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteEntityErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteEntityErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteEntityErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::DeleteEntityErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteEntityErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteEntityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSceneError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSceneError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteSceneErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteSceneErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteSceneErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteSceneErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteSceneErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteSceneErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteWorkspaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteWorkspaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteWorkspaceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteWorkspaceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteWorkspaceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteWorkspaceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteWorkspaceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteWorkspaceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetComponentTypeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetComponentTypeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetComponentTypeErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetComponentTypeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetComponentTypeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetComponentTypeErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetComponentTypeErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEntityError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetEntityError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEntityErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEntityErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetEntityErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::GetEntityErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetEntityErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetEntityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPropertyValueError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPropertyValueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetPropertyValueErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetPropertyValueErrorKind::ConnectorFailureException(inner) => Error::ConnectorFailureException(inner),
                crate::error::GetPropertyValueErrorKind::ConnectorTimeoutException(inner) => Error::ConnectorTimeoutException(inner),
                crate::error::GetPropertyValueErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetPropertyValueErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetPropertyValueErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetPropertyValueErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetPropertyValueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPropertyValueHistoryError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPropertyValueHistoryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetPropertyValueHistoryErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::ConnectorFailureException(inner) => Error::ConnectorFailureException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::ConnectorTimeoutException(inner) => Error::ConnectorTimeoutException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetPropertyValueHistoryErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSceneError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSceneError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSceneErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetSceneErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetSceneErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetSceneErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetSceneErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetSceneErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetWorkspaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetWorkspaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetWorkspaceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetWorkspaceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetWorkspaceErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::GetWorkspaceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetWorkspaceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetWorkspaceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListComponentTypesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListComponentTypesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListComponentTypesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListComponentTypesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListComponentTypesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListComponentTypesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListComponentTypesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEntitiesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEntitiesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListEntitiesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListEntitiesErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::ListEntitiesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListEntitiesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListEntitiesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListScenesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListScenesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListScenesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListScenesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListScenesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListScenesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListScenesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListTagsForResourceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListWorkspacesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListWorkspacesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListWorkspacesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListWorkspacesErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::ListWorkspacesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListWorkspacesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListWorkspacesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::TagResourceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::TagResourceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
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
                crate::error::UntagResourceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateComponentTypeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateComponentTypeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateComponentTypeErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateComponentTypeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateComponentTypeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateComponentTypeErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::UpdateComponentTypeErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateComponentTypeErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateComponentTypeErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateEntityError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateEntityError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateEntityErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateEntityErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::UpdateEntityErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateEntityErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateEntityErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::UpdateEntityErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateEntityErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateEntityErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSceneError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateSceneError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateSceneErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateSceneErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateSceneErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateSceneErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateSceneErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateSceneErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateWorkspaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateWorkspaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateWorkspaceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateWorkspaceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateWorkspaceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateWorkspaceErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::UpdateWorkspaceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateWorkspaceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateWorkspaceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

