use std::{fmt::Display, str::FromStr, time::Duration};

use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Part {
    First,
    Second,
}
impl Part {
    pub const fn idx(&self) -> usize {
        match self {
            Part::First => 0,
            Part::Second => 1,
        }
    }
}
impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "1" => Self::First,
            "2" => Self::Second,
            _ => bail!("Unrecognized part"),
        })
    }
}
impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::First => write!(f, "1"),
            Part::Second => write!(f, "2"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baseline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev_time: Option<Duration>,
    pub prev_answer: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub year: u16,
    pub day: u8,
    pub part: Part,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
    #[serde(flatten)]
    pub baseline: Option<Baseline>,
}
#[derive(Debug, Clone, Serialize)]
pub struct RunResult {
    #[serde(flatten)]
    pub run: Run,

    pub answer: String,
    pub time: Duration,
}
