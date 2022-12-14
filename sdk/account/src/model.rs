// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details of the primary contact information associated with an Amazon Web Services account.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ContactInformation  {
    /// <p>The full name of the primary contact address.</p>
    #[doc(hidden)]pub full_name: std::option::Option<std::string::String>,
    /// <p>The first line of the primary contact address.</p>
    #[doc(hidden)]pub address_line1: std::option::Option<std::string::String>,
    /// <p>The second line of the primary contact address, if any.</p>
    #[doc(hidden)]pub address_line2: std::option::Option<std::string::String>,
    /// <p>The third line of the primary contact address, if any.</p>
    #[doc(hidden)]pub address_line3: std::option::Option<std::string::String>,
    /// <p>The city of the primary contact address.</p>
    #[doc(hidden)]pub city: std::option::Option<std::string::String>,
    /// <p>The state or region of the primary contact address. This field is required in selected countries.</p>
    #[doc(hidden)]pub state_or_region: std::option::Option<std::string::String>,
    /// <p>The district or county of the primary contact address, if any.</p>
    #[doc(hidden)]pub district_or_county: std::option::Option<std::string::String>,
    /// <p>The postal code of the primary contact address.</p>
    #[doc(hidden)]pub postal_code: std::option::Option<std::string::String>,
    /// <p>The ISO-3166 two-letter country code for the primary contact address.</p>
    #[doc(hidden)]pub country_code: std::option::Option<std::string::String>,
    /// <p>The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.</p>
    #[doc(hidden)]pub phone_number: std::option::Option<std::string::String>,
    /// <p>The name of the company associated with the primary contact information, if any.</p>
    #[doc(hidden)]pub company_name: std::option::Option<std::string::String>,
    /// <p>The URL of the website associated with the primary contact information, if any.</p>
    #[doc(hidden)]pub website_url: std::option::Option<std::string::String>,
}
impl ContactInformation {
    /// <p>The full name of the primary contact address.</p>
    pub fn full_name(&self) -> std::option::Option<& str> {
        self.full_name.as_deref()
    }
    /// <p>The first line of the primary contact address.</p>
    pub fn address_line1(&self) -> std::option::Option<& str> {
        self.address_line1.as_deref()
    }
    /// <p>The second line of the primary contact address, if any.</p>
    pub fn address_line2(&self) -> std::option::Option<& str> {
        self.address_line2.as_deref()
    }
    /// <p>The third line of the primary contact address, if any.</p>
    pub fn address_line3(&self) -> std::option::Option<& str> {
        self.address_line3.as_deref()
    }
    /// <p>The city of the primary contact address.</p>
    pub fn city(&self) -> std::option::Option<& str> {
        self.city.as_deref()
    }
    /// <p>The state or region of the primary contact address. This field is required in selected countries.</p>
    pub fn state_or_region(&self) -> std::option::Option<& str> {
        self.state_or_region.as_deref()
    }
    /// <p>The district or county of the primary contact address, if any.</p>
    pub fn district_or_county(&self) -> std::option::Option<& str> {
        self.district_or_county.as_deref()
    }
    /// <p>The postal code of the primary contact address.</p>
    pub fn postal_code(&self) -> std::option::Option<& str> {
        self.postal_code.as_deref()
    }
    /// <p>The ISO-3166 two-letter country code for the primary contact address.</p>
    pub fn country_code(&self) -> std::option::Option<& str> {
        self.country_code.as_deref()
    }
    /// <p>The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.</p>
    pub fn phone_number(&self) -> std::option::Option<& str> {
        self.phone_number.as_deref()
    }
    /// <p>The name of the company associated with the primary contact information, if any.</p>
    pub fn company_name(&self) -> std::option::Option<& str> {
        self.company_name.as_deref()
    }
    /// <p>The URL of the website associated with the primary contact information, if any.</p>
    pub fn website_url(&self) -> std::option::Option<& str> {
        self.website_url.as_deref()
    }
}
impl  std::fmt::Debug for ContactInformation  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ContactInformation");
        formatter.field("full_name", &"*** Sensitive Data Redacted ***");
        formatter.field("address_line1", &"*** Sensitive Data Redacted ***");
        formatter.field("address_line2", &"*** Sensitive Data Redacted ***");
        formatter.field("address_line3", &"*** Sensitive Data Redacted ***");
        formatter.field("city", &"*** Sensitive Data Redacted ***");
        formatter.field("state_or_region", &"*** Sensitive Data Redacted ***");
        formatter.field("district_or_county", &"*** Sensitive Data Redacted ***");
        formatter.field("postal_code", &"*** Sensitive Data Redacted ***");
        formatter.field("country_code", &"*** Sensitive Data Redacted ***");
        formatter.field("phone_number", &"*** Sensitive Data Redacted ***");
        formatter.field("company_name", &"*** Sensitive Data Redacted ***");
        formatter.field("website_url", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`ContactInformation`](crate::model::ContactInformation).
pub mod contact_information {
    
    /// A builder for [`ContactInformation`](crate::model::ContactInformation).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) full_name: std::option::Option<std::string::String>,
        pub(crate) address_line1: std::option::Option<std::string::String>,
        pub(crate) address_line2: std::option::Option<std::string::String>,
        pub(crate) address_line3: std::option::Option<std::string::String>,
        pub(crate) city: std::option::Option<std::string::String>,
        pub(crate) state_or_region: std::option::Option<std::string::String>,
        pub(crate) district_or_county: std::option::Option<std::string::String>,
        pub(crate) postal_code: std::option::Option<std::string::String>,
        pub(crate) country_code: std::option::Option<std::string::String>,
        pub(crate) phone_number: std::option::Option<std::string::String>,
        pub(crate) company_name: std::option::Option<std::string::String>,
        pub(crate) website_url: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The full name of the primary contact address.</p>
        pub fn full_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.full_name = Some(input.into());
            self
        }
        /// <p>The full name of the primary contact address.</p>
        pub fn set_full_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.full_name = input; self
        }
        /// <p>The first line of the primary contact address.</p>
        pub fn address_line1(mut self, input: impl Into<std::string::String>) -> Self {
            self.address_line1 = Some(input.into());
            self
        }
        /// <p>The first line of the primary contact address.</p>
        pub fn set_address_line1(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.address_line1 = input; self
        }
        /// <p>The second line of the primary contact address, if any.</p>
        pub fn address_line2(mut self, input: impl Into<std::string::String>) -> Self {
            self.address_line2 = Some(input.into());
            self
        }
        /// <p>The second line of the primary contact address, if any.</p>
        pub fn set_address_line2(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.address_line2 = input; self
        }
        /// <p>The third line of the primary contact address, if any.</p>
        pub fn address_line3(mut self, input: impl Into<std::string::String>) -> Self {
            self.address_line3 = Some(input.into());
            self
        }
        /// <p>The third line of the primary contact address, if any.</p>
        pub fn set_address_line3(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.address_line3 = input; self
        }
        /// <p>The city of the primary contact address.</p>
        pub fn city(mut self, input: impl Into<std::string::String>) -> Self {
            self.city = Some(input.into());
            self
        }
        /// <p>The city of the primary contact address.</p>
        pub fn set_city(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.city = input; self
        }
        /// <p>The state or region of the primary contact address. This field is required in selected countries.</p>
        pub fn state_or_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.state_or_region = Some(input.into());
            self
        }
        /// <p>The state or region of the primary contact address. This field is required in selected countries.</p>
        pub fn set_state_or_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.state_or_region = input; self
        }
        /// <p>The district or county of the primary contact address, if any.</p>
        pub fn district_or_county(mut self, input: impl Into<std::string::String>) -> Self {
            self.district_or_county = Some(input.into());
            self
        }
        /// <p>The district or county of the primary contact address, if any.</p>
        pub fn set_district_or_county(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.district_or_county = input; self
        }
        /// <p>The postal code of the primary contact address.</p>
        pub fn postal_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.postal_code = Some(input.into());
            self
        }
        /// <p>The postal code of the primary contact address.</p>
        pub fn set_postal_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.postal_code = input; self
        }
        /// <p>The ISO-3166 two-letter country code for the primary contact address.</p>
        pub fn country_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.country_code = Some(input.into());
            self
        }
        /// <p>The ISO-3166 two-letter country code for the primary contact address.</p>
        pub fn set_country_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.country_code = input; self
        }
        /// <p>The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.</p>
        pub fn phone_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.phone_number = Some(input.into());
            self
        }
        /// <p>The phone number of the primary contact information. The number will be validated and, in some countries, checked for activation.</p>
        pub fn set_phone_number(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.phone_number = input; self
        }
        /// <p>The name of the company associated with the primary contact information, if any.</p>
        pub fn company_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.company_name = Some(input.into());
            self
        }
        /// <p>The name of the company associated with the primary contact information, if any.</p>
        pub fn set_company_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.company_name = input; self
        }
        /// <p>The URL of the website associated with the primary contact information, if any.</p>
        pub fn website_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.website_url = Some(input.into());
            self
        }
        /// <p>The URL of the website associated with the primary contact information, if any.</p>
        pub fn set_website_url(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.website_url = input; self
        }
        /// Consumes the builder and constructs a [`ContactInformation`](crate::model::ContactInformation).
        pub fn build(self) -> crate::model::ContactInformation {
            crate::model::ContactInformation {
                full_name: self.full_name
                ,
                address_line1: self.address_line1
                ,
                address_line2: self.address_line2
                ,
                address_line3: self.address_line3
                ,
                city: self.city
                ,
                state_or_region: self.state_or_region
                ,
                district_or_county: self.district_or_county
                ,
                postal_code: self.postal_code
                ,
                country_code: self.country_code
                ,
                phone_number: self.phone_number
                ,
                company_name: self.company_name
                ,
                website_url: self.website_url
                ,
            }
        }
    }
    
    
}
impl ContactInformation {
    /// Creates a new builder-style object to manufacture [`ContactInformation`](crate::model::ContactInformation).
    pub fn builder() -> crate::model::contact_information::Builder {
        crate::model::contact_information::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::Eq, std::cmp::Ord, std::cmp::PartialEq, std::cmp::PartialOrd, std::fmt::Debug, std::hash::Hash, )]
pub enum AlternateContactType {
    #[allow(missing_docs)] // documentation missing in model
    Billing,
    #[allow(missing_docs)] // documentation missing in model
    Operations,
    #[allow(missing_docs)] // documentation missing in model
    Security,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String)
}
impl std::convert::From<&str> for AlternateContactType {
    fn from(s: &str) -> Self {
        match s {
            "BILLING" => AlternateContactType::Billing,
            "OPERATIONS" => AlternateContactType::Operations,
            "SECURITY" => AlternateContactType::Security,
            other => AlternateContactType::Unknown(other.to_owned())
        }
    }
}
impl std::str::FromStr for AlternateContactType {
                type Err = std::convert::Infallible;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Ok(AlternateContactType::from(s))
                }
            }
impl AlternateContactType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            AlternateContactType::Billing => "BILLING",
            AlternateContactType::Operations => "OPERATIONS",
            AlternateContactType::Security => "SECURITY",
            AlternateContactType::Unknown(s) => s.as_ref()
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "BILLING", "OPERATIONS", "SECURITY"
        ]
    }
}
impl AsRef<str> for AlternateContactType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A structure that contains the details of an alternate contact associated with an Amazon Web Services account</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct AlternateContact  {
    /// <p>The name associated with this alternate contact.</p>
    #[doc(hidden)]pub name: std::option::Option<std::string::String>,
    /// <p>The title associated with this alternate contact.</p>
    #[doc(hidden)]pub title: std::option::Option<std::string::String>,
    /// <p>The email address associated with this alternate contact.</p>
    #[doc(hidden)]pub email_address: std::option::Option<std::string::String>,
    /// <p>The phone number associated with this alternate contact.</p>
    #[doc(hidden)]pub phone_number: std::option::Option<std::string::String>,
    /// <p>The type of alternate contact.</p>
    #[doc(hidden)]pub alternate_contact_type: std::option::Option<crate::model::AlternateContactType>,
}
impl AlternateContact {
    /// <p>The name associated with this alternate contact.</p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p>The title associated with this alternate contact.</p>
    pub fn title(&self) -> std::option::Option<& str> {
        self.title.as_deref()
    }
    /// <p>The email address associated with this alternate contact.</p>
    pub fn email_address(&self) -> std::option::Option<& str> {
        self.email_address.as_deref()
    }
    /// <p>The phone number associated with this alternate contact.</p>
    pub fn phone_number(&self) -> std::option::Option<& str> {
        self.phone_number.as_deref()
    }
    /// <p>The type of alternate contact.</p>
    pub fn alternate_contact_type(&self) -> std::option::Option<& crate::model::AlternateContactType> {
        self.alternate_contact_type.as_ref()
    }
}
impl  std::fmt::Debug for AlternateContact  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AlternateContact");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("title", &"*** Sensitive Data Redacted ***");
        formatter.field("email_address", &"*** Sensitive Data Redacted ***");
        formatter.field("phone_number", &"*** Sensitive Data Redacted ***");
        formatter.field("alternate_contact_type", &self.alternate_contact_type);
        formatter.finish()
    }
}
/// See [`AlternateContact`](crate::model::AlternateContact).
pub mod alternate_contact {
    
    /// A builder for [`AlternateContact`](crate::model::AlternateContact).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) title: std::option::Option<std::string::String>,
        pub(crate) email_address: std::option::Option<std::string::String>,
        pub(crate) phone_number: std::option::Option<std::string::String>,
        pub(crate) alternate_contact_type: std::option::Option<crate::model::AlternateContactType>,
    }
    impl Builder {
        /// <p>The name associated with this alternate contact.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name associated with this alternate contact.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p>The title associated with this alternate contact.</p>
        pub fn title(mut self, input: impl Into<std::string::String>) -> Self {
            self.title = Some(input.into());
            self
        }
        /// <p>The title associated with this alternate contact.</p>
        pub fn set_title(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.title = input; self
        }
        /// <p>The email address associated with this alternate contact.</p>
        pub fn email_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.email_address = Some(input.into());
            self
        }
        /// <p>The email address associated with this alternate contact.</p>
        pub fn set_email_address(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.email_address = input; self
        }
        /// <p>The phone number associated with this alternate contact.</p>
        pub fn phone_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.phone_number = Some(input.into());
            self
        }
        /// <p>The phone number associated with this alternate contact.</p>
        pub fn set_phone_number(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.phone_number = input; self
        }
        /// <p>The type of alternate contact.</p>
        pub fn alternate_contact_type(mut self, input: crate::model::AlternateContactType) -> Self {
            self.alternate_contact_type = Some(input);
            self
        }
        /// <p>The type of alternate contact.</p>
        pub fn set_alternate_contact_type(mut self, input: std::option::Option<crate::model::AlternateContactType>) -> Self {
            self.alternate_contact_type = input; self
        }
        /// Consumes the builder and constructs a [`AlternateContact`](crate::model::AlternateContact).
        pub fn build(self) -> crate::model::AlternateContact {
            crate::model::AlternateContact {
                name: self.name
                ,
                title: self.title
                ,
                email_address: self.email_address
                ,
                phone_number: self.phone_number
                ,
                alternate_contact_type: self.alternate_contact_type
                ,
            }
        }
    }
    
    
}
impl AlternateContact {
    /// Creates a new builder-style object to manufacture [`AlternateContact`](crate::model::AlternateContact).
    pub fn builder() -> crate::model::alternate_contact::Builder {
        crate::model::alternate_contact::Builder::default()
    }
}

