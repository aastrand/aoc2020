use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Tile {
    id: String,
    edges: HashSet<String>,
    sides: HashMap<String, Side>,
}

impl Tile {
    fn new(input: &str) -> Tile {
        let mut edges = HashSet::new();
        let mut sides = HashMap::new();
        let split = input.split("\n").collect::<Vec<&str>>();
        let tile_id_re: Regex = Regex::new(r"^Tile ([0-9]+):$").unwrap();

        let id = if let Some(cap) = tile_id_re.captures(split[0]) {
            cap[1].to_string()
        } else {
            panic!("Could not parse id: {}", split[0]);
        };

        // top
        edges.insert(split[1].to_string());
        edges.insert(split[1].chars().rev().collect::<String>());
        sides.insert(split[1].to_string(), Side::Top);
        sides.insert(split[1].chars().rev().collect::<String>(), Side::Top);

        // bottom
        edges.insert(split[10].to_string());
        edges.insert(split[10].chars().rev().collect::<String>());
        sides.insert(split[10].to_string(), Side::Bottom);
        sides.insert(split[10].chars().rev().collect::<String>(), Side::Bottom);

        // left and right edgs
        let mut left: Vec<char> = vec![];
        let mut right: Vec<char> = vec![];
        for i in 1..split.len() {
            let chars: Vec<char> = split[i].chars().collect();
            left.push(chars[0]);
            right.push(chars[9]);
        }

        edges.insert(left.iter().collect::<String>());
        edges.insert(left.iter().rev().collect::<String>());
        sides.insert(left.iter().collect::<String>(), Side::Left);
        sides.insert(left.iter().rev().collect::<String>(), Side::Left);

        edges.insert(right.iter().collect::<String>());
        edges.insert(right.iter().rev().collect::<String>());
        sides.insert(right.iter().collect::<String>(), Side::Right);
        sides.insert(right.iter().rev().collect::<String>(), Side::Right);

        Tile {
            id: id,
            edges: edges,
            sides: sides,
        }
    }
}

fn solve1(filename: &str) -> u64 {
    let tiles = std::fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|t| Tile::new(t))
        .collect::<Vec<Tile>>();

    let mut corners = vec![];
    for t1 in &tiles {
        let mut matches = 0;
        for t2 in &tiles {
            if t1.id != t2.id {
                matches += t1
                    .edges
                    .intersection(&t2.edges)
                    .collect::<Vec<&String>>()
                    .len();
            }
        }
        if matches == 4 {
            corners.push(t1.id.clone());
        }
    }

    corners
        .iter()
        .map(|id| id.parse::<u64>().unwrap())
        .product()
}

/*
r0
1 2 3
4 5 6
7 8 9
r90
7 4 1
8 5 2
9 6 3
r180
9 8 7
6 5 4
3 2 1
r270
3 6 9
2 5 8
1 4 7
r0 flipped x
3 2 1
6 5 4
9 8 7
r90 flipped x
1 4 7
2 5 8
3 6 9
r180 flipped x
7 8 9
4 5 6
1 2 3
r270 flipped x
9 6 3
8 5 2
7 4 1
r0 flipped y == 180 flipped x
7 8 9
4 5 6
1 2 3
r90 flipped y == 270 flipped x
9 6 3
8 5 2
7 4 1
r180 flipped y == r0 flipped x
3 2 1
6 5 4
9 8 7
r270 flipped y == r90 flipped x
1 4 7
2 5 8
3 6 9
*/

fn rotate(input: &Vec<String>) -> Vec<String> {
    let mut out = vec![];
    for _ in 0..input.len() {
        out.push(vec![]);
    }

    for i in 0..input.len() {
        let chars: Vec<char> = input[i].chars().collect();
        for j in 0..chars.len() {
            out[j].insert(0, chars[j]);
        }
    }

    out.iter().map(|v| v.iter().collect::<String>()).collect()
}

fn flip(input: &Vec<String>) -> Vec<String> {
    input
        .iter()
        .map(|v| v.chars().rev().collect::<String>())
        .collect()
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Id {
    id: String,
    num: String,
}

impl Id {
    fn new(id: String, num: String) -> Id {
        Id { id: id, num: num }
    }
}

#[derive(Debug, Clone)]
struct Tile2 {
    id: Id,
    content: Vec<String>,
    sides: HashMap<String, Side>,
    side_to_content: HashMap<Side, String>,
}

impl Tile2 {
    fn new(id: Id, input: Vec<String>) -> Tile2 {
        let mut sides = HashMap::new();
        let mut side_to_content = HashMap::new();

        // top
        sides.insert(input[0].to_string(), Side::Top);
        side_to_content.insert(Side::Top, input[0].to_string());

        // bottom
        sides.insert(input[9].to_string(), Side::Bottom);
        side_to_content.insert(Side::Bottom, input[9].to_string());

        // left and right edgs
        let mut left: Vec<char> = vec![];
        let mut right: Vec<char> = vec![];
        for i in 0..input.len() {
            let chars: Vec<char> = input[i].chars().collect();
            left.push(chars[0]);
            right.push(chars[9]);
        }

        sides.insert(left.iter().collect::<String>(), Side::Left);
        side_to_content.insert(Side::Left, left.iter().collect::<String>());

        sides.insert(right.iter().collect::<String>(), Side::Right);
        side_to_content.insert(Side::Right, right.iter().collect::<String>());

        Tile2 {
            id: id,
            content: input,
            sides: sides,
            side_to_content: side_to_content,
        }
    }
}

fn insert_tile(
    t: &Tile2,
    tiles: &mut HashMap<Id, Tile2>,
    edge_to_tile_id: &mut HashMap<(Side, String), Vec<Id>>,
) {
    tiles.insert(t.id.clone(), t.clone());
    for content in t.sides.keys() {
        let side = t.sides.get(content).unwrap();
        let key = (side.clone(), content.clone());
        if !edge_to_tile_id.contains_key(&key) {
            edge_to_tile_id.insert(key, vec![]);
        }
        let list = edge_to_tile_id
            .get_mut(&(side.clone(), content.clone()))
            .unwrap();
        list.push(t.id.clone());
    }
}

/*
                  #
#    ##    ##    ###
 #  #  #  #  #  #
*/
const OFFSETS: [usize; 15] = [18, 20, 25, 26, 31, 32, 37, 38, 39, 41, 44, 47, 50, 53, 56];
fn find_seamonsters(sea: &str, width_padding: usize) -> u64 {
    let mut sum = 0;
    let chars = sea.chars().collect::<Vec<char>>();
    let mut offsets = OFFSETS.clone();
    for i in 1..offsets.len() {
        offsets[i] += width_padding;
    }
    for i in 9..offsets.len() {
        offsets[i] += width_padding;
    }

    for i in 0..sea.len() - offsets[14] - 3 {
        let mut found = true;
        for idx in offsets.iter() {
            if chars[i + idx] != '#' {
                found = false;
                break;
            }
        }
        if found {
            sum += 1;
        }
    }
    sum
}

fn solve2(filename: &str) -> u64 {
    // for all tiles, create all 8 versions
    // create map (string, side) -> tile
    // pick starter (any corner)
    // right edge -> tile -> right -> .. border
    // bottom edge -> tile -> right -> .. border
    // border

    // build long string
    // look for offsets for a sea monster?
    // count amount of matches
    // count amount of '#'
    // answer = ^ - amount of matches * amount of '#' in sea monster (15?)

    let mut edge_to_tile_id: HashMap<(Side, String), Vec<Id>> = HashMap::new();
    let mut tiles: HashMap<Id, Tile2> = HashMap::new();
    for tile in std::fs::read_to_string(filename).unwrap().split("\n\n") {
        let split = tile.split("\n").collect::<Vec<&str>>();
        let tile_id_re: Regex = Regex::new(r"^Tile ([0-9]+):$").unwrap();

        let id = if let Some(cap) = tile_id_re.captures(split[0]) {
            cap[1].to_string()
        } else {
            panic!("Could not parse id: {}", split[0]);
        };

        let content: Vec<String> = split[1..split.len()]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let t = Tile2::new(Id::new(format!("{} R0", id), id.clone()), content.clone());
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);
        let flipped_content = flip(&content);
        let t = Tile2::new(
            Id::new(format!("{} R0 FX", id), id.clone()),
            flipped_content.clone(),
        );
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);

        let content = rotate(&content);
        let t = Tile2::new(Id::new(format!("{} R90", id), id.clone()), content.clone());
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);
        let flipped_content = flip(&content);
        let t = Tile2::new(
            Id::new(format!("{} R90 FX", id), id.clone()),
            flipped_content.clone(),
        );
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);

        let content = rotate(&content);
        let t = Tile2::new(Id::new(format!("{} R180", id), id.clone()), content.clone());
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);
        let flipped_content = flip(&content);
        let t = Tile2::new(
            Id::new(format!("{} R180 FX", id), id.clone()),
            flipped_content.clone(),
        );
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);

        let content = rotate(&content);
        let t = Tile2::new(Id::new(format!("{} R270", id), id.clone()), content.clone());
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);
        let flipped_content = flip(&content);
        let t = Tile2::new(
            Id::new(format!("{} R270 FX", id), id.clone()),
            flipped_content.clone(),
        );
        insert_tile(&t, &mut tiles, &mut edge_to_tile_id);
    }

    let mut graph: HashMap<Id, HashMap<Side, Id>> = HashMap::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut stack = vec![tiles.values().next().unwrap()];

    while stack.len() > 0 {
        let tile = stack.pop().unwrap();
        visited.insert(tile.id.num.as_ref());
        let mut node: HashMap<Side, Id> = HashMap::new();
        for (content, side) in &tile.sides {
            let other_side = match side {
                Side::Top => Side::Bottom,
                Side::Bottom => Side::Top,
                Side::Left => Side::Right,
                Side::Right => Side::Left,
            };

            let neighbour = edge_to_tile_id
                .get(&(other_side.clone(), content.to_string()))
                .unwrap()
                .iter()
                .filter(|id| (*id).num != tile.id.num)
                .map(|id| id.clone())
                .collect::<Vec<Id>>();

            if neighbour.len() > 0 {
                node.insert(side.clone(), neighbour[0].clone());
                if !visited.contains(&neighbour[0].num.as_ref()) {
                    stack.push(tiles.get(&neighbour[0]).unwrap());
                }
            }
        }
        graph.insert(tile.id.clone(), node);
    }

    let mut next_row = None;
    for (key, value) in &graph {
        if !value.contains_key(&Side::Top) && !value.contains_key(&Side::Left) {
            next_row = Some(key);
        }
    }
    let mut puzzle: Vec<Vec<Tile2>> = vec![];

    while let Some(topleft) = next_row {
        let mut row = vec![];
        row.push(tiles.get(&topleft).unwrap().clone());
        let mut next = graph.get(&topleft).unwrap().get(&Side::Right);
        while let Some(neightbour) = next {
            row.push(tiles.get(&neightbour).unwrap().clone());
            next = graph.get(&neightbour).unwrap().get(&Side::Right);
        }

        puzzle.push(row);
        next_row = graph.get(topleft).unwrap().get(&Side::Bottom);
    }

    let mut puzzle_content: Vec<String> = vec![];
    let mut full_content: Vec<String> = vec![];

    for y in 0..puzzle.len() {
        let row = &puzzle[y];
        for _ in 0..8 {
            puzzle_content.push(String::new());
        }
        for _ in 0..11 {
            full_content.push(String::new());
        }
        for x in 0..row.len() {
            let col = &row[x];
            print!("{} ", col.id.num);
            for s in 1..col.content.len() - 1 {
                puzzle_content[(y * 8) + s - 1].push_str(&col.content[s][1..9]);
            }
            for s in 0..col.content.len() {
                full_content[(y * 11) + s].push_str(&col.content[s]);
                full_content[(y * 11) + s].push_str(" ");
            }
        }
        println!("");
    }

    let width_padding = (puzzle.len() * 8) - 20;
    let mut monsters = vec![];
    monsters.push(find_seamonsters(&puzzle_content.join(""), width_padding));
    monsters.push(find_seamonsters(
        &flip(&puzzle_content).join(""),
        width_padding,
    ));
    puzzle_content = rotate(&puzzle_content);
    monsters.push(find_seamonsters(&puzzle_content.join(""), width_padding));
    monsters.push(find_seamonsters(
        &flip(&puzzle_content).join(""),
        width_padding,
    ));
    puzzle_content = rotate(&puzzle_content);
    monsters.push(find_seamonsters(&puzzle_content.join(""), width_padding));
    monsters.push(find_seamonsters(
        &flip(&puzzle_content).join(""),
        width_padding,
    ));
    puzzle_content = rotate(&puzzle_content);
    monsters.push(find_seamonsters(&puzzle_content.join(""), width_padding));
    monsters.push(find_seamonsters(
        &flip(&puzzle_content).join(""),
        width_padding,
    ));

    /*for row in full_content {
        println!("{}", row);
    }*/

    puzzle_content
        .join("")
        .chars()
        .filter(|c| *c == '#')
        .collect::<Vec<char>>()
        .len() as u64
        - (monsters.iter().max().unwrap() * 15)
}

fn main() {
    println!("{}", solve1("../input/2020/day20.txt"));
    println!("{}", solve2("../input/2020/day20.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve1("example.txt"), 20899048083289);
        assert_eq!(solve2("example.txt"), 273);
    }

    #[test]
    fn test_rotate() {
        /*
        r0
        1 2 3
        4 5 6
        7 8 9
        r90
        7 4 1
        8 5 2
        9 6 3
        r180
        9 8 7
        6 5 4
        3 2 1
        r270
        3 6 9
        2 5 8
        1 4 7
        */
        let input = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let expected90 = vec!["741".to_string(), "852".to_string(), "963".to_string()];
        assert_eq!(rotate(&input), expected90);

        let expected180 = vec!["987".to_string(), "654".to_string(), "321".to_string()];
        assert_eq!(rotate(&rotate(&input)), expected180);

        let expected270 = vec!["369".to_string(), "258".to_string(), "147".to_string()];
        assert_eq!(rotate(&rotate(&rotate(&input))), expected270);
    }
    #[test]
    fn test_flip() {
        /*
        r0 flipped x
        3 2 1
        6 5 4
        9 8 7
        */
        let input = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let expected = vec!["321".to_string(), "654".to_string(), "987".to_string()];
        assert_eq!(flip(&input), expected);
    }

    #[test]
    fn test_combo() {
        /*
        r90 flipped x
        1 4 7
        2 5 8
        3 6 9
        */
        let input = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let expected90flip = vec!["147".to_string(), "258".to_string(), "369".to_string()];
        assert_eq!(flip(&rotate(&input)), expected90flip);
    }

    #[test]
    fn test_find_seamonsters_mask() {
        let input = ".####...#####..#...###..#####..#..#.#.####..#.#..#.#...#.###...#.##.##..#.#.##.###.#.##.##.#####..##.###.####..#.####.##...#.#..##.##...#..#..###.##.#..#.#..#..##.#.#...###.##.....#...###.#...#.####.#.#....##.#..#.#.##...#..#....#..#...####..#.##...###..#.#####..#....#.##.#.#####....#.....##.##.###.....#.##..#.#...#...###..####....##..#.##...#.##.#.#.###...##.###.#..####...##..#...#.###...#.##...#.######..###.###.#######..#####...##.#..#..#.#######.####.#..##.########..#..##.#.#####..#.#...##..#....#....##..#.#########..###...#.....#..##...###.###..###....##.#...##.##.#";
        assert_eq!(find_seamonsters(input, 4), 2,);
    }
}
