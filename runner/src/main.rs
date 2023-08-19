#![feature(btree_drain_filter)]
#![feature(once_cell)]
#![feature(string_leak)]

use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::{self, File},
    io::{self, stdout},
    panic::catch_unwind,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Mutex,
    time::{Duration, Instant},
};

use anyhow::{bail, Context};
use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

use aoc_library::{library, Day, Solution};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Debug, Parser)]
struct Cli {
    /// Limit on what problems to run
    #[clap(short, long)]
    filter: Option<Vec<SingleFilter>>,
    /// Directory for the inputs database
    #[clap(short, long, default_value = "./inputs")]
    inputs: PathBuf,
    /// File for the correct answers, for checking
    #[clap(short, long)]
    answers: Option<PathBuf>,
    /// Baseline for caching/checking times and answers [default: ./baseline.json]
    #[clap(short, long)]
    baseline: Option<Option<PathBuf>>,
    /// Save new baseline for caching/checking times and answers [default if no value is given: --baseline value or ./baseline.json]
    #[clap(short, long)]
    save_baseline: Option<Option<PathBuf>>,
    /// Verbosity of the output
    #[clap(short, action = clap::ArgAction::Count)]
    verbosity: u8,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
enum Range<Int> {
    #[default]
    All,
    From(Int),
    To(Int),
    Between(Int, Int),
}
impl<Int: Ord> Range<Int> {
    fn matches(&self, item: &Int) -> bool {
        match self {
            Range::All => true,
            Range::From(a) => a <= item,
            Range::To(b) => item <= b,
            Range::Between(a, b) => a <= item && item <= b,
        }
    }
}
impl<Int> FromStr for Range<Int>
where
    Int: FromStr + Clone + Ord,
    Result<Int, Int::Err>: Context<Int, Int::Err>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((a, b)) = s.split_once("..") else {
            let n: Int = s.trim().parse().context("Cannot parse value")?;
            return Ok(Self::Between(n.clone(), n));
        };
        Ok(match (a.trim(), b.trim()) {
            ("", "") => Self::All,
            (a, "") => Self::From(a.parse().context("Cannot parse starting point")?),
            ("", b) => Self::To(b.parse().context("Cannot parse end point")?),
            (a, b) => {
                if b < a {
                    bail!("Invalid range")
                }
                Self::Between(
                    a.parse().context("Cannot parse starting point")?,
                    b.parse().context("Cannot parse end point")?,
                )
            }
        })
    }
}
impl<Int> Display for Range<Int>
where
    Int: Display + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Range::All => write!(f, ".."),
            Range::From(a) => write!(f, "{a}.."),
            Range::To(b) => write!(f, "..{b}"),
            Range::Between(a, b) if a == b => write!(f, "{a}"),
            Range::Between(a, b) => write!(f, "{a}..{b}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Part {
    First,
    Second,
}
impl Part {
    fn idx(&self) -> usize {
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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
struct SingleFilter {
    years: Range<u16>,
    days: Range<u8>,
    parts: Range<Part>,
}
impl SingleFilter {
    fn matches(&self, year: &u16, day: &u8, part: &Part) -> bool {
        self.years.matches(year) && self.days.matches(day) && self.parts.matches(part)
    }

    fn simplify(self) -> Self {
        let Self { years, days, parts } = self;
        let years = match years {
            Range::From(a) if a <= 2015 => Range::All,
            Range::Between(a, b) if a <= 2015 => Range::To(b),
            years => years,
        };
        let days = match days {
            Range::From(0 | 1) => Range::All,
            Range::To(b) if b >= 25 => Range::All,
            Range::Between(0 | 1, b) if b >= 25 => Range::All,
            Range::Between(0 | 1, b) => Range::To(b),
            Range::Between(a, b) if b >= 25 => Range::From(a),
            day => day,
        };
        let parts = match parts {
            Range::All
            | Range::From(Part::First)
            | Range::To(Part::Second)
            | Range::Between(Part::First, Part::Second) => Range::All,
            Range::From(Part::Second) | Range::Between(Part::Second, Part::Second) => {
                Range::Between(Part::Second, Part::Second)
            }
            Range::To(Part::First) | Range::Between(Part::First, Part::First) => {
                Range::Between(Part::First, Part::First)
            }
            Range::Between(Part::Second, Part::First) => unreachable!(),
        };
        Self { years, days, parts }
    }
}
impl FromStr for SingleFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: arrayvec::ArrayVec<_, 3> = s.splitn(3, ':').map(str::trim).collect();
        Ok(Self {
            years: parts
                .get(0)
                .unwrap()
                .parse()
                .context("Cannot parse years")?,
            days: parts
                .get(1)
                .copied()
                .map(str::parse)
                .transpose()
                .context("Cannot parse days")?
                .unwrap_or_default(),
            parts: parts
                .get(2)
                .copied()
                .map(str::parse)
                .transpose()
                .context("Cannot parse days")?
                .unwrap_or_default(),
        }
        .simplify())
    }
}
impl Display for SingleFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self {
                years,
                days: Range::All,
                parts: Range::All,
            } => write!(f, "{years}"),
            Self {
                years,
                days,
                parts: Range::All,
            } => write!(f, "{years}:{days}"),
            Self { years, days, parts } => write!(f, "{years}:{days}:{parts}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Filter(Vec<SingleFilter>);

impl Filter {
    fn matches(&self, year: &u16, day: &u8, part: &Part) -> bool {
        self.0.iter().any(|f| f.matches(year, day, part))
    }

    fn is_all(&self) -> bool {
        matches!(
            &*self.0,
            [SingleFilter {
                years: Range::All,
                days: Range::All,
                parts: Range::All
            }]
        )
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self(vec![SingleFilter::default()])
    }
}
impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().format(" && ").fmt(f)
    }
}
impl From<Vec<SingleFilter>> for Filter {
    fn from(value: Vec<SingleFilter>) -> Self {
        Self(value)
    }
}

lazy_static! {
    static ref DEFAULT_BASELINE_FILE: PathBuf = PathBuf::from("./baseline.json");
}

fn setup_logger(verbosity: u8) -> anyhow::Result<()> {
    let mut logger = SimpleLogger::new()
        .without_timestamps()
        .with_level(log::LevelFilter::Warn)
        .env();
    if verbosity != 0 {
        logger = logger.with_level(match verbosity {
            0 => unreachable!(),
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
    }
    logger
        .init()
        .context("Error while initializing output logging")?;
    log::trace!("Begin logging with verbosity {verbosity}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Cli {
        filter,
        inputs,
        answers,
        baseline,
        save_baseline,
        verbosity,
    } = Cli::parse();
    setup_logger(verbosity)?;

    let filter = filter.map(Filter::from).unwrap_or_default();

    let library = filter_library(&filter, library());

    let mut answers = answers
        .map(read_answers)
        .transpose()
        .context("Cannot read answer database")?
        .unwrap_or_default();
    let baseline = baseline.map(|b| b.unwrap_or(DEFAULT_BASELINE_FILE.clone()));
    let save_baseline = save_baseline.map(|sb| {
        sb.or(baseline.clone())
            .unwrap_or(DEFAULT_BASELINE_FILE.clone())
    });
    let mut baseline = if let Some(baseline) = baseline {
        read_baselines(baseline).context("Cannot read baseline")?
    } else if let Ok(baseline) = read_baselines(&*DEFAULT_BASELINE_FILE) {
        baseline
    } else {
        Default::default()
    };

    let mut results = vec![];
    for (year, days) in library {
        log::info!("Running year {year}");
        for (day, Day { part1, part2 }) in days {
            log::info!("Running day {day}");
            for (part, solution) in [(Part::First, part1), (Part::Second, part2)]
                .into_iter()
                .filter_map(|(p, s)| s.map(|s| (p, s)))
            {
                log::info!("Running part {part}");
                let run_data = Run {
                    year,
                    day,
                    part,
                    solution,
                    answer: answers
                        .get_mut(&year)
                        .and_then(|y| y.get_mut(&day))
                        .and_then(|d| d[part.idx()].take()),
                    baseline: baseline
                        .get_mut(&year)
                        .and_then(|y| y.get_mut(&day))
                        .and_then(|d| d[part.idx()].take()),
                };
                let res = run(run_data, &inputs);
                match res {
                    Ok(res) => results.push(res),
                    Err(err) => log::error!("Error during year {year}, day {day}: {err:?}"),
                };
            }
        }
    }

    if save_baseline.is_some() {
        for RunResult {
            run: Run {
                year, day, part, ..
            },
            answer,
            time,
        } in results.iter()
        {
            baseline.entry(*year).or_default().entry(*day).or_default()[part.idx()] =
                Some(Baseline {
                    time: Some(*time),
                    answer: answer.clone(),
                })
        }
    }

    let table = FullTable::new(filter, results, Default::default());

    serde_yaml::to_writer(stdout(), &table)?;

    if let Some(savefile) = save_baseline {
        serde_json::to_writer(
            File::create(savefile).context("Cannot save the baseline")?,
            &baseline,
        )
        .context("Cannot save the baseline")?
    }

    Ok(())
}

type SolutionsLibrary = BTreeMap<u16, BTreeMap<u8, Day>>;

fn filter_library(filter: &Filter, library: SolutionsLibrary) -> SolutionsLibrary {
    log::info!("Filtering library");
    log::info!(
        "Total solution parts before: {}",
        library
            .iter()
            .flat_map(|(_, y)| y.iter())
            .map(|(_, d)| match d {
                Day {
                    part1: None,
                    part2: None,
                } => 0,
                Day {
                    part1: Some(_),
                    part2: None,
                }
                | Day {
                    part1: None,
                    part2: Some(_),
                } => 1,
                Day {
                    part1: Some(_),
                    part2: Some(_),
                } => 2,
            })
            .sum::<usize>()
    );
    let mut res: SolutionsLibrary = BTreeMap::new();
    let mut filtered = 0usize;
    for (year, days) in library {
        for (day, Day { part1, part2 }) in days {
            if let Some(part1) = part1 {
                if filter.matches(&year, &day, &Part::First) {
                    res.entry(year).or_default().entry(day).or_default().part1 = Some(part1);
                    filtered += 1;
                }
            }
            if let Some(part2) = part2 {
                if filter.matches(&year, &day, &Part::Second) {
                    res.entry(year).or_default().entry(day).or_default().part2 = Some(part2);
                    filtered += 1;
                }
            }
        }
    }
    log::info!("Filtered solution parts: {filtered}");
    res
}

fn read_input(inputs: &Path, year: u16, day: u8) -> anyhow::Result<&'static str> {
    static INPUT: Mutex<BTreeMap<u16, BTreeMap<u8, Result<&'static str, &'static io::Error>>>> =
        Mutex::new(BTreeMap::new());

    match INPUT.lock() {
        Ok(mut l) => Ok(l
            .entry(year)
            .or_default()
            .entry(day)
            .or_insert_with(|| {
                fs::read_to_string(inputs.join(year.to_string()).join(day.to_string()))
                    .map(|s| &*s.leak())
                    .map_err(|err| &*Box::leak(Box::new(err)))
            })
            .clone()?),
        Err(err) => {
            bail!("Cannot lock input cache: {err}");
        }
    }
}

type AnswersLibrary = BTreeMap<u16, BTreeMap<u8, [Option<String>; 2]>>;

fn read_answers(dir: impl AsRef<Path>) -> anyhow::Result<AnswersLibrary> {
    log::info!("Reading answers");
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum SerDeAnswer {
        Numerical(i64),
        Alpha(String),
    }

    impl SerDeAnswer {
        #[must_use]
        fn into_string(self) -> String {
            match self {
                SerDeAnswer::Numerical(n) => n.to_string(),
                SerDeAnswer::Alpha(s) => s,
            }
        }
    }
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum SerDeDay {
        Struct {
            #[serde(default, alias = "1")]
            part1: Option<SerDeAnswer>,
            #[serde(default, alias = "2")]
            part2: Option<SerDeAnswer>,
        },
        Tuple([Option<SerDeAnswer>; 2]),
    }
    impl From<SerDeDay> for [Option<String>; 2] {
        fn from(value: SerDeDay) -> Self {
            match value {
                SerDeDay::Struct { part1, part2 } => [
                    part1.map(SerDeAnswer::into_string),
                    part2.map(SerDeAnswer::into_string),
                ],
                SerDeDay::Tuple(vals) => vals.map(|v| v.map(SerDeAnswer::into_string)),
            }
        }
    }

    let answers: BTreeMap<u16, BTreeMap<u8, SerDeDay>> =
        serde_json::from_reader(File::open(dir).context("Cannot open file")?)
            .context("Cannot parse file")?;

    Ok(answers
        .into_iter()
        .map(|(y, yrs)| (y, yrs.into_iter().map(|(d, day)| (d, day.into())).collect()))
        .collect())
}

type BaselinesLibrary = BTreeMap<u16, BTreeMap<u8, [Option<Baseline>; 2]>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Baseline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time: Option<Duration>,
    answer: String,
}

fn read_baselines(dir: impl AsRef<Path>) -> anyhow::Result<BaselinesLibrary> {
    log::info!("Reading baselines");
    serde_json::from_reader(File::open(dir).context("Cannot open file")?)
        .context("Cannot parse file")
}

#[derive(Debug, Clone)]
struct Run {
    year: u16,
    day: u8,
    part: Part,

    solution: Solution,

    answer: Option<String>,
    baseline: Option<Baseline>,
}

#[derive(Debug, Clone)]
struct RunResult {
    run: Run,

    answer: String,
    time: Duration,
}

fn run(run: Run, inputs: &Path) -> anyhow::Result<RunResult> {
    let input = read_input(inputs, run.year, run.day).context("Cannot read input")?;
    match catch_unwind(|| match run.solution {
        Solution::Numeric(fun) => {
            let start = Instant::now();
            let ans = fun(&input);
            let time = Instant::now() - start;
            (ans.to_string(), time)
        }
        Solution::Alpha(fun) => {
            let start = Instant::now();
            let ans = fun(&input);
            let time = Instant::now() - start;
            (ans, time)
        }
    }) {
        Ok((answer, time)) => Ok(RunResult { run, answer, time }),
        Err(cause) => match cause
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| cause.downcast_ref::<&str>().map(|s| *s))
        {
            Some(s) => bail!("Panicked at {s:?}"),
            None => bail!("Panicked with an unknow object"),
        },
    }
}

#[derive(Debug, Clone, Copy)]
struct Showing {
    answers: bool,
    times: bool,
    prev_times: bool,
    correct: bool,
    was_correct: bool,
}
impl Default for Showing {
    fn default() -> Self {
        Self {
            answers: true,
            times: true,
            prev_times: true,
            correct: true,
            was_correct: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct FullTable {
    #[serde(default, skip_serializing_if = "Filter::is_all")]
    filter: Filter,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    years: BTreeMap<u16, YearTable>,
    #[serde(default, skip_serializing_if = "StatCell::is_empty")]
    total: StatCell,
}
impl FullTable {
    fn new(filter: Filter, data: impl IntoIterator<Item = RunResult>, showing: Showing) -> Self {
        let mut years: BTreeMap<u16, YearTable> = BTreeMap::new();
        for res @ RunResult {
            run: Run { year, .. },
            ..
        } in data
        {
            years.entry(year).or_default().insert(res, &showing)
        }
        let total = if !years.is_empty() {
            years.values_mut().fold(
                StatCell {
                    correct: Some(true),
                    was_correct: Some(true),
                    time: Some(Duration::ZERO),
                    prev_time: Some(Duration::ZERO),
                },
                |t, y| t.complexive(y.calc_total()),
            )
        } else {
            Default::default()
        };

        Self {
            filter,
            years,
            total,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct YearTable {
    days: BTreeMap<u8, DayRow>,
    #[serde(default, skip_serializing_if = "StatCell::is_empty")]
    total: StatCell,
}
impl YearTable {
    fn insert(
        &mut self,
        res @ RunResult {
            run: Run { day, .. },
            ..
        }: RunResult,
        showing: &Showing,
    ) {
        self.days.entry(day).or_default().insert(res, showing)
    }

    fn calc_total(&mut self) -> &StatCell {
        self.total = self.days.values_mut().fold(
            StatCell {
                correct: Some(true),
                was_correct: Some(true),
                time: Some(Duration::ZERO),
                prev_time: Some(Duration::ZERO),
            },
            |t, d| t.complexive(d.calc_total()),
        );
        &self.total
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct DayRow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    part1: Option<PartCell>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    part2: Option<PartCell>,
    #[serde(default, skip_serializing_if = "StatCell::is_empty")]
    total: StatCell,
}
impl DayRow {
    fn insert(
        &mut self,
        res @ RunResult {
            run: Run { part, .. },
            ..
        }: RunResult,
        showing: &Showing,
    ) {
        match part {
            Part::First => self.part1 = Some(PartCell::new(res, showing)),
            Part::Second => self.part2 = Some(PartCell::new(res, showing)),
        }
    }

    fn calc_total(&mut self) -> &StatCell {
        self.total = StatCell {
            correct: Some(true),
            was_correct: Some(true),
            time: Some(Duration::ZERO),
            prev_time: Some(Duration::ZERO),
        };
        if let Some(part1) = &self.part1 {
            self.total = self.total.complexive(&part1.stats)
        }
        if let Some(part2) = &self.part2 {
            self.total = self.total.complexive(&part2.stats)
        }
        &self.total
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PartCell {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    answer: Option<String>,
    #[serde(flatten)]
    stats: StatCell,
}

impl PartCell {
    fn new(RunResult { run, answer, time }: RunResult, showing: &Showing) -> PartCell {
        let correct_answer = run.answer.as_ref();
        let prev_time = run.baseline.as_ref().and_then(|b| b.time.as_ref());
        let prev_answer = run.baseline.as_ref().map(|b| &b.answer);

        PartCell {
            stats: StatCell {
                correct: if let (true, Some(correct_answer)) = (showing.correct, correct_answer) {
                    Some(&answer == correct_answer)
                } else {
                    None
                },
                was_correct: if let (true, Some(correct_answer), Some(prev_answer)) =
                    (showing.was_correct, correct_answer, prev_answer)
                {
                    Some(prev_answer == correct_answer)
                } else {
                    None
                },
                time: if showing.times { Some(time) } else { None },
                prev_time: if let (true, Some(prev_time)) = (showing.prev_times, prev_time) {
                    Some(*prev_time)
                } else {
                    None
                },
            },
            answer: if showing.answers { Some(answer) } else { None },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
struct StatCell {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    correct: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    was_correct: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time: Option<Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    prev_time: Option<Duration>,
}

impl StatCell {
    fn complexive(&self, other: &Self) -> Self {
        Self {
            correct: match (self.correct, other.correct) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a && b),
            },
            was_correct: match (self.was_correct, other.was_correct) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a && b),
            },
            time: match (self.time, other.time) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a + b),
            },
            prev_time: match (self.prev_time, other.prev_time) {
                (None, None) | (None, Some(_)) | (Some(_), None) => None,
                (Some(a), Some(b)) => Some(a + b),
            },
        }
    }

    fn is_empty(&self) -> bool {
        self.correct.is_none()
            && self.was_correct.is_none()
            && self.time.is_none()
            && self.prev_time.is_none()
    }
}
