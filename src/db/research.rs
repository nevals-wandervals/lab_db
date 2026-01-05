use crate::db::{Id, analysis_results::AnalysisResult};

#[derive(Debug, Clone)]
pub struct Research {
    pub id: Id,
    pub date_time: chrono::DateTime<chrono::Utc>,
    pub analysis: Vec<AnalysisResult>,
}
