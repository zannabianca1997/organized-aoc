#![feature(slice_group_by)]
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_input<'a>(input: &'a str) -> (usize, Vec<usize>, Vec<Vec<usize>>) {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(
            r"(?m)^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (..(?:, ..)*)$"
        )
        .unwrap();
    }

    // function to map names uniquely
    let mut name_id = {
        let mut name_map = HashMap::new();
        let mut idx_count = 1;
        move |name: &'a str| -> usize {
            if let Some(id) = name_map.get(name) {
                *id
            } else {
                let id = idx_count;
                idx_count += 1;
                name_map.insert(name, id);
                id
            }
        }
    };

    let mut nodes: Vec<(_, Vec<_>)> = vec![];
    for captures in LINE_RE.captures_iter(input) {
        let id = (&mut name_id)(captures.get(1).unwrap().as_str());
        let node = (
            captures[2].parse().unwrap(),
            captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(&mut name_id)
                .collect(),
        );
        if nodes.len() <= id {
            nodes.resize(id + 1, (0, vec![]))
        }
        nodes[id] = node;
    }

    // now we calculate the distance between each node
    let mut distances: Vec<Vec<_>> = vec![vec![usize::MAX / 2; nodes.len()]; nodes.len()];
    // adding the trivial distances
    for (node, (_, links)) in nodes.iter().enumerate() {
        distances[node][node] = 0;
        for other_node in links {
            distances[node][*other_node] = 1;
        }
    }
    // Floyd–Warshall algorithm
    for k in 0..nodes.len() {
        // todo: halve these cycles
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                let dist_by_k = distances[i][k] + distances[k][j];
                if distances[i][j] > dist_by_k {
                    distances[i][j] = dist_by_k;
                }
            }
        }
    }

    let flows = nodes.iter().map(|(flow, _)| *flow).collect();

    // finding the possible
    (name_id("AA"), flows, distances)
}

#[cfg(feature = "2022_16_bitmaps")]
type Valves = u64;
#[cfg(not(feature = "2022_16_bitmaps"))]
type Valves = Vec<bool>;

#[cfg(feature = "2022_16_bitmaps")]
#[inline]
fn get_valve(v: &Valves, i: usize) -> bool {
    (*v >> i) & 1 != 0
}
#[cfg(not(feature = "2022_16_bitmaps"))]
#[inline]
fn get_valve(v: &Valves, i: usize) -> bool {
    v[i]
}

#[inline]
#[cfg(feature = "2022_16_bitmaps")]
fn open_valve(v: Valves, i: usize) -> Valves {
    v | (1 << i)
}
#[inline]
#[cfg(not(feature = "2022_16_bitmaps"))]
fn open_valve(mut v: Valves, i: usize) -> Valves {
    v[i] = true;
    v
}
#[inline]
#[cfg(feature = "2022_16_bitmaps")]
fn empty_valves(_nodes_num: usize) -> Valves {
    0
}
#[inline]
#[cfg(not(feature = "2022_16_bitmaps"))]
fn empty_valves(nodes_num: usize) -> Valves {
    vec![false; nodes_num]
}
#[inline]
#[cfg(feature = "2022_16_bitmaps")]
fn are_disjointed(valves_1: &Valves, valves_2: &Valves) -> bool {
    (valves_1 & valves_2) == 0
}
#[inline]
#[cfg(not(feature = "2022_16_bitmaps"))]
fn are_disjointed(valves_1: &Valves, valves_2: &Valves) -> bool {
    valves_1
        .iter()
        .zip(valves_2.iter())
        .all(|(v1, v2)| !(*v1 && *v2))
}

fn paths(
    pos: usize,
    minutes: usize,
    flows: &[usize],
    distances: &[&[usize]],
) -> Vec<(Valves, usize)> {
    #[cfg(feature = "2022_16_bitmaps")]
    assert!(
        flows.len() <= Valves::BITS as _,
        "Bitmaps can be used only with less than {} nodes",
        Valves::BITS
    );

    let mut paths: Vec<(usize, usize, (Valves, usize))> =
        vec![(pos, minutes, (empty_valves(flows.len()), 0))];
    let mut complete_paths = vec![];
    while let Some((pos, minutes, (valves, total_flow))) = paths.pop() {
        let mut extended = false;
        // try to extend the path
        for valve in 0..flows.len() {
            if ! get_valve(&valves, valve) // the valve is still unopen
            && flows[valve]>0  // is useful to open it
            && distances[pos][valve]+1 < minutes
            // i can reach and open it in time
            {
                // go to this valve
                let minutes = minutes - distances[pos][valve];
                let pos = valve;
                // open it
                let minutes = minutes - 1;
                let valves = open_valve(valves.clone(), valve);
                let total_flow = total_flow + flows[pos] * minutes;
                // push it to be extended
                paths.push((pos, minutes, (valves, total_flow)));
                extended = true;
            }
        }
        if !extended {
            // path cannot be extended. Adding it to the possible paths
            complete_paths.push((valves, total_flow))
        }
    }
    complete_paths
}

pub fn part1(input: &str) -> usize {
    let (pos, flows, distances) = parse_input(input);

    let flows: Vec<(Valves, usize)> = paths(
        pos,
        30,
        &flows,
        &distances.iter().map(|l| l.as_slice()).collect::<Vec<_>>(),
    );

    // dbg!(flows.len());

    // find the best flow
    flows.iter().map(|(_, flow)| *flow).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let (pos, flows, distances) = parse_input(input);

    let mut flows: Vec<(Valves, usize)> = paths(
        pos,
        26,
        &flows,
        &distances.iter().map(|l| l.as_slice()).collect::<Vec<_>>(),
    );

    // dbg!(flows.len());

    // erasing duplicates keeping only the max for each valve configuration
    flows.sort_unstable_by(|(vs1, _), (vs2, _)| vs1.cmp(vs2));
    let flows: Vec<&(Valves, usize)> = flows
        .group_by(|(vs1, _), (vs2, _)| vs1 == vs2)
        .map(|run| run.iter().max_by_key(|(_, flow)| flow).unwrap())
        .collect();

    // dbg!(flows.len());

    let mut max_flow = 0;
    for (i, (valves_1, flow_1)) in flows.iter().enumerate() {
        for (valves_2, flow_2) in flows[i + 1..].iter() {
            // check they do not intersect
            if are_disjointed(valves_1, valves_2) {
                max_flow = max_flow.max(flow_1 + flow_2)
            }
        }
    }

    // find the best flow
    max_flow
}
