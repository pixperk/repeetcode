use std::collections::HashSet;
use sqlx::PgPool;

use crate::{domain::{Problem, ReviewStage}, gql::recent_problems::fetch_recent_solved_from_leetcode};

pub async fn sync_user_problems(db : &PgPool, user_id : uuid::Uuid, username : &str, limit: Option<usize>) -> Result<(), sqlx::Error> {
    let existing_problems: HashSet<String> = sqlx::query_scalar!(
        "SELECT problem_slug FROM tracked_problems WHERE user_id = $1",
        user_id
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .collect();

    let solves = match fetch_recent_solved_from_leetcode(username, limit).await{
        Ok(solved) => solved,
        Err(e) => {
            eprintln!("Error fetching recent solved problems: {}", e);
            return Err(sqlx::Error::RowNotFound);
        }
    };

 
    for solve in solves{
       
        if !existing_problems.contains(&solve.id){
            println!("Syncing problem: {}", solve.id);
            let problem = Problem::new(
                solve.id.clone(),
                user_id,
                solve.timestamp,
                solve.title.clone(),
            );

             match sqlx::query!(
                r#"
                INSERT INTO tracked_problems (problem_slug, title, user_id, first_solve, next_solve_on, stage, paused)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,

                problem.problem_slug,
                problem.title,
                problem.user_id,
                problem.first_solved,
                problem.next_solve_on,
                problem.stage as ReviewStage,
                problem.paused,
            )
            .execute(db)
            .await{
                Ok(_) => {
                    tracing::info!("Problem {} synced for user {}", problem.problem_slug, user_id);
                },
                Err(e) => {
                    tracing::error!("Failed to insert problem {}: {}", problem.problem_slug, e);
                    return Err(e);
                }
            };
        }
    }


    Ok(())
}