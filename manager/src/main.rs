#![feature(iterator_try_collect)]

use std::{
    collections::BTreeMap,
    fmt::Write,
    fs::{self, create_dir_all},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{bail, Context};
use cargo_manifest::{Dependency, DependencyDetail, Manifest};
use clap::{Parser, Subcommand};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use syn::PathSegment;
use toml::toml;

#[derive(Debug, Parser)]
struct Args {
    /// Solutions folder
    #[clap(short, long, default_value = "./solutions")]
    solutions: PathBuf,
    /// Generated library folder [defaults to "library" in the same dir than the solutions]
    #[clap(short, long)]
    library: Option<PathBuf>,
    /// Verbosity of the output
    #[clap(short, action = clap::ArgAction::Count)]
    verbosity: u8,
    /// What to do
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand, Default)]
enum Command {
    /// Initialize a day with a skeleton
    Init {
        /// Year of the new solution
        year: u16,
        /// Day of the new solution
        day: u8,
        /// Do not update the library
        #[clap(long)]
        do_not_update: bool,
    },
    /// Update the library with all day discovered
    #[default]
    Update,
}

fn main() -> anyhow::Result<()> {
    let Args {
        solutions,
        library,
        command,
        verbosity,
    } = Args::parse();

    setup_logger(verbosity)?;

    let solutions = visit_solutions(solutions).context("Cannot read solution directory")?;

    match command.unwrap_or_default() {
        Command::Init {
            year,
            day,
            do_not_update,
        } => {
            init(&solutions, year, day).context("Cannot init day")?;
            if !do_not_update {
                update(&solutions, library).context("Cannot update library")?;
            }
        }
        Command::Update => update(&solutions, library).context("Cannot update library")?,
    }
    Ok(())
}

#[derive(Clone)]
struct Solutions {
    path: PathBuf,
    years: BTreeMap<u16, Year>,
}

#[derive(Clone)]
struct Year {
    days: BTreeMap<u8, SolutionMetas>,
}

#[derive(Clone)]
struct SolutionMetas {
    path: PathBuf,
    part1: Option<syn::Path>,
    part2: Option<syn::Path>,
}
#[derive(Deserialize, Serialize, Default)]
struct CargoMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    part1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    part2: Option<String>,
}

fn visit_solution(year: u16, day: u8, path: PathBuf) -> anyhow::Result<SolutionMetas> {
    let manifest: Manifest<CargoMetadata> =
        cargo_manifest::Manifest::from_path_with_metadata(path.join("Cargo.toml"))
            .context("Cannot parse Cargo.toml")?;

    // checks package name and type
    {
        let Some(name) = manifest.package.as_ref().map(|pkg| &pkg.name) else {bail!("The solution crate must be a package")};
        if name != &format!("aoc_{year}_{day}") {
            bail!(
                "The name of the solution package must be 'aoc_{year}_{day}', found instead {name}"
            )
        }
    }

    let CargoMetadata { part1, part2 } = manifest.package.unwrap().metadata.unwrap_or_default();
    let part1 = part1
        .map(|p| syn::parse_str(&p).context("part1 is not a valid rust path"))
        .transpose()?;
    let part2 = part2
        .map(|p| syn::parse_str(&p).context("part2 is not a valid rust path"))
        .transpose()?;

    Ok(SolutionMetas { path, part1, part2 })
}

fn visit_solutions(solutions: PathBuf) -> anyhow::Result<Solutions> {
    if !solutions.exists() {
        return Ok(Solutions {
            path: solutions,
            years: BTreeMap::new(),
        });
    }
    log::info!("Visiting solution folder '{}'", solutions.display());

    let mut sols = BTreeMap::new();
    for year_dir in solutions.read_dir()? {
        let year_dir = year_dir?;
        if let Some(year) = year_dir
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.parse().ok())
        {
            let mut days = BTreeMap::new();

            for day_dir in year_dir
                .path()
                .read_dir()
                .context(format!("Error in reading year {year}"))?
            {
                let day_dir = day_dir.context(format!("Error in reading year {year}"))?;
                if let Some(day) = day_dir
                    .path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|n| n.parse().ok())
                {
                    log::trace!(
                        "Found year {year} day {day} at '{}'",
                        day_dir.path().display()
                    );
                    match visit_solution(year, day, day_dir.path()) {
                        Ok(metas) => {
                            log::info!("Found solution for year {year}, day {day}");
                            days.insert(day, metas);
                        }
                        Err(err) => {
                            let err = err.context(format!(
                                "Cannot parse solution for year {year}, day {day} from {}",
                                day_dir.path().display()
                            ));
                            let mut ind_err = String::new();
                            write!(indenter::indented(&mut ind_err), "{err:?}").unwrap();
                            log::warn!("Ignoring solution for year {year}, day {day}\n{ind_err}")
                        }
                    }
                } else {
                    log::trace!("Ignoring non-day item '{}'", day_dir.path().display())
                }
            }
            sols.insert(year, Year { days });
        } else {
            log::trace!("Ignoring non-year item '{}'", year_dir.path().display())
        }
    }

    Ok(Solutions {
        path: solutions,
        years: sols,
    })
}

fn init(solutions: &Solutions, year: u16, day: u8) -> anyhow::Result<()> {
    if solutions
        .years
        .get(&year)
        .and_then(|y| y.days.get(&day))
        .is_some()
    {
        bail!("Solution already exist.")
    }

    let path = solutions.path.join(year.to_string()).join(day.to_string());
    log::info!(
        "Initializing solution for year {year}, day {day} in {}",
        path.display()
    );

    create_dir_all(&path)?;

    // Cargo.toml
    {
        log::trace!("Writing Cargo.toml");
        let name = format!("aoc_{year}_{day}");
        let manifest = toml::to_string_pretty(&toml!(
            [package]
            name = name
            version = "0.1.0"
            edition = "2021"

            [package.metadata.aoc]
            part1 = "part1"

            [dependencies]
        ))
        .unwrap();
        fs::write(path.join("Cargo.toml"), manifest)?;
    }

    // src/lib.rs
    {
        log::trace!("Writing src/lib.rs");

        let src = path.join("src");
        create_dir_all(&src)?;

        let doc = syn::parse_file(&format!("//! Aoc year {year} day {day}")).unwrap();
        let lib_rs = prettyplease::unparse(
            &syn::parse2(quote!(
                #doc

                pub fn part1(input: &str) -> i64 {
                    todo!()
                }
            ))
            .unwrap(),
        );

        fs::write(src.join("lib.rs"), lib_rs)?;
    }

    Ok(())
}

fn update(solutions: &Solutions, library: Option<PathBuf>) -> anyhow::Result<()> {
    let library = if let Some(library) = library {
        library
    } else {
        solutions
            .path
            .parent()
            .ok_or_else(|| {
                anyhow::format_err!(
                    "'{}' does not have a parent directory",
                    solutions.path.display()
                )
            })
            .context("Cannot choose library directory")?
            .join("library")
    }
    .canonicalize()
    .context("Cannot canonicalize library path")?;
    let lib_ref = &library;

    log::info!("Updating library folder in {}", library.display());

    // creating directory
    create_dir_all(&library)?;

    // Cargo.toml
    {
        log::trace!("Writing 'Cargo.toml'");
        let mut manifest = Manifest::from_str(
            r#"
            [package]
            name = "aoc_library"
            version = "0.1.0"
            edition = "2021"
        "#,
        )
        .unwrap();
        manifest.dependencies = Some(
            solutions
                .years
                .iter()
                .flat_map(|(year, Year { days, .. })| {
                    days.iter().map(
                        move |(day, SolutionMetas { path, .. })| -> anyhow::Result<_> {
                            let path = path
                                .canonicalize()
                                .context("Cannot canonicalize solution path")?;
                            Ok((
                                format!("aoc_{year}_{day}"),
                                Dependency::Detailed(DependencyDetail {
                                    path: Some(
                                        pathdiff::diff_paths(path, lib_ref)
                                            .expect("both path should be absolute")
                                            .to_str()
                                            .context("Dependency has a non-utf8 path")?
                                            .to_owned(),
                                    ),
                                    ..Default::default()
                                }),
                            ))
                        },
                    )
                })
                .try_collect()
                .context("Error in processing dependencies")?,
        );

        fs::write(
            library.join("Cargo.toml"),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .context("Cannot write `Cargo.toml`")?;
    }

    let src = library.join("src");
    create_dir_all(&src)?;

    // lib.rs
    {
        log::trace!("Writing 'src/lib.rs'");

        let years = solutions.years.iter().map(|(year, Year { days, .. })| {
            log::trace!("Writing year {year}");
            let days = days
                .into_iter()
                .map(|(day, SolutionMetas { part1, part2, .. })| {
                    log::trace!("Writing day {day}");
                    let [part1, part2] = [part1, part2].map(|part| {
                        if let Some(part) = part {
                            let sol_crate = PathSegment {
                                ident: format_ident!("aoc_{}_{}", year, day),
                                arguments: syn::PathArguments::None,
                            };
                            let path = Some(&sol_crate).into_iter().chain(part.segments.iter());
                            quote!(Some((::#(#path)::* as fn(&str) -> _).into()))
                        } else {
                            quote!(None)
                        }
                    });
                    quote!(
                        (
                            #day,
                            Day {
                                part1: #part1,
                                part2: #part2,
                            }
                        )
                    )
                });
            quote!((
                #year,
                BTreeMap::from([
                    #( #days ),*
                ])
            ))
        });

        let code = quote!(
            //! Auto generated library code

            use std::collections::BTreeMap;

            /// Generic solution
            pub enum Solution {
                /// Numerical solution
                Numeric(fn(&str) -> i64),
                /// Alphanumerical solution
                Alpha(fn(&str) -> String),
            }

            impl From<fn(&str) -> i64> for Solution {
                fn from(value: fn(&str) -> i64) -> Self {
                    Self::Numeric(value)
                }
            }

            impl From<fn(&str) -> String> for Solution {
                fn from(value: fn(&str) -> String) -> Self {
                    Self::Alpha(value)
                }
            }

            /// Solution for an entire day
            pub struct Day {
                pub part1: Option<Solution>,
                pub part2: Option<Solution>,
            }

            pub fn library() -> BTreeMap<u16, BTreeMap<u8, Day>> {
                BTreeMap::from([
                    #( #years ),*
                ])
            }

        );

        let code = prettyplease::unparse(&syn::parse2(code).unwrap());
        fs::write(src.join("lib.rs"), code)?;
    }

    Ok(())
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
