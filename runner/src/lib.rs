#![feature(iterator_try_collect)]
use std::{
    borrow::Cow,
    collections::BTreeMap,
    ffi::OsString,
    fmt::{self, Debug, Display, Write},
    fs::{read_to_string, File},
    hint::black_box,
    io,
    num::NonZeroU32,
    panic::catch_unwind,
    path::{Path, PathBuf},
    process::exit,
    rc::Rc,
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::Parser;
use html_builder::{Buffer, Html5, Node};
use report::Report;
use serde::{Deserialize, Serialize, Serializer};

use crate::filters::Filters;

#[derive(Clone, Debug, Copy)]
pub enum PartFn {
    ISize(fn(&str) -> isize),
    I64(fn(&str) -> i64),
    I32(fn(&str) -> i32),
    I16(fn(&str) -> i16),
    I8(fn(&str) -> i8),
    USize(fn(&str) -> usize),
    U64(fn(&str) -> u64),
    U32(fn(&str) -> u32),
    U16(fn(&str) -> u16),
    U8(fn(&str) -> u8),
    String(fn(&str) -> String),
    Str(fn(&str) -> &str),
    Bites8(fn(&str) -> [u8; 8]),
}
impl PartFn {
    fn call(&self, input: &str) -> String {
        match self {
            PartFn::ISize(f) => (f)(input).to_string(),
            PartFn::I64(f) => (f)(input).to_string(),
            PartFn::I32(f) => (f)(input).to_string(),
            PartFn::I16(f) => (f)(input).to_string(),
            PartFn::I8(f) => (f)(input).to_string(),
            PartFn::USize(f) => (f)(input).to_string(),
            PartFn::U64(f) => (f)(input).to_string(),
            PartFn::U32(f) => (f)(input).to_string(),
            PartFn::U16(f) => (f)(input).to_string(),
            PartFn::U8(f) => (f)(input).to_string(),
            PartFn::String(f) => (f)(input),
            PartFn::Str(f) => (f)(input).to_string(),
            PartFn::Bites8(f) => String::from_utf8(Vec::from(f(input)))
                .expect("The solution must return valid utf-8"),
        }
    }
    fn time(&self, input: &str, reps: u32) -> Duration {
        (match self {
            PartFn::ISize(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::I64(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::I32(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::I16(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::I8(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::USize(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::U64(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::U32(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::U16(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::U8(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::String(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::Str(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
            PartFn::Bites8(f) => {
                let s = Instant::now();
                for _ in 0..reps {
                    black_box((f)(black_box(input)));
                }
                s.elapsed()
            }
        }) / reps
    }
}

impl From<fn(&str) -> isize> for PartFn {
    fn from(value: fn(&str) -> isize) -> Self {
        PartFn::ISize(value)
    }
}
impl From<fn(&str) -> i64> for PartFn {
    fn from(value: fn(&str) -> i64) -> Self {
        PartFn::I64(value)
    }
}
impl From<fn(&str) -> i32> for PartFn {
    fn from(value: fn(&str) -> i32) -> Self {
        PartFn::I32(value)
    }
}
impl From<fn(&str) -> i16> for PartFn {
    fn from(value: fn(&str) -> i16) -> Self {
        PartFn::I16(value)
    }
}
impl From<fn(&str) -> i8> for PartFn {
    fn from(value: fn(&str) -> i8) -> Self {
        PartFn::I8(value)
    }
}
impl From<fn(&str) -> usize> for PartFn {
    fn from(value: fn(&str) -> usize) -> Self {
        PartFn::USize(value)
    }
}
impl From<fn(&str) -> u64> for PartFn {
    fn from(value: fn(&str) -> u64) -> Self {
        PartFn::U64(value)
    }
}
impl From<fn(&str) -> u32> for PartFn {
    fn from(value: fn(&str) -> u32) -> Self {
        PartFn::U32(value)
    }
}
impl From<fn(&str) -> u16> for PartFn {
    fn from(value: fn(&str) -> u16) -> Self {
        PartFn::U16(value)
    }
}
impl From<fn(&str) -> u8> for PartFn {
    fn from(value: fn(&str) -> u8) -> Self {
        PartFn::U8(value)
    }
}
impl From<fn(&str) -> String> for PartFn {
    fn from(value: fn(&str) -> String) -> Self {
        PartFn::String(value)
    }
}
impl From<fn(&str) -> &str> for PartFn {
    fn from(value: fn(&str) -> &str) -> Self {
        PartFn::Str(value)
    }
}
impl From<fn(&str) -> [u8; 8]> for PartFn {
    fn from(value: fn(&str) -> [u8; 8]) -> Self {
        PartFn::Bites8(value)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Part {
    fun: PartFn,
    multiline: bool,
    long_running: bool,
}
impl Part {
    fn measure(
        &self,
        repeats: Option<NonZeroU32>,
        input: &str,
    ) -> Result<Measurements, FailedMeasurements> {
        catch_unwind(|| {
            // first run it once to find the answer
            if !self.long_running {
                let answer = self.fun.call(input);
                let time = if let Some(reps) = repeats {
                    Some(self.fun.time(input, reps.get()))
                } else {
                    None
                };
                Measurements {
                    answer,
                    time,
                    multiline: self.multiline,
                }
            } else {
                log::info!("Checking long running solution");
                let s = Instant::now();
                let answer = self.fun.call(input);
                let time = if repeats.is_some() {
                    Some(s.elapsed())
                } else {
                    None
                };
                Measurements {
                    answer,
                    time,
                    multiline: self.multiline,
                }
            }
        })
        .map_err(|err| match err.downcast::<String>() {
            Ok(s) => FailedMeasurements {
                err: Some(Cow::Owned(*s)),
            },
            Err(err) => match err.downcast::<&'static str>() {
                Ok(s) => FailedMeasurements {
                    err: Some(Cow::Borrowed(*s)),
                },
                Err(_) => FailedMeasurements { err: None },
            },
        })
    }
}

#[derive(Debug)]

pub struct Day {
    day: (u16, u8),
    parts: [Option<Part>; 2],
    filters: Rc<Filters>,
}
impl Day {
    pub fn add_part_1(
        &mut self,
        fun: impl Into<PartFn>,
        multiline: bool,
        long_running: bool,
    ) -> &mut Self {
        if self.filters.accept_part(self.day.0, self.day.1, 1) {
            self.parts[0] = Some(Part {
                fun: fun.into(),
                multiline,
                long_running,
            });
        }
        self
    }
    pub fn add_part_2(
        &mut self,
        fun: impl Into<PartFn>,
        multiline: bool,
        long_running: bool,
    ) -> &mut Self {
        if self.filters.accept_part(self.day.0, self.day.1, 2) {
            self.parts[1] = Some(Part {
                fun: fun.into(),
                multiline,
                long_running,
            });
        }
        self
    }

    fn measure(
        &self,
        repeats: Option<NonZeroU32>,
        inputs: &Path,
    ) -> Result<[Option<Result<Measurements, FailedMeasurements>>; 2], io::Error> {
        if self.parts.iter().all(|p| p.is_none()) {
            return Ok([None, None]);
        }
        log::info!("Measuring day {}", self.day.1);
        let input = read_to_string(
            inputs
                .join(self.day.0.to_string())
                .join(self.day.1.to_string()),
        )?;
        Ok(self.parts.map(|p| p.map(|p| p.measure(repeats, &input))))
    }

    fn is_empty(&self) -> bool {
        self.parts.iter().all(|p| p.is_none())
    }
}

#[derive(Debug)]
pub struct Year {
    year: u16,
    solutions: BTreeMap<u8, Day>,
    filters: Rc<Filters>,
}
impl Year {
    pub fn add_day<F>(&mut self, day: u8, build: F) -> &mut Self
    where
        F: FnOnce(&mut Day),
    {
        if day < 1 || day > 25 {
            panic!("Day {day} is invalid, not an advent day")
        }
        if self.filters.accept_day(self.year, day) {
            match self.solutions.entry(day) {
                std::collections::btree_map::Entry::Vacant(v) => {
                    let mut entries = Day {
                        day: (self.year, day),
                        parts: [None, None],
                        filters: self.filters.clone(),
                    };
                    build(&mut entries);
                    // do not create empty days
                    if !entries.is_empty() {
                        v.insert(entries);
                    }
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => build(entry.get_mut()),
            }
        }
        self
    }

    fn measure(
        &self,
        repeats: Option<NonZeroU32>,
        inputs: &Path,
    ) -> BTreeMap<u8, io::Result<[Option<Result<Measurements, FailedMeasurements>>; 2]>> {
        log::info!("Measuring year {}", self.year);
        self.solutions
            .iter()
            .map(|(day, sols)| (*day, sols.measure(repeats, inputs)))
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.solutions.is_empty()
    }
}

#[derive(Debug)]
pub struct Library {
    solutions: BTreeMap<u16, Year>,
    filters: Rc<Filters>,
}

impl Library {
    pub fn add_year<F>(&mut self, year: u16, build: F) -> &mut Self
    where
        F: FnOnce(&mut Year),
    {
        if year < 2015 {
            panic!("Year {year} is invalid, AoC was not present before 2015")
        }
        if self.filters.accept_year(year) {
            match self.solutions.entry(year) {
                std::collections::btree_map::Entry::Vacant(v) => {
                    let mut entries = Year {
                        year,
                        solutions: BTreeMap::new(),
                        filters: self.filters.clone(),
                    };
                    build(&mut entries);
                    // do not create empty years
                    if !entries.is_empty() {
                        v.insert(entries);
                    }
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => build(entry.get_mut()),
            }
        }
        self
    }

    fn measure(
        &self,
        repeats: Option<NonZeroU32>,
        inputs: &Path,
    ) -> BTreeMap<
        u16,
        BTreeMap<u8, io::Result<[Option<Result<Measurements, FailedMeasurements>>; 2]>>,
    > {
        self.solutions
            .iter()
            .map(|(year, sols)| (*year, sols.measure(repeats, inputs)))
            .collect()
    }
}

#[derive(Parser)]
struct Args {
    #[clap(default_value_t)]
    /// What problems to run
    problems: filters::Filters,
    /// Number of repeats used to measure running time (0 to not measure times)
    #[clap(long, short, default_value = "1")]
    repeats: u32,
    /// Directory for the inputs
    #[clap(long, short, default_value = "./inputs")]
    inputs: PathBuf,
    /// File with the correct answers [default: INPUTS/answers.json]
    #[clap(long, short)]
    answers: Option<Option<PathBuf>>,
    /// File with the baseline for the benches
    #[clap(long, short)]
    baseline: Option<Option<PathBuf>>,
    /// File where to save the new baseline
    #[clap(long)]
    save_baseline: Option<Option<PathBuf>>,
}

mod filters;

fn main<F>(
    build: F,
    Args {
        problems,
        repeats,
        inputs,
        answers,
        baseline,
        save_baseline,
    }: Args,
) -> anyhow::Result<()>
where
    F: FnOnce(&mut Library),
{
    let repeats = NonZeroU32::new(repeats);
    log::info!("Parsing databases");
    let answers = match answers {
        Some(Some(answers)) => Some(read_answers(&answers).context("Cannot read answer file")?),
        Some(None) => Some(
            read_answers(inputs.join("answers.json")).context("Cannot read default answer file")?,
        ),
        // try to read the default file, but silently ignore errors
        None => read_answers(inputs.join("answers.json")).ok(),
    }
    // Empty answers
    .unwrap_or_default();
    let save_baseline = save_baseline.map(|sb| {
        sb.or(baseline.clone().flatten())
            .unwrap_or_else(|| PathBuf::from("./baseline.json"))
    });
    let baseline = match baseline {
        Some(Some(baseline)) => {
            Some(read_baseline(&baseline).context("Cannot read baseline file")?)
        }
        Some(None) => {
            Some(read_baseline(&"./baseline.json").context("Cannot read default baseline file")?)
        }
        // try to read the default file, but silently ignore errors
        None => read_baseline(&"./baseline.json").ok(),
    }
    // Empty baseline if nothing is given
    .unwrap_or_default();
    log::info!("Building library");
    let library = {
        let mut lib = Library {
            solutions: BTreeMap::new(),
            filters: Rc::new(problems),
        };
        (build)(&mut lib);
        lib
    };
    log::info!("Executing tests");
    let measures = library.measure(repeats, &inputs);
    // saving baselines
    if let Some(save_baseline) = save_baseline {
        log::info!("Saving baselines");
        if let Err(err) = dump_baseline(&save_baseline, &measures) {
            log::warn!("Failed to save baselines: {err:?}")
        }
    }

    log::info!("Building report");
    let report = Report::new(library.filters, measures, answers, baseline);

    print!(
        "{}",
        html_page(report).expect("Writing in a html buffer should be error_free")
    );
    Ok(())
}

fn html_page(report: Report) -> Result<impl Display, fmt::Error> {
    log::info!("Rendering report");
    let mut buf = Buffer::new();
    buf.doctype();
    let mut head = buf.head();
    writeln!(
        head.title(),
        "Report for the query \"{}\"",
        report.problems()
    )?;
    head.meta().attr("charset=\"UTF-8\"");
    head.meta()
        .attr("name=\"description\"")
        .attr("content=\"Self generated Advent Of Code report\"");
    head.meta()
        .attr("name=\"author\"")
        .attr("content=\"zannabianca1997\"");
    head.meta().attr("name=\"generator\"").attr(&format!(
        "content=\"{} {}\"",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
    writeln!(head.style(), "{}", include_str!("report_style.css"))?;
    let mut body = buf.body();
    report.render(&mut body)?;
    Ok(buf.finish())
}

mod report;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Baseline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    time: Option<Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    answer: Option<String>,
}
type Baselines = BTreeMap<u16, BTreeMap<u8, [Option<Baseline>; 2]>>;

fn read_baseline(file: impl AsRef<Path>) -> anyhow::Result<Baselines> {
    let file = File::open(file).context("Cannot open baseline file")?;
    serde_json::from_reader(file).context("Cannot parse baseline file")
}

fn dump_baseline(
    save_baseline: &PathBuf,
    measures: &BTreeMap<
        u16,
        BTreeMap<u8, Result<[Option<Result<Measurements, FailedMeasurements>>; 2], io::Error>>,
    >,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct BorrowedBaseline<'a> {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        answer: Option<&'a str>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time: Option<&'a Duration>,
    }

    struct Day<'a>(&'a [Option<Result<Measurements, FailedMeasurements>>; 2]);
    impl Serialize for Day<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(self.0.iter().map(|s| {
                s.as_ref().map(|r| r.as_ref().ok()).flatten().map(
                    |Measurements { answer, time, .. }| BorrowedBaseline {
                        answer: Some(answer),
                        time: time.as_ref(),
                    },
                )
            }))
        }
    }

    struct Year<'a>(
        &'a BTreeMap<u8, Result<[Option<Result<Measurements, FailedMeasurements>>; 2], io::Error>>,
    );
    impl Serialize for Year<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_map(
                self.0
                    .iter()
                    .filter_map(|(d, s)| s.as_ref().ok().map(|s| (d, Day(s)))),
            )
        }
    }

    struct Measures<'a>(
        &'a BTreeMap<
            u16,
            BTreeMap<u8, Result<[Option<Result<Measurements, FailedMeasurements>>; 2], io::Error>>,
        >,
    );
    impl Serialize for Measures<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_map(self.0.iter().map(|(y, s)| (y, Year(s))))
        }
    }

    let file = File::create(save_baseline).context("Cannot open save file")?;
    serde_json::to_writer(file, &Measures(measures)).context("Cannot serialize measurements")?;
    Ok(())
}

type Answers = BTreeMap<u16, BTreeMap<u8, [Option<String>; 2]>>;

fn read_answers(file: impl AsRef<Path>) -> anyhow::Result<Answers> {
    let file = File::open(file).context("Cannot open answer file")?;
    serde_json::from_reader(file).context("Cannot parse answer file")
}

#[derive(Debug, Clone)]
struct Measurements {
    answer: String,
    multiline: bool,
    time: Option<Duration>,
}

#[derive(Debug, Clone)]
struct FailedMeasurements {
    err: Option<Cow<'static, str>>,
}
impl FailedMeasurements {
    fn to_html(&self, node: Node<'_>) -> Result<(), fmt::Error> {
        let mut node = node.attr("class='part failed panic'");
        if let Some(msg) = self.err.as_ref() {
            writeln!(node, "{}", msg)?;
        } else {
            writeln!(node, "Solution panicked with unknown type")?;
        }
        Ok(())
    }
}

pub fn run<F, I, T>(build: F, args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    F: FnOnce(&mut Library),
{
    simple_logger::SimpleLogger::new()
        .without_timestamps()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    if let Err(err) = main(build, Args::parse_from(args)) {
        log::error!("Fatal error: {err:?}");
        exit(1)
    }
}
