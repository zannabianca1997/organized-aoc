#![feature(iter_next_chunk)]
#![feature(array_windows)]
#![feature(array_methods)]

use std::mem;

pub fn parse(input: &str) -> impl Iterator<Item = ([isize; 4], usize)> + '_ {
    input.trim().lines().map(|l| {
        // Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8
        let [capacity, durability, flavour, texture, calories] = l
            .split_once(':')
            .unwrap()
            .1
            .splitn(5, ',')
            .map(|p| p.trim().split_once(' ').unwrap().1)
            .next_chunk()
            .unwrap();
        (
            [capacity, durability, flavour, texture].map(|n| n.parse().unwrap()),
            calories.parse().unwrap(),
        )
    })
}

fn best_properties(
    remaining_ingredients: &[[isize; 4]],
    budget: usize,
    totals: [isize; 4],
) -> usize {
    if budget == 0 {
        // budget is completly used, no more ingredients can be added
        return totals
            .into_iter()
            .map(|f| f.try_into().unwrap_or(0))
            .product();
    }

    let (ingredient, remaining_ingredients) = remaining_ingredients.split_first().unwrap();
    if remaining_ingredients.is_empty() {
        // last ingredient must use the whole budget
        let mut new_totals = totals;
        for idx in 0..4 {
            new_totals[idx] += ingredient[idx] * budget as isize;
        }
        return best_properties(&[], 0, new_totals);
    }

    // recurse
    let mut max = usize::MIN;
    for i in 0..=budget {
        let mut new_totals = totals;
        for idx in 0..4 {
            new_totals[idx] += ingredient[idx] * i as isize;
        }
        max = max.max(best_properties(
            remaining_ingredients,
            budget - i,
            new_totals,
        ))
    }

    max
}

fn specialized_part_1([pi, pj, pk, pl]: [[isize; 4]; 4]) -> usize {
    let mut max = usize::MIN;
    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let l = 100 - i - j - k;
                max = max.max(
                    usize::try_from(pi[0] * i + pj[0] * j + pk[0] * k + pl[0] * l).unwrap_or(0)
                        * usize::try_from(pi[1] * i + pj[1] * j + pk[1] * k + pl[1] * l)
                            .unwrap_or(0)
                        * usize::try_from(pi[2] * i + pj[2] * j + pk[2] * k + pl[2] * l)
                            .unwrap_or(0)
                        * usize::try_from(pi[3] * i + pj[3] * j + pk[3] * k + pl[3] * l)
                            .unwrap_or(0),
                )
            }
        }
    }
    max
}

pub fn part1(input: &str) -> usize {
    let ingredients: Vec<_> = parse(input).map(|(p, _)| p).collect();
    if let Ok(ingredients) = <&[_; 4]>::try_from(ingredients.as_slice()) {
        specialized_part_1(*ingredients)
    } else {
        best_properties(&ingredients, 100, [0, 0, 0, 0])
    }
}

fn best_properties_2(
    remaining_ingredients: &[([isize; 4], usize)],
    budget: usize,
    caloric_budget: usize,
    totals: [isize; 4],
) -> usize {
    if (budget == 0) != (caloric_budget == 0) {
        // only one budget is used up, this branch is dead
        // no ingredients can be added, fulfilling the requirements is impossible
        return usize::MIN;
    }
    if budget == 0 && caloric_budget == 0 {
        // both budgets are completly used, no more ingredients can be added
        return totals
            .into_iter()
            .map(|f| f.try_into().unwrap_or(0))
            .product();
    }

    let ((properties, calorie), remaining_ingredients) =
        remaining_ingredients.split_first().unwrap();
    if remaining_ingredients.is_empty() {
        // last ingredient must use both budgets
        if calorie * budget != caloric_budget {
            // impossible to empty both budgets
            return usize::MIN;
        }
        let mut new_totals = totals;
        for idx in 0..4 {
            new_totals[idx] += properties[idx] * budget as isize;
        }
        return best_properties_2(&[], 0, 0, new_totals);
    }

    // recurse
    let mut max = usize::MIN;
    for i in 0..=usize::min(budget, caloric_budget / calorie) {
        let mut new_totals = totals;
        for idx in 0..4 {
            new_totals[idx] += properties[idx] * i as isize;
        }
        max = max.max(best_properties_2(
            remaining_ingredients,
            budget - i,
            caloric_budget - i * calorie,
            new_totals,
        ))
    }

    max
}

pub fn part2(input: &str) -> usize {
    let ingredients: Vec<_> = parse(input).collect();
    if let Ok(ingredients) = <&[_; 4]>::try_from(ingredients.as_slice()) {
        let [mut a, mut b, mut c, mut d] = *ingredients;
        if c.1 == d.1 {
            if b.1 != d.1 {
                mem::swap(&mut b, &mut c)
            } else if a.1 != d.1 {
                mem::swap(&mut a, &mut c)
            }
        }
        if c.1 != d.1 {
            if c.1 < d.1 {
                mem::swap(&mut c, &mut d)
            }
            specialized_part_2_dishomogeneus([a, b, c, d])
        } else {
            // all ingredients have the same calories
            assert_eq!(a.1, 5, "Impossible input: all ingredients have the same calories, different from 5==500/100");
            // is now equal to the part 1
            specialized_part_1([a.0, b.0, c.0, d.0])
        }
    } else {
        best_properties_2(&ingredients, 100, 500, [0, 0, 0, 0])
    }
}

/// Specialized function for the case n=4
/// this algorithm skip a cycle, but requires ck > cl
fn specialized_part_2_dishomogeneus(
    [(pi, ci), (pj, cj), (pk, ck), (pl, cl)]: [([isize; 4], usize); 4],
) -> usize {
    let [ci, cj, ck, cl] = [ci as isize, cj as isize, ck as isize, cl as isize];
    let mut max = usize::MIN;
    for i in 0..=isize::min(100, 500 / ci) {
        for j in 0..=isize::min(100 - i, (500 - i * ci) / cj) {
            let r = 500 - i * ci - j * cj - (100 - i - j) * cl;
            if r >= 0 && r % (ck - cl) == 0 {
                let k = r / (ck - cl);
                let l = 100 - i - j - k;
                /*
                   i * ci + j * cj + k * ck + l * cl == 500
                   i * ci + j * cj + k * ck + (100 - i - j - k) * cl == 500
                   i * ci + j * cj + k * (ck-cl) + (100 - i - j) * cl == 500
                   i * ci + j * cj + r + (100 - i - j) * cl == 500
                   i * ci + j * cj + 500 - i * ci - j * cj - (100 - i - j) * cl + (100 - i - j) * cl == 500
                */
                max = max.max(
                    usize::try_from(pi[0] * i + pj[0] * j + pk[0] * k + pl[0] * l).unwrap_or(0)
                        * usize::try_from(pi[1] * i + pj[1] * j + pk[1] * k + pl[1] * l)
                            .unwrap_or(0)
                        * usize::try_from(pi[2] * i + pj[2] * j + pk[2] * k + pl[2] * l)
                            .unwrap_or(0)
                        * usize::try_from(pi[3] * i + pj[3] * j + pk[3] * k + pl[3] * l)
                            .unwrap_or(0),
                )
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn parse() {
        let table: Vec<_> = super::parse(EXAMPLE).collect();
        assert_eq!(table, vec![([-1, -2, 6, 3], 8), ([2, 3, -2, -1], 3)])
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(EXAMPLE), 62842880)
    }
}
