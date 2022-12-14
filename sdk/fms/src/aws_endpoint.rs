// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
                        .id("aws")
                        .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
        .default_endpoint(
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
        .endpoint("af-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ap-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ap-northeast-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ap-northeast-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ap-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ap-southeast-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ap-southeast-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("ca-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("eu-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("eu-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("eu-west-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("eu-west-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("eu-west-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("fips-af-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.af-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("af-south-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ap-east-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-east-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-northeast-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ap-northeast-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-northeast-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ap-northeast-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-2")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ap-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-south-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-southeast-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ap-southeast-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-southeast-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ap-southeast-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-2")
                    .build()
                ,
            }
        )
        .endpoint("fips-ca-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.ca-central-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ca-central-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.eu-central-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-central-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.eu-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-south-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-west-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.eu-west-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-west-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-west-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.eu-west-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-west-2")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-west-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.eu-west-3.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-west-3")
                    .build()
                ,
            }
        )
        .endpoint("fips-me-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.me-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("me-south-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-sa-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.sa-east-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("sa-east-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-us-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms-fips.us-east-1.amazonaws.com",
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
                uri_template: "fms-fips.us-east-2.amazonaws.com",
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
                uri_template: "fms-fips.us-west-1.amazonaws.com",
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
                uri_template: "fms-fips.us-west-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-west-2")
                    .build()
                ,
            }
        )
        .endpoint("me-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("sa-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .build()
                ,
            }
        )
        .endpoint("us-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "fms.{region}.amazonaws.com",
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
                uri_template: "fms.{region}.amazonaws.com",
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
                uri_template: "fms.{region}.amazonaws.com",
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
                uri_template: "fms.{region}.amazonaws.com",
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
                    uri_template: "fms.{region}.amazonaws.com.cn",
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
                    uri_template: "fms.{region}.c2s.ic.gov",
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
                    uri_template: "fms.{region}.sc2s.sgov.gov",
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
                    uri_template: "fms.{region}.amazonaws.com",
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
                    uri_template: "fms-fips.us-gov-east-1.amazonaws.com",
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
                    uri_template: "fms-fips.us-gov-west-1.amazonaws.com",
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
                    uri_template: "fms.{region}.amazonaws.com",
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
                    uri_template: "fms.{region}.amazonaws.com",
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

