use std::num::NonZeroUsize;

use chrono::Duration;
use serde::{Deserialize, Serialize};

use aoc_runtime::calendar::{AoCDay, AoCPart, AoCYear};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum Request {
    List {
        year: AoCYear,
        day: AoCDay,
        part: AoCPart,
    },
    Run {
        idx: usize,
        input: String,
    },
    TimeIt {
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
    aoc_runtime::SOLUTIONS
        .iter()
        .enumerate()
        .filter(
            move |(
                _,
                aoc_runtime::Solution {
                    year: y,
                    day: d,
                    part: p,
                    ..
                },
            )| *y == year && *d == day && *p == part,
        )
        .map(
            |(
                idx,
                aoc_runtime::Solution {
                    year,
                    day,
                    part,
                    long_running,
                    fun,
                },
            )| SolutionInfo {
                year: *year,
                day: *day,
                part: *part,
                idx,
                multiline: fun.is_multiline(),
                long_running: *long_running,
            },
        )
}

pub fn run(idx: usize, input: &str) -> String {
    todo!()
}

pub fn timeit(idx: usize, input: &str, reps: NonZeroUsize) -> Duration {
    todo!()
}

pub fn exec(req: Request) -> Response {
    match req {
        Request::List { year, day, part } => Response::List {
            found: list(year, day, part).collect(),
        },
        Request::Run { idx, input } => Response::Run {
            answer: run(idx, &input),
        },
        Request::TimeIt { idx, reps, input } => Response::TimeIt {
            time: timeit(idx, &input, reps),
        },
    }
}
