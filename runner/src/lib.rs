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
pub enum Part {
    Numeric(fn(&str) -> i64),
    Alpha(fn(&str) -> String),
    Multiline(fn(&str) -> String),
}
impl Part {
    fn measure(
        &self,
        repeats: Option<NonZeroU32>,
        input: &str,
    ) -> Result<Measurements, FailedMeasurements> {
        catch_unwind(|| {
            // first run it once to find the answer
            let answer = match self {
                Part::Numeric(fun) => (fun)(input).to_string(),
                Part::Alpha(fun) | Part::Multiline(fun) => (fun)(input),
            };
            let time = if let Some(repeats) = repeats {
                Some(
                    match self {
                        Part::Numeric(fun) => {
                            let start = Instant::now();
                            for _ in 0..repeats.get() {
                                black_box((fun)(black_box(input)));
                            }
                            start.elapsed()
                        }
                        Part::Alpha(fun) | Part::Multiline(fun) => {
                            let start = Instant::now();
                            for _ in 0..repeats.get() {
                                black_box((fun)(black_box(input)));
                            }
                            start.elapsed()
                        }
                    } / repeats.get(),
                )
            } else {
                None
            };

            Measurements {
                answer,
                time,
                multiline: self.is_multiline(),
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

    /// Returns `true` if the part is [`Multiline`].
    ///
    /// [`Multiline`]: Part::Multiline
    #[must_use]
    pub fn is_multiline(&self) -> bool {
        matches!(self, Self::Multiline(..))
    }
}

#[derive(Debug)]

pub struct Day {
    day: (u16, u8),
    parts: [Option<Part>; 2],
    filters: Rc<Filters>,
}
impl Day {
    pub fn add_part_1(&mut self, sol: Part) -> &mut Self {
        if self.filters.accept_part(self.day.0, self.day.1, 1) {
            self.parts[0] = Some(sol);
        }
        self
    }
    pub fn add_part_2(&mut self, sol: Part) -> &mut Self {
        if self.filters.accept_part(self.day.0, self.day.1, 2) {
            self.parts[1] = Some(sol);
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
        let input = read_to_string(
            inputs
                .join(self.day.0.to_string())
                .join(self.day.1.to_string()),
        )?;
        Ok(self.parts.map(|p| p.map(|p| p.measure(repeats, &input))))
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
            (build)(self.solutions.entry(day).or_insert_with(|| Day {
                parts: [None; 2],
                day: (self.year, day),
                filters: self.filters.clone(),
            }));
        }
        self
    }

    fn measure(
        &self,
        repeats: Option<NonZeroU32>,
        inputs: &Path,
    ) -> BTreeMap<u8, io::Result<[Option<Result<Measurements, FailedMeasurements>>; 2]>> {
        self.solutions
            .iter()
            .map(|(day, sols)| (*day, sols.measure(repeats, inputs)))
            .collect()
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
            (build)(self.solutions.entry(year).or_insert_with(|| Year {
                year,
                solutions: BTreeMap::new(),
                filters: self.filters.clone(),
            }));
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
    // building library
    let library = {
        let mut lib = Library {
            solutions: BTreeMap::new(),
            filters: Rc::new(problems),
        };
        (build)(&mut lib);
        lib
    };
    // executing tests
    let measures = library.measure(repeats, &inputs);
    // saving baselines
    if let Some(save_baseline) = save_baseline {
        if let Err(err) = dump_baseline(&save_baseline, &measures) {
            log::warn!("Failed to save baselines: {err:?}")
        }
    }

    // building a report
    let report = Report::new(library.filters, measures, answers, baseline);

    print!(
        "{}",
        html_page(report).expect("Writing in a html buffer should be error_free")
    );
    Ok(())
}

fn html_page(report: Report) -> Result<impl Display, fmt::Error> {
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
    if let Err(err) = main(build, Args::parse_from(args)) {
        log::error!("Fatal error: {err:?}");
        exit(1)
    }
}
