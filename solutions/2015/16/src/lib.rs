#[derive(Debug, Default, Clone, Copy)]
struct Info {
    num: usize,

    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}
impl Info {
    fn prop(&mut self, prop: &str) -> &mut Option<usize> {
        match prop {
            "children" => &mut self.children,
            "cats" => &mut self.cats,
            "samoyeds" => &mut self.samoyeds,
            "pomeranians" => &mut self.pomeranians,
            "akitas" => &mut self.akitas,
            "vizslas" => &mut self.vizslas,
            "goldfish" => &mut self.goldfish,
            "trees" => &mut self.trees,
            "cars" => &mut self.cars,
            "perfumes" => &mut self.perfumes,
            _ => panic!("unknow property: {prop}"),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Info> + '_ {
    input.trim().lines().map(|l| {
        //Sue 1: goldfish: 9, cars: 0, samoyeds: 9
        let (name, list) = l.split_once(':').unwrap();
        let mut info = Info {
            num: name.trim().split_once(' ').unwrap().1.parse().unwrap(),
            ..Default::default()
        };
        for (prop, num) in list.split(',').map(|i| {
            let (p, n) = i.split_once(':').unwrap();
            (p.trim(), n.trim().parse().unwrap())
        }) {
            *info.prop(prop) = Some(num)
        }
        info
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .find(
            |Info {
                 num: _,
                 children,
                 cats,
                 samoyeds,
                 pomeranians,
                 akitas,
                 vizslas,
                 goldfish,
                 trees,
                 cars,
                 perfumes,
             }| {
                /*
                    children: 3
                    cats: 7
                    samoyeds: 2
                    pomeranians: 3
                    akitas: 0
                    vizslas: 0
                    goldfish: 5
                    trees: 3
                    cars: 2
                    perfumes: 1
                */
                !(children.is_some_and(|c| c != 3)
                    || cats.is_some_and(|c| c != 7)
                    || samoyeds.is_some_and(|s| s != 2)
                    || pomeranians.is_some_and(|p| p != 3)
                    || akitas.is_some_and(|a| a != 0)
                    || vizslas.is_some_and(|v| v != 0)
                    || goldfish.is_some_and(|g| g != 5)
                    || trees.is_some_and(|t| t != 3)
                    || cars.is_some_and(|c| c != 2)
                    || perfumes.is_some_and(|p| p != 1))
            },
        )
        .unwrap()
        .num
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .find(
            |Info {
                 num: _,
                 children,
                 cats,
                 samoyeds,
                 pomeranians,
                 akitas,
                 vizslas,
                 goldfish,
                 trees,
                 cars,
                 perfumes,
             }| {
                /*
                    children: 3
                    cats: 7
                    samoyeds: 2
                    pomeranians: 3
                    akitas: 0
                    vizslas: 0
                    goldfish: 5
                    trees: 3
                    cars: 2
                    perfumes: 1
                */
                !(children.is_some_and(|c| c != 3)
                    || cats.is_some_and(|c| c <= 7)
                    || samoyeds.is_some_and(|s| s != 2)
                    || pomeranians.is_some_and(|p| p >= 3)
                    || akitas.is_some_and(|a| a != 0)
                    || vizslas.is_some_and(|v| v != 0)
                    || goldfish.is_some_and(|g| g >= 5)
                    || trees.is_some_and(|t| t <= 3)
                    || cars.is_some_and(|c| c != 2)
                    || perfumes.is_some_and(|p| p != 1))
            },
        )
        .unwrap()
        .num
}
