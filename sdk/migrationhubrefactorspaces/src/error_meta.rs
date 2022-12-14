// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>The user does not have sufficient access to perform this action. </p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>An unexpected error occurred while processing the request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The resource policy is not valid.</p>
    InvalidResourcePolicyException(crate::error::InvalidResourcePolicyException),
    /// <p>The request references a resource that does not exist. </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request would cause a service quota to be exceeded. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>Request was denied because the request was throttled. </p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The input does not satisfy the constraints specified by an Amazon Web Service. </p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidResourcePolicyException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateApplicationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateApplicationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateApplicationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateApplicationErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateApplicationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateApplicationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::CreateApplicationErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateApplicationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateApplicationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateApplicationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateEnvironmentErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateEnvironmentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::CreateEnvironmentErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateEnvironmentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateRouteError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateRouteError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateRouteErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateRouteErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateRouteErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateRouteErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::CreateRouteErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateRouteErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateRouteErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateRouteErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateServiceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateServiceErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateServiceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::CreateServiceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::CreateServiceErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateServiceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::CreateServiceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateServiceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteApplicationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteApplicationErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::DeleteApplicationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteApplicationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteApplicationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteApplicationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteApplicationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteEnvironmentErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::DeleteEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteEnvironmentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteEnvironmentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteResourcePolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteResourcePolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteResourcePolicyErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteResourcePolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteResourcePolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteResourcePolicyErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteResourcePolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteResourcePolicyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteRouteError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteRouteError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteRouteErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteRouteErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::DeleteRouteErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteRouteErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteRouteErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteRouteErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteRouteErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteServiceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteServiceErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::DeleteServiceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteServiceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteServiceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteServiceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteServiceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetApplicationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetApplicationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetApplicationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetApplicationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetApplicationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetApplicationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetApplicationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetApplicationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetEnvironmentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetEnvironmentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetResourcePolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetResourcePolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetResourcePolicyErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetResourcePolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetResourcePolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetResourcePolicyErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetResourcePolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetResourcePolicyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRouteError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRouteError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetRouteErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetRouteErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetRouteErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetRouteErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetRouteErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetRouteErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetServiceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetServiceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetServiceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetServiceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetServiceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetServiceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListApplicationsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListApplicationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListApplicationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListApplicationsErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::ListApplicationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListApplicationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListApplicationsErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::ListApplicationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListApplicationsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListApplicationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEnvironmentsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEnvironmentsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListEnvironmentsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListEnvironmentsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListEnvironmentsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListEnvironmentsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListEnvironmentsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListEnvironmentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEnvironmentVpcsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEnvironmentVpcsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListEnvironmentVpcsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListEnvironmentVpcsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListEnvironmentVpcsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListEnvironmentVpcsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListEnvironmentVpcsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListEnvironmentVpcsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRoutesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRoutesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListRoutesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListRoutesErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::ListRoutesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListRoutesErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListRoutesErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::ListRoutesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListRoutesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListRoutesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListServicesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListServicesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListServicesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListServicesErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::ListServicesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListServicesErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListServicesErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::ListServicesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListServicesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListServicesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutResourcePolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutResourcePolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PutResourcePolicyErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::PutResourcePolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::PutResourcePolicyErrorKind::InvalidResourcePolicyException(inner) => Error::InvalidResourcePolicyException(inner),
                crate::error::PutResourcePolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::PutResourcePolicyErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::PutResourcePolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::PutResourcePolicyErrorKind::Unhandled(inner) => Error::Unhandled(inner),
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateRouteError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateRouteError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateRouteErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateRouteErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateRouteErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateRouteErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateRouteErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateRouteErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

