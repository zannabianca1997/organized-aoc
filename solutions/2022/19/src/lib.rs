
#[cfg(feature="2022_19_trace")]
use std::fmt::Display;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost_ore: usize,
    clay_robot_cost_ore: usize,
    obsidian_robot_cost_ore: usize,
    obsidian_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obsidian: usize,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    lazy_static! {
        static ref BLUEPRINT_RE: Regex = Regex::new(
            r"(?m)^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"
        ).unwrap();
    }
    let mut blueprints = vec![];
    for (i, captures) in BLUEPRINT_RE.captures_iter(input).enumerate() {
        if i + 1 != captures[1].parse().unwrap() {
            panic!(
                "Mismatched blueprint order: blueprint {} is in place {}",
                &captures[1],
                i + 1
            )
                    }
        blueprints.push(Blueprint {
            ore_robot_cost_ore: captures[2].parse().unwrap(),
            clay_robot_cost_ore: captures[3].parse().unwrap(),
            obsidian_robot_cost_ore: captures[4].parse().unwrap(),
            obsidian_robot_cost_clay: captures[5].parse().unwrap(),
            geode_robot_cost_ore: captures[6].parse().unwrap(),
            geode_robot_cost_obsidian: captures[7].parse().unwrap(),
        })
    }
    blueprints
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Wait(usize),
    BuildOreBot,
    BuildClayBot,
    BuildObsidianBot,
    BuildGeodeBot,
    BuildGeodeBotsFor(usize),
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature="2022_19_trace")]
struct LogItem {
    /// Minute ot this item
    minute: usize,
    /// Inventary at the START of this item
    robots: (usize, usize, usize, usize),
    /// Bots at the START of this item
    inventory: (usize, usize, usize, usize),
    /// Action taken at the START of this item
    action: Action,
}

#[derive(Debug, Clone)]
#[cfg(feature="2022_19_trace")]
struct Log<'bb> {
    trace: Vec<LogItem>,
    blueprint: &'bb Blueprint,
}
#[cfg(feature="2022_19_trace")]
impl<'bb> Log<'bb> {
    fn empty(blueprint: &'bb Blueprint) -> Self {
        Self {
            trace: vec![],
            blueprint,
        }
    }
}
#[cfg(feature="2022_19_trace")]
impl Log<'_> {
    fn extended(
        &self,
        minute: usize,
        robots: (usize, usize, usize, usize),
        inventory: (usize, usize, usize, usize),
        action: Action,
    ) -> Self {
        let mut new_log = self.clone();
        new_log.trace.push(LogItem {
            minute,
            robots,
            inventory,
            action,
        });
        new_log
    }
}
#[cfg(feature="2022_19_trace")]
impl Display for Log<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Log { trace, blueprint } = self;
        if trace.len() == 0 {
            return Ok(());
        }
        let Blueprint {
            ore_robot_cost_ore,
            clay_robot_cost_ore,
            obsidian_robot_cost_ore,
            obsidian_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obsidian,
        } = *blueprint;
        for LogItem {
            minute,
            robots: (ore_bots, clay_bots, obsidian_bots, geode_bots),
            inventory: (mut ore, mut clay, mut obsidian, mut geode),
            action,
        } in trace
        {
            if let Action::Wait(time) = action {
                for minute in (*minute)..(*minute + time) {
                    writeln!(f, "== Minute {} ==", minute)?;
                    if *ore_bots > 0 {
                        ore += ore_bots;
                        writeln!(
                        f,
                        "{ore_bots} ore-collecting robot collects {ore_bots} ore; you now have {ore} ore.",
                         
                    )?;

                    }
                    if *clay_bots > 0 {
                        clay += clay_bots;
                        writeln!(
                        f,
                        "{clay_bots} clay-collecting robot collects {clay_bots} clay; you now have {clay} clay.",
                         
                    )?
                    }
                    if *obsidian_bots > 0 {
                        obsidian += obsidian_bots;
                        writeln!(
                        f,
                        "{obsidian_bots} obsidian-collecting robot collects {obsidian_bots} obsidian; you now have {obsidian} obsidian.",
                    )?
                    }
                    if *geode_bots > 0 {
                        geode += geode_bots;
                        writeln!(
                        f,
                        "{geode_bots} geode-collecting robot collects {geode_bots} geode; you now have {geode} geode.",
                    )?
                    }
                    writeln!(f)?;
                }
            } else {
                writeln!(f, "== Minute {} ==", minute)?;
                match action {
                    Action::Wait(_) => unreachable!(),
                    Action::BuildOreBot => {
                        writeln!(
                        f,
                        "Spend {ore_robot_cost_ore} ore to start building a ore-collecting robot."
                    )?;
                        ore -= ore_robot_cost_ore;
                    }
                    Action::BuildClayBot => {
                        writeln!(
                        f,
                        "Spend {clay_robot_cost_ore} ore to start building a clay-collecting robot."
                    )?;
                        ore -= clay_robot_cost_ore;
                    }
                    Action::BuildObsidianBot => {
                        writeln!(
                        f,
                        "Spend {obsidian_robot_cost_ore} ore and {obsidian_robot_cost_clay} clay to start building a obsidian-collecting robot."
                    )?;
                        ore -= obsidian_robot_cost_ore;
                        clay -= obsidian_robot_cost_clay;
                    }
                    Action::BuildGeodeBot => {
                        writeln!(
                        f,
                        "Spend {geode_robot_cost_ore} ore and {geode_robot_cost_obsidian} obsidian to start building a geode-collecting robot."
                    )?;
                        ore -= ore_robot_cost_ore;
                        obsidian -= geode_robot_cost_obsidian;
                    }
                };
                if *ore_bots > 0 {
                    writeln!(
                    f,
                    "{ore_bots} ore-collecting robot collects {ore_bots} ore; you now have {} ore.",
                    ore + ore_bots
                )?
                }
                if *clay_bots > 0 {
                    writeln!(
                    f,
                    "{clay_bots} clay-collecting robot collects {clay_bots} clay; you now have {} clay.",
                    clay + clay_bots
                )?
                }
                if *obsidian_bots > 0 {
                    writeln!(
                    f,
                    "{obsidian_bots} obsidian-collecting robot collects {obsidian_bots} obsidian; you now have {} obsidian.",
                    obsidian + obsidian_bots
                )?
                }
                if *geode_bots > 0 {
                    writeln!(
                    f,
                    "{geode_bots} geode-collecting robot collects {geode_bots} geode; you now have {} geode.",
                    geode + geode_bots
                )?
                }
                match action {
                    Action::Wait(_) => unreachable!(),
                    Action::BuildOreBot => writeln!(
                        f,
                        "The new ore-collecting robot is ready; you now have {} of them.",
                        ore_bots + 1
                    )?,
                    Action::BuildClayBot => writeln!(
                        f,
                        "The new clay-collecting robot is ready; you now have {} of them.",
                        clay_bots + 1
                    )?,
                    Action::BuildObsidianBot => writeln!(
                        f,
                        "The new obsidian-collecting robot is ready; you now have {} of them.",
                        obsidian_bots + 1
                    )?,
                    Action::BuildGeodeBot => writeln!(
                        f,
                        "The new geode-collecting robot is ready; you now have {} of them.",
                        geode_bots + 1
                    )?,
                };
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(not(feature="2022_19_trace"))]
struct Log;
#[cfg(not(feature="2022_19_trace"))]
impl Log {
    fn empty(_blueprint: & Blueprint) -> Self {
        Self
    }
    fn extended(
        &self,
        _minute: usize,
        _robots: (usize, usize, usize, usize),
        _inventory: (usize, usize, usize, usize),
        _action: Action,
    ) -> Self { Self }
}

/*
    Optimization:
        1- Do not build more robots than the factory can consume.
            The factory can produce a robot each turn, and has a maximum consumption for each
            resources. Building more robots - except geode ones - cannot increase the number of
            geodes at the end
        2- Keep track of the best geode production, and prune the branch if it cannot keep up
            A rapid maximum estimate, that assumes we are gonna start producing geode robots once for 
            turn, give us the maximum we can hope to obtain from this branch. If it's still under the current
            maximum, aboandon the branch
        3- Try to build the best robots first
            Optimization (2) is heavily favoured if the best branches comes first. Usually if we can
            build a better robot, we should in the near future
        4- If mining resources are sufficient to build a geode bot at each turn, calculate the result of doing that and return it
            When we can build a geode-cracking at each turn, there is no use to build anything else. We just project that production and use it.
        5- Do not build robots on the last turn - they won't be of use
            On the last turn no robot will see any use. We avoid branching leaves thanks to that
        6- Calculate the result of waiting for each bot, skipping waiting <- this go from 1m to 100 ms
            While waiting to build a better bot is good, waiting to build the same bot is not. At each branch 
            we find what robot we can (and should (1)) build next, and wait for it in a single step. 
            If we cannot build any robot in the remaining time the end result can be calculated and returned.
*/

fn max_geodes(minutes: usize, blueprint: &Blueprint) -> (usize, Log) {
    if minutes == 0 {
        return (0, Log::empty(blueprint)); // no time to do anything
    }

    // calculating maximum number of bots (1)
    let max_ore_bots = blueprint
        .ore_robot_cost_ore
        .max(blueprint.clay_robot_cost_ore)
        .max(blueprint.obsidian_robot_cost_ore)
        .max(blueprint.geode_robot_cost_ore);
    let max_clay_bots = blueprint.obsidian_robot_cost_clay;
    let max_obsidian_bots = blueprint.geode_robot_cost_obsidian;

    let mut branches: Vec<(
        usize,
        (usize, usize, usize, usize),
        (usize, usize, usize, usize),
        Log,
    )> = vec![(minutes, (1, 0, 0, 0), (0, 0, 0, 0), Log::empty(blueprint))];
    let mut best_result = 0;
    let mut best_trace = Log::empty(blueprint);
    while let Some((
        minutes_left,
        (ore_bots, clay_bots, obsidian_bots, geode_bots),
        (ore, clay, obsidian, geode),
        trace,
    )) = branches.pop()
    {
        if minutes_left == 0 {
            if best_result < geode {
                best_result = geode;
                best_trace = trace;
            }
            continue;
        }
        if minutes_left == 1 {
            // do not build anything the last turn (5)
            if best_result < geode + geode_bots {
                best_result = geode + geode_bots;
                best_trace = trace.extended(
                    minutes,
                    (ore_bots, clay_bots, obsidian_bots, geode_bots),
                    (ore, clay, obsidian, geode),
                    Action::Wait(1),
                );
            }
            continue;
        }
        if geode + geode_bots*minutes_left + minutes_left * (minutes_left - 1) / 2 < best_result {
            // abort branch, it cannot make to the top (2)
            continue;
        }
        if ore_bots >= blueprint.geode_robot_cost_ore &&
            obsidian_bots>=blueprint.geode_robot_cost_obsidian {
            // from now on, we can build geode bots at the maximum speed (4)
            let geodes_at_the_end = geode + geode_bots*minutes_left + if ore >=blueprint.geode_robot_cost_ore && obsidian >= blueprint.geode_robot_cost_obsidian {minutes_left * (minutes_left - 1) / 2} else {(minutes_left-1) * (minutes_left - 2) / 2};
            if geodes_at_the_end > best_result {
                best_result = geodes_at_the_end;
                best_trace = trace.extended(
                    minutes - minutes_left+1,
                    (ore_bots, clay_bots, obsidian_bots, geode_bots),
                    (ore, clay, obsidian, geode),
                    Action::BuildGeodeBotsFor(minutes_left),
                );
            }
            continue;
        }

        let mut did_we_branch = false;

        // can we build a geode bot in the time we have left? (6)
        if ore + ore_bots * (minutes_left - 2) >= blueprint.geode_robot_cost_ore
            && obsidian + obsidian_bots * (minutes_left - 2) >= blueprint.geode_robot_cost_obsidian
        {
            let time_to_wait = 1 + usize::max(
                blueprint
                    .geode_robot_cost_ore
                    .saturating_sub(ore)
                    .div_ceil(ore_bots),
                blueprint
                    .geode_robot_cost_obsidian
                    .saturating_sub(obsidian)
                    .div_ceil(obsidian_bots),
            );
            debug_assert!(time_to_wait < minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots, clay_bots, obsidian_bots, geode_bots + 1),
                (
                    ore + ore_bots * time_to_wait - blueprint.geode_robot_cost_ore,
                    clay + clay_bots * time_to_wait,
                    obsidian + obsidian_bots * time_to_wait - blueprint.geode_robot_cost_obsidian,
                    geode + geode_bots * time_to_wait,
                ),
                trace.extended(minutes - minutes_left+1, (ore_bots, clay_bots, obsidian_bots, geode_bots), (ore, clay, obsidian, geode), Action::Wait(time_to_wait-1)).extended(
                    minutes - minutes_left + time_to_wait,
                    (ore_bots, clay_bots, obsidian_bots, geode_bots),
                    (
                        ore + ore_bots * (time_to_wait - 1),
                        clay + clay_bots * (time_to_wait - 1),
                        obsidian + obsidian_bots * (time_to_wait - 1),
                        geode + geode_bots * (time_to_wait - 1),
                    ),
                    Action::BuildGeodeBot,
                ),
            ));

            did_we_branch = true;
        }

        // can/should we build a obsidian bot in the time we have left? (6-1)
        if obsidian_bots < max_obsidian_bots
            && ore + ore_bots * (minutes_left - 2) >= blueprint.obsidian_robot_cost_ore
            && clay + clay_bots * (minutes_left - 2) >= blueprint.obsidian_robot_cost_clay
        {
            let time_to_wait = 1 + usize::max(
                blueprint
                    .obsidian_robot_cost_ore
                    .saturating_sub(ore)
                    .div_ceil(ore_bots),
                blueprint
                    .obsidian_robot_cost_clay
                    .saturating_sub(clay)
                    .div_ceil(clay_bots),
            );
            debug_assert!(time_to_wait < minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots, clay_bots, obsidian_bots + 1, geode_bots),
                (
                    ore + ore_bots * time_to_wait - blueprint.obsidian_robot_cost_ore,
                    clay + clay_bots * time_to_wait - blueprint.obsidian_robot_cost_clay,
                    obsidian + obsidian_bots * time_to_wait,
                    geode + geode_bots * time_to_wait,
                ),
                trace.extended(minutes - minutes_left+1, (ore_bots, clay_bots, obsidian_bots, geode_bots), (ore, clay, obsidian, geode), Action::Wait(time_to_wait-1)).extended(
                    minutes - minutes_left + time_to_wait,
                    (ore_bots, clay_bots, obsidian_bots, geode_bots),
                    (
                        ore + ore_bots * (time_to_wait - 1),
                        clay + clay_bots * (time_to_wait - 1),
                        obsidian + obsidian_bots * (time_to_wait - 1),
                        geode + geode_bots * (time_to_wait - 1),
                    ),
                    Action::BuildObsidianBot,
                ),
            ));

            did_we_branch = true;
        }

        // can/should we build a clay bot in the time we have left? (6-1)
        if clay_bots < max_clay_bots
            && ore + ore_bots * (minutes_left - 2) >= blueprint.clay_robot_cost_ore
        {
            let time_to_wait = 1 + blueprint
                .clay_robot_cost_ore
                .saturating_sub(ore)
                .div_ceil(ore_bots);
            debug_assert!(time_to_wait < minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots, clay_bots + 1, obsidian_bots, geode_bots),
                (
                    ore + ore_bots * time_to_wait - blueprint.clay_robot_cost_ore,
                    clay + clay_bots * time_to_wait,
                    obsidian + obsidian_bots * time_to_wait,
                    geode + geode_bots * time_to_wait,
                ),
                trace.extended(minutes - minutes_left+1, (ore_bots, clay_bots, obsidian_bots, geode_bots), (ore, clay, obsidian, geode), Action::Wait(time_to_wait-1)).extended(
                    minutes - minutes_left + time_to_wait,
                    (ore_bots, clay_bots, obsidian_bots, geode_bots),
                    (
                        ore + ore_bots * (time_to_wait - 1),
                        clay + clay_bots * (time_to_wait - 1),
                        obsidian + obsidian_bots * (time_to_wait - 1),
                        geode + geode_bots * (time_to_wait - 1),
                    ),
                    Action::BuildClayBot,
                ),
            ));

            did_we_branch = true;
        }

        // can/should we build a ore bot in the time we have left? (6-1)
        if ore_bots < max_ore_bots
            && ore + ore_bots * (minutes_left - 2) >= blueprint.ore_robot_cost_ore
        {
            let time_to_wait = 1 + blueprint
                .ore_robot_cost_ore
                .saturating_sub(ore)
                .div_ceil(ore_bots);
            debug_assert!(time_to_wait < minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots + 1, clay_bots, obsidian_bots, geode_bots),
                (
                    ore + ore_bots * time_to_wait - blueprint.ore_robot_cost_ore,
                    clay + clay_bots * time_to_wait,
                    obsidian + obsidian_bots * time_to_wait,
                    geode + geode_bots * time_to_wait,
                ),
                trace.extended(minutes - minutes_left+1, (ore_bots, clay_bots, obsidian_bots, geode_bots), (ore, clay, obsidian, geode), Action::Wait(time_to_wait-1)).extended(
                    minutes - minutes_left + time_to_wait,
                    (ore_bots, clay_bots, obsidian_bots, geode_bots),
                    (
                        ore + ore_bots * (time_to_wait - 1),
                        clay + clay_bots * (time_to_wait - 1),
                        obsidian + obsidian_bots * (time_to_wait - 1),
                        geode + geode_bots * (time_to_wait - 1),
                    ),
                    Action::BuildOreBot,
                ),
            ));

            did_we_branch = true;
        }

        // if we did not branch, it means we have to wait the end without building
        if !did_we_branch {
            if geode + geode_bots * minutes_left > best_result {
                best_result = geode + geode_bots * minutes_left;
                best_trace = trace.extended(minutes - minutes_left+1, (ore_bots, clay_bots, obsidian_bots, geode_bots), (ore, clay, obsidian, geode), Action::Wait(minutes_left));
            }
        }
    }
    (best_result, best_trace)
}

pub fn part1(input: &str) -> usize {
    let blueprints = parse_input(input);
    let blueprints_geodes = blueprints.iter().enumerate().map(|(_i, blueprint)| {
        let (geodes, _trace) = max_geodes(24, blueprint);
        #[cfg(feature="_19_trace")]{
            println!("==== Blueprint {} ====", _i + 1);
            println!();
            println!("Produced {} geodes.", geodes);
            println!();
            println!("=== Trace ===");
            println!("{_trace}");
        }
        geodes
    });
    let quality_levels = blueprints_geodes
        .enumerate()
        .map(|(i, geodes)| (i + 1) * geodes);
quality_levels.sum()
}


pub fn part2(input: &str) -> usize {
    let all_blueprints = parse_input(input);
    let blueprints = all_blueprints[..3].iter();
    let blueprints_geodes = blueprints.enumerate().map(|(_i, blueprint)| {
        let (geodes, _trace) = max_geodes(32, blueprint);
        #[cfg(feature="_19_trace")]{
            println!("==== Blueprint {} ====", _i + 1);
            println!();
            println!("Produced {} geodes.", geodes);
            println!();
            println!("=== Trace ===");
            println!("{_trace}");
        }
        geodes
    });
blueprints_geodes.fold(1, |acc, v| acc * v)
}

