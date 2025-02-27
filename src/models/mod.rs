use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct FailureRuntimeReturn {
    pub passed: bool,
    pub fail_report: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct PassedRuntimeReturn {
    pub passed: bool,
}

#[derive(Serialize, Deserialize)]
pub enum ModelType {
    LinearRegression,
    LogisiticRegression,
    BinaryClassification,
}

impl TryFrom<&str> for ModelType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "LinearRegression" => Ok(Self::LinearRegression),
            "LogisiticRegression" => Ok(Self::LogisiticRegression),
            "BinaryClassification" => Ok(Self::BinaryClassification),
            _ => Err("invalid model type".into()),
        }
    }
}
