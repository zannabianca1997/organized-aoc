#!/bin/env python3

# Update the project structure discovering new solutions and links them to the runner


from dataclasses import dataclass
from pathlib import Path
import toml
from os.path import relpath

from logging import getLogger

logger = getLogger(__name__)


@dataclass
class Part:
    fun: str
    numeric: bool = True
    multiline: bool = False


@dataclass
class Day:
    name: str
    path: Path
    part1: None | Part = None
    part2: None | Part = None

    @staticmethod
    def parse(path: Path) -> "Day | None":
        try:
            manifest = toml.load(path.joinpath("Cargo.toml"))
        except toml.TomlDecodeError:
            return None

        name = manifest["package"]["name"]

        try:
            aoc = manifest["package"]["metadata"]["aoc"]
        except KeyError:
            return Day(path)

        try:
            part1 = aoc["part1"]
        except KeyError:
            part1 = None
        else:
            part1 = Part(**part1)

        try:
            part2 = aoc["part2"]
        except KeyError:
            part2 = None
        else:
            part2 = Part(**part2)

        return Day(name, path, part1, part2)


def pathdiff(path: Path, base: Path) -> Path:
    return Path(relpath(path, base))


def update(
    workspace: Path = Path("./Cargo.toml"),
    runner: Path = Path("./runner"),
    solutions: Path = Path("./solutions"),
    library: Path = Path("./library"),
):
    """Update the project to include new solutions"""
    workspace = Path(workspace).resolve()
    runner = Path(runner).resolve()
    solutions = Path(solutions).resolve()
    library = Path(library).resolve()

    logger.info("Finding solution packages")
    solutions_pkgs: dict[int, dict[int, Day]] = {}
    for year_dir in solutions.iterdir():
        try:
            year = int(year_dir.name)
        except ValueError:
            continue
        if year >= 2015:
            for day_dir in year_dir.iterdir():
                try:
                    day = int(day_dir.name)
                except ValueError:
                    continue
                if 0 < day <= 25:
                    sol = Day.parse(day_dir)
                    if day is not None:
                        if year not in solutions_pkgs:
                            solutions_pkgs[year] = {}
                        solutions_pkgs[year][day] = sol

    logger.info("Generating library")
    library.mkdir(parents=True, exist_ok=True)
    deps = {
        pkg.name: {"path": str(pathdiff(pkg.path, library))}
        for sols in solutions_pkgs.values()
        for pkg in sols.values()
    }
    runner_name = toml.load(runner / "Cargo.toml")["package"]["name"]
    deps[runner_name] = {"path": str(pathdiff(runner, library))}
    manifest = {
        "package": {"name": "library", "version": "0.1.0", "edition": "2021"},
        "dependencies": deps,
    }
    with open(library / "Cargo.toml", "w") as lib_manifest:
        toml.dump(manifest, lib_manifest)
    src = library / "src"
    src.mkdir(exist_ok=True)
    main_rs = src / "main.rs"
    with open(main_rs, "w") as main_rs:
        print(f"fn main(){{::{runner_name}::run(|l|{{l", file=main_rs, end="")
        for year, sols in solutions_pkgs.items():
            print(f".add_year({year},|y|{{y", file=main_rs, end="")
            for day, pkg in sols.items():
                print(f".add_day({day},|d|{{d", file=main_rs, end="")
                if pkg.part1 is not None:
                    if pkg.part1.numeric:
                        variant = "Numeric"
                    elif pkg.part1.multiline:
                        variant = "Multiline"
                    else:
                        variant = "Alpha"
                    print(
                        f".add_part_1(::{runner_name}::Part::{variant}(::{pkg.name}::{pkg.part1.fun}))",
                        file=main_rs,
                        end="",
                    )
                if pkg.part2 is not None:
                    if pkg.part2.numeric:
                        variant = "Numeric"
                    elif pkg.part2.multiline:
                        variant = "Multiline"
                    else:
                        variant = "Alpha"
                    print(
                        f".add_part_2(::{runner_name}::Part::{variant}(::{pkg.name}::{pkg.part2.fun}))",
                        file=main_rs,
                        end="",
                    )
                print(";})", file=main_rs, end="")
            print(";})", file=main_rs, end="")
        print(";},::std::env::args_os())}", file=main_rs, end="")

    logger.info("Updating workspace Cargo.toml")
    workspace_content = toml.load(workspace)
    members: list[str] = workspace_content["workspace"]["members"]
    new_members: list[str] = []
    for member in members:
        if solutions not in workspace.parent.joinpath(member).resolve().parents:
            new_members.append(member)
    for sols in solutions_pkgs.values():
        for pkg in sols.values():
            new_members.append(str(pathdiff(pkg.path, workspace.parent)))
    library_str = str(pathdiff(library, workspace.parent))
    if library_str not in new_members:
        new_members.append(library_str)
    workspace_content["workspace"]["members"] = new_members
    with open(workspace, "w") as workspace_manifest:
        toml.dump(workspace_content, workspace_manifest)


if __name__ == "__main__":
    from logging import basicConfig, INFO

    basicConfig(level=INFO)

    from argparse import ArgumentParser

    parser = ArgumentParser(
        description="A wrapper for cargo, that update the solution library and runs cargo with the remaining args",
        usage="%(prog)s [-h] [--workspace WORKSPACE] [--runner RUNNER] [--solutions SOLUTIONS] [--library LIBRARY] [CARGO_ARGS]",
    )
    parser.add_argument(
        "--workspace", action="store", help="Path to the workspace Cargo.toml"
    )
    parser.add_argument("--runner", action="store", help="Path to the runner library")
    parser.add_argument(
        "--solutions", action="store", help="Path to the solution library"
    )
    parser.add_argument(
        "--library", action="store", help="Where to generate the library"
    )
    parser.add_argument(
        "--cargo", action="store", help="Cargo command", default="cargo"
    )
    (args, cargo_args) = parser.parse_known_args()

    update(
        **{
            name: value
            for name, value in args.__dict__.items()
            if value is not None
            and name in ["runner", "solutions", "library", "workspace"]
        }
    )

    from subprocess import run

    cargo_res = run(args=[args.cargo, *cargo_args])
    exit(cargo_res.returncode)
