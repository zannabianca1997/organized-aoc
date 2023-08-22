use std::{borrow::Cow, num::NonZeroUsize};

use chrono::Duration;
use serde::{Deserialize, Serialize};

use aoc::{AoCDay, AoCPart, AoCYear, Solution};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Request {
    List {
        year: AoCYear,
        day: AoCDay,
        part: AoCPart,
    },
    Run {
        year: AoCYear,
        day: AoCDay,
        part: AoCPart,
        idx: usize,
        input: String,
    },
    TimeIt {
        year: AoCYear,
        day: AoCDay,
        part: AoCPart,
        idx: usize,
        input: String,
        reps: NonZeroUsize,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Response {
    List {
        found: Vec<SolutionInfo>,
    },
    Run {
        answer: String,
    },
    TimeIt {
        #[serde(with = "serde_duration")]
        time: Duration,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolutionInfo {
    pub year: AoCYear,
    pub day: AoCDay,
    pub part: AoCPart,
    pub idx: usize,
    pub multiline: bool,
    pub long_running: bool,
    pub name: Cow<'static, str>,
    pub descr: Option<Cow<'static, str>>,
}

mod serde_duration {
    use chrono::Duration;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Deserialize, Serialize)]
    #[serde(tag = "unit")]
    enum SerializedDuration {
        Nanos { num: i64 },
        Micros { num: i64 },
        Millis { num: i64 },
    }
    impl From<&Duration> for SerializedDuration {
        fn from(value: &Duration) -> Self {
            if let Some(num) = value.num_nanoseconds() {
                SerializedDuration::Nanos { num }
            } else if let Some(num) = value.num_microseconds() {
                SerializedDuration::Micros { num }
            } else {
                SerializedDuration::Millis {
                    num: value.num_milliseconds(),
                }
            }
        }
    }
    impl From<SerializedDuration> for Duration {
        fn from(value: SerializedDuration) -> Self {
            match value {
                SerializedDuration::Nanos { num } => Duration::nanoseconds(num),
                SerializedDuration::Micros { num } => Duration::microseconds(num),
                SerializedDuration::Millis { num } => Duration::milliseconds(num),
            }
        }
    }

    pub(super) fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializedDuration::from(duration).serialize(serializer)
    }
    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        SerializedDuration::deserialize(deserializer).map(Into::into)
    }
}

pub fn list(year: AoCYear, day: AoCDay, part: AoCPart) -> impl Iterator<Item = SolutionInfo> {
    index::LIBRARY
        .get_part(year, day, part)
        .into_iter()
        .copied()
        .copied()
        .enumerate()
        .map(
            move |(
                idx,
                Solution {
                    name,
                    long_running,
                    descr,
                    fun,
                },
            )| {
                SolutionInfo {
                    year,
                    day,
                    part,
                    idx,
                    multiline: fun.is_multiline(),
                    long_running,
                    name: Cow::Borrowed(name),
                    descr: descr.map(Cow::Borrowed),
                }
            },
        )
}

pub fn run(year: AoCYear, day: AoCDay, part: AoCPart, idx: usize, input: &str) -> String {
    todo!()
}

pub fn timeit(
    year: AoCYear,
    day: AoCDay,
    part: AoCPart,
    idx: usize,
    input: &str,
    reps: NonZeroUsize,
) -> Duration {
    todo!()
}

pub fn exec(req: Request) -> Response {
    match req {
        Request::List { year, day, part } => Response::List {
            found: list(year, day, part).collect(),
        },
        Request::Run {
            idx,
            input,
            year,
            day,
            part,
        } => Response::Run {
            answer: run(year, day, part, idx, &input),
        },
        Request::TimeIt {
            year,
            day,
            part,
            idx,
            reps,
            input,
        } => Response::TimeIt {
            time: timeit(year, day, part, idx, &input, reps),
        },
    }
}
