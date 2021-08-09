#[derive(Debug)]
pub struct GetRegistrations;
pub mod get_registrations {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &str = "GetRegistrations";
    pub const QUERY : & str = "query GetRegistrations($labelName: String!) {\n  registrations(where: { labelName: $labelName }) {\n    id\n    domain {\n      id\n      name\n      labelName\n      owner {\n        id\n      }\n    }\n    registrationDate\n    expiryDate\n    registrant {\n      id\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "labelName")]
        pub label_name: String,
    }
    impl Variables {}
    #[derive(Debug, Deserialize)]
    pub struct ResponseData {
        pub registrations: Vec<GetRegistrationsRegistrations>,
    }
    #[derive(Debug, Deserialize)]
    pub struct GetRegistrationsRegistrations {
        pub id: ID,
        pub domain: Option<GetRegistrationsRegistrationsDomain>,
        #[serde(rename = "registrationDate")]
        pub registration_date: BigInt,
        #[serde(rename = "expiryDate")]
        pub expiry_date: BigInt,
        pub registrant: GetRegistrationsRegistrationsRegistrant,
    }
    #[derive(Debug, Deserialize)]
    pub struct GetRegistrationsRegistrationsDomain {
        pub id: ID,
        pub name: Option<String>,
        #[serde(rename = "labelName")]
        pub label_name: Option<String>,
        pub owner: GetRegistrationsRegistrationsDomainOwner,
    }
    #[derive(Debug, Deserialize)]
    pub struct GetRegistrationsRegistrationsDomainOwner {
        pub id: ID,
    }
    #[derive(Debug, Deserialize)]
    pub struct GetRegistrationsRegistrationsRegistrant {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for GetRegistrations {
    type Variables = get_registrations::Variables;
    type ResponseData = get_registrations::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_registrations::QUERY,
            operation_name: get_registrations::OPERATION_NAME,
        }
    }
}
