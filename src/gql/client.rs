use serde::Deserialize;
use serde_json::json;
use crate::{error::AppResult, gql::leetcode_gql};

pub async fn check_user_exists(username: &str) -> AppResult<bool> {
    let query = r#"
        query userProfile($username: String!) {
            matchedUser(username: $username) {
                username
            }
        }
    "#;

    let variables = json!({ "username": username });

    #[derive(Deserialize, Debug)]
    struct GqlResponse {
        data: Option<Data>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Data {
        matched_user: Option<User>,
    }

    #[derive(Deserialize, Debug)]
    struct User {
        username: String,
    }

    let gql: GqlResponse = leetcode_gql(query, variables).await?;


    Ok(gql.data.and_then(|d| d.matched_user).is_some())
}
