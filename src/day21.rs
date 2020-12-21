use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let mut buffer = String::new();
    let input = parse(input, &mut buffer);

    match part {
        1 => println!("{}", part_1(input)),
        2 => println!("{}", part_2(input)),
        _ => unimplemented!(),
    };

    Ok(())
}

type Input<'a> = Vec<(HashSet<&'a str>, Vec<&'a str>)>;

fn parse(mut input: impl BufRead, mut buffer: &mut String) -> Input {
    input.read_to_string(&mut buffer).unwrap();

    buffer
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
            let ingredients = ingredients.split(' ').collect();
            let allergens = allergens.strip_suffix(')').unwrap().split(", ").collect();
            (ingredients, allergens)
        })
        .collect()
}

fn match_ingredients(input: Input) -> (HashMap<&str, usize>, HashSet<&str>, HashMap<&str, &str>) {
    let mut candidates = HashMap::new();
    let mut all_ingredients = HashMap::new();
    let mut solved_ingredient = HashSet::new();
    let mut solved_allergen = HashMap::new();

    for (ingredients, allergens) in input.iter() {
        for allergen in allergens {
            let entry = candidates
                .entry(*allergen)
                .or_insert_with(|| ingredients.clone());
            *entry = entry.intersection(ingredients).copied().collect();
        }

        for ingredient in ingredients {
            let entry = all_ingredients.entry(*ingredient).or_insert(0);
            *entry += 1;
        }
    }

    while solved_allergen.len() < candidates.len() {
        for (&allergen, ingredients) in candidates.iter_mut() {
            if solved_allergen.get(allergen).is_none() {
                if ingredients.len() == 1 {
                    let ingredient = ingredients.iter().next().unwrap();
                    solved_ingredient.insert(*ingredient);
                    solved_allergen.insert(allergen, *ingredient);
                } else {
                    for ingredient in ingredients.clone().iter() {
                        if solved_ingredient.contains(ingredient) {
                            ingredients.remove(ingredient);
                        }
                    }
                }
            }
        }
    }

    (all_ingredients, solved_ingredient, solved_allergen)
}

fn part_1(input: Input) -> usize {
    let (all_ingredients, solved_ingredient, _) = match_ingredients(input);

    all_ingredients
        .iter()
        .filter(|(ingredient, _)| !solved_ingredient.contains(**ingredient))
        .map(|(_, count)| count)
        .sum()
}

fn part_2(input: Input) -> String {
    let (_, _, solved_allergens) = match_ingredients(input);

    let mut solved_allergens: Vec<(&str, &str)> = solved_allergens.into_iter().collect();

    solved_allergens.sort_unstable_by_key(|(allergen, _)| *allergen);

    let last_index = solved_allergens.len() - 1;
    let mut dangerous = String::new();

    for (i, (_, ingredient)) in solved_allergens.into_iter().enumerate() {
        dangerous.push_str(ingredient);

        if i < last_index {
            dangerous.push(',');
        }
    }

    dangerous
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn part_1_example() {
        let mut buffer = String::new();
        let input = parse(io::Cursor::new(EXAMPLE), &mut buffer);
        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn part_2_example() {
        let mut buffer = String::new();
        let input = parse(io::Cursor::new(EXAMPLE), &mut buffer);
        assert_eq!(part_2(input), "mxmxvkd,sqjhc,fvjkl");
    }
}
