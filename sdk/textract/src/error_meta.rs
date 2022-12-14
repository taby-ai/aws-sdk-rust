// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>You aren't authorized to perform the action. Use the Amazon Resource Name (ARN) of an authorized user or IAM role to perform the operation.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Amazon Textract isn't able to read the document. For more information on the document limits in Amazon Textract, see <code>limits</code>.</p>
    BadDocumentException(crate::error::BadDocumentException),
    /// <p>The document can't be processed because it's too large. The maximum document size for synchronous operations 10 MB. The maximum document size for asynchronous operations is 500 MB for PDF files.</p>
    DocumentTooLargeException(crate::error::DocumentTooLargeException),
    /// <p>Indicates you have exceeded the maximum number of active human in the loop workflows available</p>
    HumanLoopQuotaExceededException(crate::error::HumanLoopQuotaExceededException),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation. </p>
    IdempotentParameterMismatchException(crate::error::IdempotentParameterMismatchException),
    /// <p>Amazon Textract experienced a service issue. Try your call again.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>An invalid job identifier was passed to <code>GetDocumentAnalysis</code> or to <code>GetDocumentAnalysis</code>.</p>
    InvalidJobIdException(crate::error::InvalidJobIdException),
    /// <p> Indicates you do not have decrypt permissions with the KMS key entered, or the KMS key was entered incorrectly. </p>
    InvalidKmsKeyException(crate::error::InvalidKmsKeyException),
    /// <p>An input parameter violated a constraint. For example, in synchronous operations, an <code>InvalidParameterException</code> exception occurs when neither of the <code>S3Object</code> or <code>Bytes</code> values are supplied in the <code>Document</code> request parameter. Validate your parameter before calling the API operation again.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>Amazon Textract is unable to access the S3 object that's specified in the request. for more information, <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-access-control.html">Configure Access to Amazon S3</a> For troubleshooting information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/troubleshooting.html">Troubleshooting Amazon S3</a> </p>
    InvalidS3ObjectException(crate::error::InvalidS3ObjectException),
    /// <p>An Amazon Textract service limit was exceeded. For example, if you start too many asynchronous jobs concurrently, calls to start operations (<code>StartDocumentTextDetection</code>, for example) raise a LimitExceededException exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Textract service limit. </p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Textract.</p>
    ProvisionedThroughputExceededException(crate::error::ProvisionedThroughputExceededException),
    /// <p>Amazon Textract is temporarily unable to process the request. Try your call again.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The format of the input document isn't supported. Documents for operations can be in PNG, JPEG, PDF, or TIFF format.</p>
    UnsupportedDocumentException(crate::error::UnsupportedDocumentException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::BadDocumentException(inner) => inner.fmt(f),
            Error::DocumentTooLargeException(inner) => inner.fmt(f),
            Error::HumanLoopQuotaExceededException(inner) => inner.fmt(f),
            Error::IdempotentParameterMismatchException(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::InvalidJobIdException(inner) => inner.fmt(f),
            Error::InvalidKmsKeyException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::InvalidS3ObjectException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ProvisionedThroughputExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::UnsupportedDocumentException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AnalyzeDocumentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AnalyzeDocumentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AnalyzeDocumentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::AnalyzeDocumentErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::AnalyzeDocumentErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::AnalyzeDocumentErrorKind::HumanLoopQuotaExceededException(inner) => Error::HumanLoopQuotaExceededException(inner),
                crate::error::AnalyzeDocumentErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::AnalyzeDocumentErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::AnalyzeDocumentErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::AnalyzeDocumentErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::AnalyzeDocumentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::AnalyzeDocumentErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::AnalyzeDocumentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AnalyzeExpenseError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AnalyzeExpenseError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AnalyzeExpenseErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::AnalyzeExpenseErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::AnalyzeExpenseErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::AnalyzeExpenseErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::AnalyzeExpenseErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::AnalyzeExpenseErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::AnalyzeExpenseErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::AnalyzeExpenseErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::AnalyzeExpenseErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::AnalyzeExpenseErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AnalyzeIDError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AnalyzeIDError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AnalyzeIDErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::AnalyzeIDErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::AnalyzeIDErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::AnalyzeIDErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::AnalyzeIDErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::AnalyzeIDErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::AnalyzeIDErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::AnalyzeIDErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::AnalyzeIDErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::AnalyzeIDErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DetectDocumentTextError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DetectDocumentTextError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DetectDocumentTextErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DetectDocumentTextErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::DetectDocumentTextErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::DetectDocumentTextErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::DetectDocumentTextErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DetectDocumentTextErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::DetectDocumentTextErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::DetectDocumentTextErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DetectDocumentTextErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::DetectDocumentTextErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDocumentAnalysisError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDocumentAnalysisError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetDocumentAnalysisErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidJobIdException(inner) => Error::InvalidJobIdException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::GetDocumentAnalysisErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::GetDocumentAnalysisErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetDocumentAnalysisErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDocumentTextDetectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDocumentTextDetectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetDocumentTextDetectionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidJobIdException(inner) => Error::InvalidJobIdException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetExpenseAnalysisError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetExpenseAnalysisError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetExpenseAnalysisErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetExpenseAnalysisErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::GetExpenseAnalysisErrorKind::InvalidJobIdException(inner) => Error::InvalidJobIdException(inner),
                crate::error::GetExpenseAnalysisErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::GetExpenseAnalysisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetExpenseAnalysisErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::GetExpenseAnalysisErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::GetExpenseAnalysisErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetExpenseAnalysisErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartDocumentAnalysisError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartDocumentAnalysisError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartDocumentAnalysisErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::StartDocumentAnalysisErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::StartDocumentAnalysisErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::StartDocumentAnalysisErrorKind::IdempotentParameterMismatchException(inner) => Error::IdempotentParameterMismatchException(inner),
                crate::error::StartDocumentAnalysisErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::StartDocumentAnalysisErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::StartDocumentAnalysisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartDocumentAnalysisErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::StartDocumentAnalysisErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartDocumentAnalysisErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::StartDocumentAnalysisErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::StartDocumentAnalysisErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::StartDocumentAnalysisErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartDocumentTextDetectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartDocumentTextDetectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartDocumentTextDetectionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::IdempotentParameterMismatchException(inner) => Error::IdempotentParameterMismatchException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartExpenseAnalysisError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartExpenseAnalysisError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartExpenseAnalysisErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::StartExpenseAnalysisErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::StartExpenseAnalysisErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::StartExpenseAnalysisErrorKind::IdempotentParameterMismatchException(inner) => Error::IdempotentParameterMismatchException(inner),
                crate::error::StartExpenseAnalysisErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::StartExpenseAnalysisErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::StartExpenseAnalysisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartExpenseAnalysisErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::StartExpenseAnalysisErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartExpenseAnalysisErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::StartExpenseAnalysisErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::StartExpenseAnalysisErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::StartExpenseAnalysisErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

