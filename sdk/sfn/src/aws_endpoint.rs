// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
                        .id("aws")
                        .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
        .default_endpoint(
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
        .endpoint("fips-us-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states-fips.us-east-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-east-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-us-east-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states-fips.us-east-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-east-2")
                    .build()
                ,
            }
        )
        .endpoint("fips-us-west-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states-fips.us-west-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-west-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-us-west-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states-fips.us-west-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-west-2")
                    .build()
                ,
            }
        )
        .endpoint("us-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("us-east-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("us-west-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("us-west-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "states.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .build().expect("invalid partition")
        ,
        vec![
            aws_endpoint::Partition::builder()
                            .id("aws-cn")
                            .region_regex(r#"^cn\-\w+\-\d+$"#)
            .default_endpoint(
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.{region}.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .build().expect("invalid partition")
            ,
            aws_endpoint::Partition::builder()
                            .id("aws-iso")
                            .region_regex(r#"^us\-iso\-\w+\-\d+$"#)
            .default_endpoint(
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.{region}.c2s.ic.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .build().expect("invalid partition")
            ,
            aws_endpoint::Partition::builder()
                            .id("aws-iso-b")
                            .region_regex(r#"^us\-isob\-\w+\-\d+$"#)
            .default_endpoint(
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.{region}.sc2s.sgov.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .build().expect("invalid partition")
            ,
            aws_endpoint::Partition::builder()
                            .id("aws-us-gov")
                            .region_regex(r#"^us\-gov\-\w+\-\d+$"#)
            .default_endpoint(
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .endpoint("fips-us-gov-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states-fips.us-gov-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("us-gov-east-1")
                        .build()
                    ,
                }
            )
            .endpoint("fips-us-gov-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.us-gov-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("us-gov-west-1")
                        .build()
                    ,
                }
            )
            .endpoint("us-gov-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .endpoint("us-gov-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "states.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .build().expect("invalid partition")
            ,
        ]
    )
}

