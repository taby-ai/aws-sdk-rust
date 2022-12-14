// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
                        .id("aws")
                        .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
        .default_endpoint(
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.{region}.amazonaws.com",
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
                uri_template: "waf-regional.af-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("af-south-1")
                    .build()
                ,
            }
        )
        .endpoint("ap-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-east-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-east-1")
                    .build()
                ,
            }
        )
        .endpoint("ap-northeast-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-northeast-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-1")
                    .build()
                ,
            }
        )
        .endpoint("ap-northeast-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-northeast-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-2")
                    .build()
                ,
            }
        )
        .endpoint("ap-northeast-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-northeast-3.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-3")
                    .build()
                ,
            }
        )
        .endpoint("ap-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-south-1")
                    .build()
                ,
            }
        )
        .endpoint("ap-southeast-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-southeast-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-1")
                    .build()
                ,
            }
        )
        .endpoint("ap-southeast-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-southeast-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-2")
                    .build()
                ,
            }
        )
        .endpoint("ap-southeast-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ap-southeast-3.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-3")
                    .build()
                ,
            }
        )
        .endpoint("ca-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.ca-central-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ca-central-1")
                    .build()
                ,
            }
        )
        .endpoint("eu-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.eu-central-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-central-1")
                    .build()
                ,
            }
        )
        .endpoint("eu-north-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.eu-north-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-north-1")
                    .build()
                ,
            }
        )
        .endpoint("eu-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.eu-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-south-1")
                    .build()
                ,
            }
        )
        .endpoint("eu-west-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.eu-west-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-west-1")
                    .build()
                ,
            }
        )
        .endpoint("eu-west-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.eu-west-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-west-2")
                    .build()
                ,
            }
        )
        .endpoint("eu-west-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.eu-west-3.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-west-3")
                    .build()
                ,
            }
        )
        .endpoint("fips-af-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.af-south-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.ap-east-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.ap-northeast-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.ap-northeast-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-2")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-northeast-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.ap-northeast-3.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-northeast-3")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.ap-south-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.ap-southeast-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.ap-southeast-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-2")
                    .build()
                ,
            }
        )
        .endpoint("fips-ap-southeast-3",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.ap-southeast-3.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("ap-southeast-3")
                    .build()
                ,
            }
        )
        .endpoint("fips-ca-central-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.ca-central-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.eu-central-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-central-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-north-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.eu-north-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("eu-north-1")
                    .build()
                ,
            }
        )
        .endpoint("fips-eu-south-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional-fips.eu-south-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.eu-west-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.eu-west-2.amazonaws.com",
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
                uri_template: "waf-regional-fips.eu-west-3.amazonaws.com",
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
                uri_template: "waf-regional-fips.me-south-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.sa-east-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.us-east-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.us-east-2.amazonaws.com",
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
                uri_template: "waf-regional-fips.us-west-1.amazonaws.com",
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
                uri_template: "waf-regional-fips.us-west-2.amazonaws.com",
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
                uri_template: "waf-regional.me-south-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("me-south-1")
                    .build()
                ,
            }
        )
        .endpoint("sa-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.sa-east-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("sa-east-1")
                    .build()
                ,
            }
        )
        .endpoint("us-east-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.us-east-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-east-1")
                    .build()
                ,
            }
        )
        .endpoint("us-east-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.us-east-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-east-2")
                    .build()
                ,
            }
        )
        .endpoint("us-west-1",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.us-west-1.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-west-1")
                    .build()
                ,
            }
        )
        .endpoint("us-west-2",
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "waf-regional.us-west-2.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope:
                    aws_endpoint::CredentialScope::builder()
                    .region("us-west-2")
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
                    uri_template: "waf-regional.{region}.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .build()
                    ,
                }
            )
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .endpoint("cn-north-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "waf-regional.cn-north-1.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("cn-north-1")
                        .build()
                    ,
                }
            )
            .endpoint("cn-northwest-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "waf-regional.cn-northwest-1.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("cn-northwest-1")
                        .build()
                    ,
                }
            )
            .endpoint("fips-cn-north-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "waf-regional-fips.cn-north-1.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("cn-north-1")
                        .build()
                    ,
                }
            )
            .endpoint("fips-cn-northwest-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "waf-regional-fips.cn-northwest-1.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("cn-northwest-1")
                        .build()
                    ,
                }
            )
            .build().expect("invalid partition")
            ,
            aws_endpoint::Partition::builder()
                            .id("aws-iso")
                            .region_regex(r#"^us\-iso\-\w+\-\d+$"#)
            .default_endpoint(
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "waf-regional.{region}.c2s.ic.gov",
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
                    uri_template: "waf-regional.{region}.sc2s.sgov.gov",
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
                    uri_template: "waf-regional.{region}.amazonaws.com",
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
                    uri_template: "waf-regional-fips.us-gov-east-1.amazonaws.com",
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
                    uri_template: "waf-regional-fips.us-gov-west-1.amazonaws.com",
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
                    uri_template: "waf-regional.us-gov-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("us-gov-east-1")
                        .build()
                    ,
                }
            )
            .endpoint("us-gov-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "waf-regional.us-gov-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope:
                        aws_endpoint::CredentialScope::builder()
                        .region("us-gov-west-1")
                        .build()
                    ,
                }
            )
            .build().expect("invalid partition")
            ,
        ]
    )
}

