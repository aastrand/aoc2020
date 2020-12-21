use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::{HashMap, HashSet};

/*
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)

dairy1: mxmxvkd kfcds sqjhc nhms
dairy2: trh fvjkl sbzzf mxmxvkd

dairy1 intersect dairy2 = mxmxvkd => mxmxvkd

fish1: mxmxvkd kfcds sqjhc nhms
fish2: sqjhc mxmxvkd sbzzf

fish1 intersect fish2 = mxmxvkd, sqjhc => sqjhc

soy1: sqjhc fvjkl => fvjkl


In the above example, none of the ingredients kfcds, nhms, sbzzf, or trh

*/

fn solve1(filename: &str) -> u64 {
    let mut allergen_to_ingredients: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
    let mut all_ingredients: HashSet<String> = HashSet::new();
    let mut ingredients_as_listed = vec![];

    for line in lines_from_file(filename) {
        let splits: Vec<&str> = line.split(" (").collect();
        let ingredients = splits[0]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();
        ingredients_as_listed.push(ingredients.clone());

        let allergens: Vec<&str> = splits[1][9..splits[1].len() - 1].split(" ").collect();
        all_ingredients = all_ingredients
            .union(&ingredients)
            .map(|i| i.to_string())
            .collect::<HashSet<String>>();

        for mut allergen in allergens {
            if allergen.chars().last().unwrap() == ',' {
                allergen = &allergen[0..allergen.len() - 1];
            }
            if !allergen_to_ingredients.contains_key(allergen) {
                allergen_to_ingredients.insert(allergen.to_string(), vec![]);
            }
            let list = allergen_to_ingredients.get_mut(allergen).unwrap();
            list.push(ingredients.clone());
        }
    }

    let mut reduced: HashMap<String, HashSet<String>> = HashMap::new();
    for allergen in allergen_to_ingredients.keys() {
        let sets = allergen_to_ingredients.get(allergen).unwrap();
        let mut reduction = sets[0].clone();
        for i in 1..sets.len() {
            reduction = reduction
                .intersection(&sets[i])
                .map(|i| i.to_string())
                .collect::<HashSet<String>>();
        }
        reduced.insert(allergen.to_string(), reduction);
    }

    let mut visited = HashSet::new();
    let keys = reduced
        .keys()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .clone();

    loop {
        let mut single = String::new();

        let mut work_left = false;
        for key in &keys {
            let list = reduced.get(key).unwrap();
            if list.len() == 1 {
                let candidate = list.iter().next().unwrap().to_string();
                if !visited.contains(&candidate) {
                    visited.insert(candidate.clone());
                    single = candidate;
                    work_left = true;
                    break;
                }
            }
        }

        if !work_left {
            break;
        }

        for key in &keys {
            let list = reduced.get_mut(key).unwrap();
            if list.len() > 1 {
                list.remove(&single);
            }
        }
    }
    println!("{:?}", reduced);
    let ingredients_with_allergens = reduced
        .values()
        .map(|s| s.iter().next().unwrap())
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();
    let ingredients_with_no_allergens = all_ingredients
        .difference(&ingredients_with_allergens)
        .map(|i| i.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0;
    for list in ingredients_as_listed {
        for ingredient in &ingredients_with_no_allergens {
            if list.contains(ingredient) {
                sum += 1;
            }
        }
    }

    let mut allergens = reduced
        .keys()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    allergens.sort();

    println!("{:?}", allergens
        .iter()
        .map(|a| reduced.get(a).unwrap().iter().next().unwrap())
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join(","));
    

    sum
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    println!("{}", solve1("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 5);
        //assert_eq!(solve2("example.txt"), "mxmxvkd,sqjhc,fvjkl");
    }
}
