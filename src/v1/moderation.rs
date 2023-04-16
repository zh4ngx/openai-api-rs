use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateModerationRequest {
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateModerationResponse {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
}

#[derive(Debug, Deserialize)]
pub struct ModerationResult {
    pub categories: ModerationCategories,
    pub category_scores: ModerationCategoryScores,
    pub flagged: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModerationCategories {
    #[serde(rename = "hate")]
    pub is_hate: bool,
    #[serde(rename = "hate/threatening")]
    pub is_hate_threatening: bool,
    #[serde(rename = "self-harm")]
    pub is_self_harm: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub is_sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub is_violence_graphic: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModerationCategoryScores {
    #[serde(rename = "hate")]
    pub hate_score: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening_score: f64,
    #[serde(rename = "self-harm")]
    pub self_harm_score: f64,
    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors_score: f64,
    pub violence: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic_score: f64,
}