use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

use crate::{error::AppResult, gql::leetcode_gql};

#[derive(Debug, Deserialize)]
pub struct RecentSubmissionListResponse {
    pub data: RecentSubmissionData,
}

#[derive(Debug, Deserialize)]
pub struct RecentSubmissionData {
    pub recentSubmissionList: Vec<RecentSubmission>,
}

#[derive(Debug, Deserialize)]
pub struct RecentSubmission {
    pub title: String,
    pub titleSlug: String,
    pub timestamp: String, 
    pub statusDisplay: String,
}

#[derive(Debug, Clone)]
pub struct SolvedProblem {
    pub id: String,           // titleSlug
    pub title: String,
    pub timestamp: DateTime<Utc>,
}

pub async fn fetch_recent_solved_from_leetcode(
    username: &str,
    limit : Option<usize>,
) -> AppResult<Vec<SolvedProblem>> {
    let query = r#"
        query getRecentSubmissions($username: String!, $limit: Int) {
            recentSubmissionList(username: $username, limit: $limit) {
                title
                titleSlug
                timestamp
                statusDisplay
            }
        }
    "#;

    let res: RecentSubmissionListResponse = leetcode_gql(
        query,
        json!({
            "username": username,
            "limit": limit.map(|l| l as i32),
        }),
    )
    .await?;

    let solved: Vec<SolvedProblem> = res
        .data
        .recentSubmissionList
        .into_iter()
        /* .filter(|sub| sub.statusDisplay == "Accepted") */
        .filter_map(|sub| {
            let timestamp = sub.timestamp.parse::<i64>().ok()?;
            Some(SolvedProblem {
                id: sub.titleSlug,
                title: sub.title,
                timestamp: DateTime::<Utc>::from_timestamp(timestamp, 0)?,
            })
        })
        .collect();

    println!("Fetched {} solved problems for user {}", solved.len(), username);
    if let Some(limit) = limit {
        println!("Limit applied: {}", limit);
    } else {
        println!("No limit applied");
    }

    Ok(solved)
}