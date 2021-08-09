use graphql_client::reqwest::post_graphql;
use reqwest::Client;

use super::lookup::{get_registrations, GetRegistrations};

pub async fn lookup(label_name: String) {
    let variables = get_registrations::Variables {
        label_name: label_name,
    };

    let client = Client::new();
    let result = post_graphql::<GetRegistrations, _>(
        &client,
        "https://api.thegraph.com/subgraphs/name/ensdomains/ens",
        variables,
    )
    .await
    .unwrap();

    let data: get_registrations::ResponseData = result.data.expect("missing response data");
    println!("{:?}", data);
}

#[cfg(test)]
mod tests {
    use super::lookup;
    use tokio;

    #[tokio::test]
    async fn test_lookup() {
        lookup("pyoner".into()).await;
    }
}
