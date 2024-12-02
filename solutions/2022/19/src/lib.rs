use lazy_static::lazy_static;
use regex::Regex;

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

fn max_geodes(minutes: usize, blueprint: &Blueprint) -> usize {
    if minutes == 0 {
        return 0; // no time to do anything
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
    )> = vec![(minutes, (1, 0, 0, 0), (0, 0, 0, 0))];
    let mut best_result = 0;
    while let Some((
        minutes_left,
        (ore_bots, clay_bots, obsidian_bots, geode_bots),
        (ore, clay, obsidian, geode),
    )) = branches.pop()
    {
        if minutes_left == 0 {
            if best_result < geode {
                best_result = geode;
            }
            continue;
        }
        if minutes_left == 1 {
            // do not build anything the last turn (5)
            if best_result < geode + geode_bots {
                best_result = geode + geode_bots;
            }
            continue;
        }
        if geode + geode_bots * minutes_left + minutes_left * (minutes_left - 1) / 2 < best_result {
            // abort branch, it cannot make to the top (2)
            continue;
        }
        if ore_bots >= blueprint.geode_robot_cost_ore
            && obsidian_bots >= blueprint.geode_robot_cost_obsidian
        {
            // from now on, we can build geode bots at the maximum speed (4)
            let geodes_at_the_end = geode
                + geode_bots * minutes_left
                + if ore >= blueprint.geode_robot_cost_ore
                    && obsidian >= blueprint.geode_robot_cost_obsidian
                {
                    minutes_left * (minutes_left - 1) / 2
                } else {
                    (minutes_left - 1) * (minutes_left - 2) / 2
                };
            if geodes_at_the_end > best_result {
                best_result = geodes_at_the_end;
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
            debug_assert!(time_to_wait <= minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots, clay_bots, obsidian_bots, geode_bots + 1),
                (
                    ore + ore_bots * time_to_wait - blueprint.geode_robot_cost_ore,
                    clay + clay_bots * time_to_wait,
                    obsidian + obsidian_bots * time_to_wait - blueprint.geode_robot_cost_obsidian,
                    geode + geode_bots * time_to_wait,
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
            debug_assert!(time_to_wait <= minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots, clay_bots, obsidian_bots + 1, geode_bots),
                (
                    ore + ore_bots * time_to_wait - blueprint.obsidian_robot_cost_ore,
                    clay + clay_bots * time_to_wait - blueprint.obsidian_robot_cost_clay,
                    obsidian + obsidian_bots * time_to_wait,
                    geode + geode_bots * time_to_wait,
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
            debug_assert!(time_to_wait <= minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots, clay_bots + 1, obsidian_bots, geode_bots),
                (
                    ore + ore_bots * time_to_wait - blueprint.clay_robot_cost_ore,
                    clay + clay_bots * time_to_wait,
                    obsidian + obsidian_bots * time_to_wait,
                    geode + geode_bots * time_to_wait,
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
            debug_assert!(time_to_wait <= minutes_left - 1);

            branches.push((
                minutes_left - time_to_wait,
                (ore_bots + 1, clay_bots, obsidian_bots, geode_bots),
                (
                    ore + ore_bots * time_to_wait - blueprint.ore_robot_cost_ore,
                    clay + clay_bots * time_to_wait,
                    obsidian + obsidian_bots * time_to_wait,
                    geode + geode_bots * time_to_wait,
                ),
            ));

            did_we_branch = true;
        }

        // if we did not branch, it means we have to wait the end without building
        if !did_we_branch {
            if geode + geode_bots * minutes_left > best_result {
                best_result = geode + geode_bots * minutes_left;
            }
        }
    }
    best_result
}

pub fn part1(input: &str) -> usize {
    let blueprints = parse_input(input);
    let blueprints_geodes = blueprints.iter().enumerate().map(|(_i, blueprint)| {
        let geodes = max_geodes(24, blueprint);
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
        let geodes = max_geodes(32, blueprint);
        geodes
    });
    blueprints_geodes.fold(1, |acc, v| acc * v)
}
