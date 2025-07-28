use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
#[repr(i32)]
pub enum ReviewStage {
    First = 0,
    After8Hours = 1,
    After8Days = 2,
    After16Days = 3,
    After32Days = 4,
    After90Days = 5,
}

impl Default for ReviewStage {
    fn default() -> Self {
        ReviewStage::First
    }
}


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Problem {
    pub id: uuid::Uuid,
    pub title: String,
    pub user_id: uuid::Uuid,
    pub first_solved: DateTime<Utc>,
    pub next_solve_on: DateTime<Utc>,
    pub stage: ReviewStage,
    pub paused: bool,
}

impl Problem {
    pub fn new(id: uuid::Uuid, title: String, user_id: uuid::Uuid, first_solved: DateTime<Utc>) -> Self {
        let mut prob = Self {
            id,
            title,
            user_id,
            first_solved,
            next_solve_on : first_solved,
            stage: ReviewStage::First,
            paused: false,
        };
        prob.advance_stage();
        prob
    }

    pub fn advance_stage(&mut self) {
        self.stage = match self.stage{
            ReviewStage::First => ReviewStage::After8Hours,
            ReviewStage::After8Hours=>ReviewStage::After8Days,
            ReviewStage::After8Days => ReviewStage::After16Days,
            ReviewStage::After16Days => ReviewStage::After32Days,
            ReviewStage::After32Days => ReviewStage::After90Days,
            ReviewStage::After90Days => ReviewStage::After90Days                                                                       
        };

          let delta = match self.stage {
            ReviewStage::After8Hours => Duration::hours(8),
            ReviewStage::After8Days => Duration::days(8),
            ReviewStage::After16Days => Duration::days(16),
            ReviewStage::After32Days => Duration::days(32),
            ReviewStage::After90Days => Duration::days(90),
            ReviewStage::First => Duration::zero(),
        };

        self.next_solve_on = self.first_solved + delta;
    } 
}
