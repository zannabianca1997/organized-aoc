use std::{
    collections::BTreeMap,
    fmt::{self, Write},
    io,
    ops::Add,
    rc::Rc,
    time::Duration,
};

use either::Either::{self, Left, Right};
use html_builder::{Html5, Node};

use crate::{filters::Filters, Answers, Baseline, Baselines, FailedMeasurements, Measurements};

#[derive(Debug, Clone)]
pub struct Report {
    problems: Rc<Filters>,
    years: BTreeMap<u16, Year>,
    totals: Option<Stats>,
}
impl Report {
    pub(crate) fn new(
        problems: Rc<Filters>,
        measures: BTreeMap<
            u16,
            BTreeMap<
                u8,
                Result<[Option<Result<Measurements, FailedMeasurements>>; 2], std::io::Error>,
            >,
        >,
        answers: Answers,
        baseline: Baselines,
    ) -> Self {
        let years: BTreeMap<_, _> = measures
            .into_iter()
            .map(|(y, sols)| {
                (
                    y,
                    Year::new(
                        y,
                        sols,
                        answers.get(&y).unwrap_or_else(|| {
                            static DEFAULT: BTreeMap<
                                u8,
                                [std::option::Option<std::string::String>; 2],
                            > = BTreeMap::new();
                            &DEFAULT
                        }),
                        baseline.get(&y).unwrap_or_else(|| {
                            static DEFAULT: BTreeMap<u8, [std::option::Option<Baseline>; 2]> =
                                BTreeMap::new();
                            &DEFAULT
                        }),
                    ),
                )
            })
            .collect();
        let totals = years.values().filter_map(|y| y.totals).reduce(Stats::add);
        Self {
            problems,
            years,
            totals,
        }
    }

    pub(crate) fn problems(&self) -> &Filters {
        self.problems.as_ref()
    }

    pub(crate) fn render(&self, node: &mut Node<'_>) -> Result<(), fmt::Error> {
        {
            let mut header = node.header();
            let mut title = header.h1().attr("class='report title'");
            writeln!(title, "Report for the query ")?;
            writeln!(title.code(), "\"{}\"", &self.problems)?;
        }

        if let Some(totals) = self.totals {
            let mut table = node.table().attr("class='report totals container'");
            let mut head = table.thead();
            writeln!(head.tr().th().attr("colspan='2'"), "Combined totals")?;
            let mut r2 = head.tr();
            writeln!(r2.th(), "Time")?;
            writeln!(r2.th(), "Ok")?;
            totals.to_html_table_cells(&mut table.tbody().tr(), 2, "report totals")?;
        }

        let mut years = node.div().attr("class='years container'");
        for year in self.years.values() {
            year.render(years.div())?
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Year {
    year: u16,
    days: BTreeMap<u8, Result<Day, LoadInputError>>,
    multilines: BTreeMap<MultilineDest, String>,
    totals: Option<Stats>,
}
impl Year {
    fn new(
        year: u16,
        sols: BTreeMap<
            u8,
            Result<[Option<Result<Measurements, FailedMeasurements>>; 2], io::Error>,
        >,
        answers: &BTreeMap<u8, [Option<String>; 2]>,
        baselines: &BTreeMap<u8, [Option<Baseline>; 2]>,
    ) -> Self {
        assert!(year >= 2015);

        let mut multilines = BTreeMap::new();
        let days: BTreeMap<_, _> = sols
            .into_iter()
            .map(|(d, s)| {
                (
                    d,
                    match s {
                        Ok(meas) => Ok(Day::new(
                            year,
                            d,
                            meas,
                            answers.get(&d).unwrap_or_else(|| &[None, None]),
                            baselines.get(&d).unwrap_or_else(|| &[None, None]),
                            |dest: MultilineDest, ans: String| {
                                multilines.insert(dest, ans);
                            },
                        )),
                        Err(err) => Err(LoadInputError { err: Rc::new(err) }),
                    },
                )
            })
            .collect();
        let totals = days
            .values()
            .filter_map(|d| d.as_ref().ok().map(|d| d.totals).flatten())
            .reduce(Stats::add);
        Self {
            year,
            days,
            multilines,
            totals,
        }
    }

    fn render(&self, node: Node<'_>) -> Result<(), fmt::Error> {
        let mut node = node.attr("class='year'");

        writeln!(node.h2().attr("class='year title'"), "Year {}", &self.year)?;

        let mut table = node.table().attr("class='days container'");
        {
            let mut head = table.thead();
            let mut r1 = head.tr();
            writeln!(r1.th().attr("rowspan='2'"), "Days")?;
            writeln!(r1.th().attr("colspan='3'"), "Part 1")?;
            writeln!(r1.th().attr("colspan='3'"), "Part 2")?;
            writeln!(r1.th().attr("colspan='2'"), "Total")?;
            let mut r2 = head.tr();
            for _ in 0..2 {
                writeln!(r2.th(), "Answer")?;
                writeln!(r2.th(), "Time")?;
                writeln!(r2.th(), "Ok")?;
            }
            writeln!(r2.th(), "Time")?;
            writeln!(r2.th(), "Ok")?;
        }
        {
            let mut body = table.tbody();
            for (d, day) in &self.days {
                match day {
                    Ok(day) => day.to_html(body.tr())?,
                    Err(err) => {
                        let mut row = body.tr().attr("class='day input-error'");
                        writeln!(row.th().attr("class='day number'"), "{}", d)?;
                        err.to_html(row.td().attr("colspan='8'"))
                    }?,
                }
            }
        }
        {
            let mut foot = table.tfoot();
            let mut row = foot.tr();
            writeln!(
                row.th().attr("class='year totals'").attr("colspan='7'"),
                "Totals"
            )?;

            if let Some(totals) = self.totals {
                totals.to_html_table_cells(&mut row, 2, "year totals")?;
            } else {
                writeln!(
                    row.td()
                        .attr("colspan='2'")
                        .attr("class='year totals missing'"),
                    "-"
                )?;
            }
        }

        // multilines
        if !self.multilines.is_empty() {
            let mut div = node.div().attr("class='multilines container'");
            writeln!(
                div.h3().attr("class='multilines title'"),
                "Multiline outputs"
            )?;
            for (dest, value) in &self.multilines {
                assert_eq!(dest.0, self.year);
                let mut div = div.div().attr("class='multiline container'");
                writeln!(
                    dest.set_id(div.h4().attr("class='multiline title'")),
                    "Day {} part {}",
                    dest.1,
                    dest.2
                )?;
                // writing the content with raw streams to avoid formatting
                let mut content = div.raw();
                write!(content, "<pre class='multiline content'><code>")?;
                let mut content = content.safe();
                write!(content, "{}", value)?;
                let mut content = content.raw();
                writeln!(content, "</code></pre>")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Day {
    day: u8,
    parts: [Option<Result<Part, FailedMeasurements>>; 2],
    totals: Option<Stats>,
}
impl Day {
    fn new(
        year: u16,
        day: u8,
        meas: [Option<Result<Measurements, FailedMeasurements>>; 2],
        answers: &[Option<String>; 2],
        baselines: &[Option<Baseline>; 2],
        mut multiline: impl FnMut(MultilineDest, String),
    ) -> Self {
        assert!(year >= 2015);
        assert!(day >= 1 && day <= 25);

        let mut f = |m: Option<Result<Measurements, FailedMeasurements>>, p: u8| {
            m.map(|m| {
                m.map(|m| {
                    Part::new(
                        year,
                        day,
                        p + 1,
                        m,
                        answers[p as usize].as_ref().map(String::as_str),
                        baselines[p as usize].as_ref().unwrap_or_else(|| {
                            static DEFAULT: Baseline = Baseline {
                                answer: None,
                                time: None,
                            };
                            &DEFAULT
                        }),
                        &mut multiline,
                    )
                })
            })
        };
        let [p1, p2] = meas;
        let parts = [f(p1, 0), f(p2, 1)];
        let totals = parts
            .iter()
            .filter_map(|p| p.as_ref().and_then(|r| r.as_ref().ok()).map(|p| p.stats))
            .reduce(Stats::add);
        Self { day, parts, totals }
    }

    fn to_html(&self, node: Node<'_>) -> Result<(), fmt::Error> {
        let mut node = node.attr("class='day'");
        writeln!(node.th().attr("class='day number'"), "{}", self.day)?;

        for part in &self.parts {
            match part {
                Some(Ok(part)) => part.to_html_table_cells(&mut node, 3)?,
                Some(Err(err)) => {
                    err.to_html(node.td().attr("colspan='3'"))?;
                }
                None => {
                    writeln!(
                        node.td().attr("colspan='3'").attr("class='part missing'"),
                        "-"
                    )?;
                }
            }
        }

        if let Some(totals) = self.totals {
            totals.to_html_table_cells(&mut node, 2, "day totals")?;
        } else {
            writeln!(
                node.td()
                    .attr("colspan='2'")
                    .attr("class='day totals missing'"),
                "-"
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct LoadInputError {
    err: Rc<io::Error>,
}
impl LoadInputError {
    fn to_html(&self, node: Node<'_>) -> Result<(), fmt::Error> {
        let mut node = node.attr("class='day input-error'");
        writeln!(node, "Error in reading input: {}", self.err)
    }
}

#[derive(Debug, Clone)]
struct Part {
    part: u8,
    answer: Either<String, MultilineRef>,
    stats: Stats,
}
impl Part {
    fn new(
        year: u16,
        day: u8,
        part: u8,
        Measurements {
            answer,
            time,
            multiline,
        }: Measurements,
        correct_answer: Option<&str>,
        Baseline {
            time: prev_time,
            answer: prev_answer,
        }: &Baseline,
        mut multiline_fun: impl FnMut(MultilineDest, String),
    ) -> Self {
        assert!(year >= 2015);
        assert!(day >= 1 && day <= 25);
        assert!(part == 1 || part == 2);

        let stats = Stats::new(
            &answer,
            prev_answer.as_ref().map(String::as_str),
            correct_answer,
            time,
            *prev_time,
        );
        let answer = if multiline {
            let (mref, mdest) = multiline_pairs(year, day, part);
            multiline_fun(mdest, answer);
            Right(mref)
        } else {
            Left(answer)
        };
        Self {
            part,
            answer,
            stats,
        }
    }

    fn to_html_table_cells(&self, row: &mut Node<'_>, cells: usize) -> Result<(), fmt::Error> {
        assert_eq!(cells, 3);
        let correctedness_class = match self.stats.correct {
            Some(true) => "correct",
            Some(false) => "wrong",
            None => "unknown-correctedness",
        };
        let part = match self.part {
            1 => "first",
            2 => "second",
            _ => unreachable!(),
        };
        match &self.answer {
            Left(s) => writeln!(
                row.td().attr(&format!(
                    "class='part {part} answer {correctedness_class} no-multiline'"
                )),
                "{}",
                s
            )?,
            Right(mref) => mref.to_html(row.td().attr(&format!(
                "class='part {part} answer {correctedness_class} multiline'"
            )))?,
        }
        self.stats
            .to_html_table_cells(row, 2, &format!("part {part}"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Stats {
    correct: Option<bool>,
    was_correct: Option<bool>,

    time: Option<Duration>,
    previous_time: Option<Duration>,
}
impl Stats {
    fn new(
        answer: &str,
        prev_answer: Option<&str>,
        correct_answer: Option<&str>,
        time: Option<Duration>,
        prev_time: Option<Duration>,
    ) -> Self {
        Self {
            correct: correct_answer.map(|ca| answer == ca),
            was_correct: correct_answer.and_then(|ca| prev_answer.map(|pa| ca == pa)),
            time,
            previous_time: prev_time,
        }
    }

    fn to_html_table_cells(
        &self,
        row: &mut Node<'_>,
        cells: usize,
        additional_td_classes: &str,
    ) -> Result<(), fmt::Error> {
        assert_eq!(cells, 2);
        let correctedness_class = match self.correct {
            Some(true) => "correct",
            Some(false) => "wrong",
            None => "unknown-correctedness",
        };
        {
            let mut time_td = row.td().attr(&format!(
                "class='{additional_td_classes} time {correctedness_class} {}'",
                if self.time.is_some() {
                    "known"
                } else {
                    "missing"
                }
            ));
            if let Some(time) = self.time {
                writeln!(time_td, "{}", humantime::format_duration(time))?;
                if let Some(previous_time) = self.previous_time {
                    time_td.br();
                    if previous_time <= time {
                        writeln!(
                            time_td.em().attr("class='time-diff slower'"),
                            "[+ {}]",
                            humantime::format_duration(time - previous_time)
                        )?;
                    } else {
                        writeln!(
                            time_td.em().attr("class='time-diff faster'"),
                            "[- {}]",
                            humantime::format_duration(previous_time - time)
                        )?;
                    }
                }
            } else {
                writeln!(time_td, "-")?;
            }
        }
        {
            let mut check = row
                .td()
                .attr(&format!(
                    "class='{additional_td_classes} check {correctedness_class}'"
                ))
                .raw(); // do not escape my character codes
            if let Some(correct) = self.correct {
                if correct {
                    write!(check, "&#x2714;")?;
                } else {
                    write!(check, "&#x2718;")?;
                }
                if let Some(was_correct) = self.was_correct {
                    if was_correct != correct {
                        if was_correct {
                            write!(check, "(&#x2714;)")?;
                        } else {
                            write!(check, "(&#x2718;)")?;
                        }
                    }
                }
                writeln!(check)?;
            } else {
                writeln!(check, "-")?;
            }
        }

        Ok(())
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            correct: self.correct.map(|s| rhs.correct.map(|r| s && r)).flatten(),
            was_correct: self
                .was_correct
                .map(|s| rhs.was_correct.map(|r| s && r))
                .flatten(),
            time: self.time.map(|s| rhs.time.map(|r| s + r)).flatten(),
            previous_time: self
                .previous_time
                .map(|s| rhs.previous_time.map(|r| s + r))
                .flatten(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct MultilineRef(u16, u8, u8);
impl MultilineRef {
    fn to_html(&self, mut node: Node<'_>) -> Result<(), fmt::Error> {
        writeln!(
            node.a().attr(&format!(
                "href='#multiline_{}_{}_{}'",
                self.0, self.1, self.2
            )),
            "multiline"
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct MultilineDest(u16, u8, u8);
impl MultilineDest {
    fn set_id<'n>(&self, node: Node<'n>) -> Node<'n> {
        node.attr(&format!("id='multiline_{}_{}_{}'", self.0, self.1, self.2))
    }
}

fn multiline_pairs(year: u16, day: u8, part: u8) -> (MultilineRef, MultilineDest) {
    assert!(year >= 2015);
    assert!(day >= 1 && day <= 25);
    assert!(part == 1 || part == 2);
    (
        MultilineRef(year, day, part),
        MultilineDest(year, day, part),
    )
}
