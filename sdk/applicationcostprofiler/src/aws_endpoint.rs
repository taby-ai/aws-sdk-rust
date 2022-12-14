// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
                        .id("aws")
                        .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
        .default_endpoint(
            aws_endpoint::partition::endpoint::Metadata {
                uri_template: "application-cost-profiler.{region}.amazonaws.com",
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
        vec![
            aws_endpoint::Partition::builder()
                            .id("aws-cn")
                            .region_regex(r#"^cn\-\w+\-\d+$"#)
            .default_endpoint(
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "application-cost-profiler.{region}.amazonaws.com.cn",
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
                    uri_template: "application-cost-profiler.{region}.c2s.ic.gov",
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
                    uri_template: "application-cost-profiler.{region}.sc2s.sgov.gov",
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
                    uri_template: "application-cost-profiler.{region}.amazonaws.com",
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
        ]
    )
}

