use anyhow::{bail, Context};
use std::{collections::BTreeSet, fmt::Display, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum Range<T> {
    #[default]
    All,
    From(T),
    To(T),
    Between(T, T),
}
impl<T> Range<T>
where
    T: Ord,
{
    fn accept(&self, value: T) -> bool {
        match self {
            Range::All => true,
            Range::From(a) => a <= &value,
            Range::To(b) => &value <= b,
            Range::Between(a, b) => a <= &value && &value <= b,
        }
    }
}
impl<T> FromStr for Range<T>
where
    T: FromStr + Clone,
    T::Err: Send + Sync + 'static + std::error::Error,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s.split_once("..").map(|(a, b)| (a.trim(), b.trim())) {
                Some(("", "")) => Self::All,
                Some(("", b)) => Self::To(b.parse().context("Cannot parse end value")?),
                Some((a, "")) => Self::From(a.parse().context("Cannot parse starting value")?),
                Some((a, b)) => Self::Between(
                    a.parse().context("Cannot parse starting value")?,
                    b.parse().context("Cannot parse end value")?,
                ),
                None => {
                    let a: T = s.parse().context("Cannot parse value")?;
                    Self::Between(a.clone(), a)
                }
            },
        )
    }
}
impl<T> Display for Range<T>
where
    T: Display + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Range::All => write!(f, "..")?,
            Range::From(a) => {
                a.fmt(f)?;
                write!(f, "..")?;
            }
            Range::To(b) => {
                write!(f, "..")?;
                b.fmt(f)?
            }
            Range::Between(a, b) if a == b => a.fmt(f)?,
            Range::Between(a, b) => {
                a.fmt(f)?;
                write!(f, "..")?;
                b.fmt(f)?
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum PartFilter {
    First,
    Second,
    #[default]
    Both,
}
impl PartFilter {
    fn accept(&self, part: u8) -> bool {
        assert!(part == 1 || part == 2);
        match self {
            PartFilter::First => part == 1,
            PartFilter::Second => part == 2,
            PartFilter::Both => true,
        }
    }
}
impl FromStr for PartFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(
            match s.split_once("..").map(|(a, b)| (a.trim(), b.trim())) {
                Some(("", "1")) | Some(("1", "1")) => Self::First,
                Some(("2", "")) | Some(("2", "2")) => Self::Second,
                Some(("", "2")) | Some(("1", "2")) | Some(("1", "")) => Self::Both,
                Some(("2", "1")) => bail!("Invalid part order"),
                Some(("" | "1" | "2", _)) => bail!("Unrecognized end part"),
                Some((_, "" | "1" | "2")) => bail!("Unrecognized start part"),
                Some((_, _)) => bail!("Unrecognized limit parts"),
                None => match s {
                    "1" => Self::First,
                    "2" => Self::Second,
                    _ => bail!("Unrecognized part"),
                },
            },
        )
    }
}
impl Display for PartFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartFilter::First => write!(f, "1"),
            PartFilter::Second => write!(f, "2"),
            PartFilter::Both => write!(f, ".."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimpleFilter {
    years: Range<u16>,
    day: Range<u8>,
    part: PartFilter,
}
impl SimpleFilter {
    fn accept_year(&self, year: u16) -> bool {
        self.years.accept(year)
    }

    fn accept_day(&self, year: u16, day: u8) -> bool {
        self.accept_year(year) && self.day.accept(day)
    }

    fn accept_part(&self, year: u16, day: u8, part: u8) -> bool {
        self.accept_day(year, day) && self.part.accept(part)
    }
}
impl FromStr for SimpleFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, "::").map(str::trim);
        Ok(Self {
            years: parts
                .next()
                .map(str::parse)
                .transpose()
                .context("Cannot parse year range")?
                .unwrap_or_default(),
            day: parts
                .next()
                .map(str::parse)
                .transpose()
                .context("Cannot parse day range")?
                .unwrap_or_default(),
            part: parts
                .next()
                .map(str::parse)
                .transpose()
                .context("Cannot parse part filter")?
                .unwrap_or_default(),
        })
    }
}
impl Display for SimpleFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.years.fmt(f)?;
        if self.day != Range::All || self.part != PartFilter::Both {
            if f.alternate() {
                write!(f, " :: ")
            } else {
                write!(f, "::")
            }?;
            self.day.fmt(f)?;
        }
        if self.part != PartFilter::Both {
            if f.alternate() {
                write!(f, " :: ")
            } else {
                write!(f, "::")
            }?;
            self.part.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Filters(BTreeSet<SimpleFilter>);
impl Filters {
    pub(crate) fn accept_year(&self, year: u16) -> bool {
        self.0.iter().any(|f| f.accept_year(year))
    }

    pub(crate) fn accept_day(&self, year: u16, day: u8) -> bool {
        self.0.iter().any(|f| f.accept_day(year, day))
    }

    pub(crate) fn accept_part(&self, year: u16, day: u8, part: u8) -> bool {
        self.0.iter().any(|f| f.accept_part(year, day, part))
    }
}
impl FromStr for Filters {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("|").map(str::trim).map(str::parse).try_collect()?,
        ))
    }
}
impl Display for Filters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .format(if f.alternate() { " | " } else { "|" })
            .fmt(f)
    }
}
impl Default for Filters {
    fn default() -> Self {
        Self(BTreeSet::from([Default::default()]))
    }
}
