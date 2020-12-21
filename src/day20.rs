use std::io;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    let solution = match part {
        1 => part_1(input),
        2 => part_2(input),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

type TileId = usize;
type EdgeIdx = u8;
type Pixel = bool;
type Edge = Vec<Pixel>;

#[derive(Debug, Clone)]
struct Tile(Vec<Vec<Pixel>>);

impl Tile {
    /// Rotate 90deg anti-clockwise
    fn rotate(&self) -> Self {
        let mut tile = Vec::new();

        let n = self.0.len();

        for _ in 0..n {
            let j = n - 1 - tile.len();
            tile.push(self.0.iter().map(|r| r[j]).collect());
        }

        Self(tile)
    }

    /// Flip in the vertical direction (around the horizontal axis)
    fn flip_v(&self) -> Self {
        Self(self.0.iter().cloned().rev().collect())
    }

    /// Flip in the horizontal direction (around the vertical axis)
    fn flip_h(&self) -> Self {
        Self(
            self.0
                .iter()
                .cloned()
                .map(|r| r.into_iter().rev().collect())
                .collect(),
        )
    }

    // All possible transformations (including the original one)
    fn possible_transforms(self) -> Vec<Self> {
        let mut trans: Vec<Self> = Vec::new();

        let v_flipped = self.flip_v();
        let h_flipped = self.flip_h();
        let d1_flipped = self.flip_h().flip_v();
        let d2_flipped = self.flip_v().flip_h();

        trans.push(self.rotate());
        trans.push(v_flipped.rotate());
        trans.push(h_flipped.rotate());
        trans.push(d1_flipped.rotate());
        trans.push(d2_flipped.rotate());

        trans.push(self.rotate().rotate());
        trans.push(v_flipped.rotate().rotate());
        trans.push(h_flipped.rotate().rotate());
        trans.push(d1_flipped.rotate().rotate());
        trans.push(d2_flipped.rotate().rotate());

        trans.push(self.rotate().rotate().rotate());
        trans.push(v_flipped.rotate().rotate().rotate());
        trans.push(h_flipped.rotate().rotate().rotate());
        trans.push(d1_flipped.rotate().rotate().rotate());
        trans.push(d2_flipped.rotate().rotate().rotate());

        trans.push(self);
        trans.push(v_flipped);
        trans.push(h_flipped);
        trans.push(d1_flipped);
        trans.push(d2_flipped);

        trans
    }

    // N, W, S, E
    // N, S: left to right
    // W, E: top to bottom
    fn edges(&self) -> Vec<Edge> {
        let n = self.0.len();
        assert_eq!(self.0[0].len(), n);
        assert_eq!(self.0[9].len(), n);

        vec![
            self.0[0].clone(),
            self.0.iter().map(|r| r[0]).collect(),
            self.0[n - 1].clone(),
            self.0.iter().map(|r| r[n - 1]).collect(),
        ]
    }
}

type Tiles = HashMap<TileId, Tile>;

fn parse(mut input: impl BufRead) -> Tiles {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    let mut tiles = HashMap::new();
    let mut tile = Vec::new();
    let mut tile_id = None;

    for line in input_str.lines() {
        if line.is_empty() {
            if let Some(id) = tile_id {
                let mut finished_tile = Vec::new();
                std::mem::swap(&mut finished_tile, &mut tile);
                tiles.insert(id, Tile(finished_tile));
            }
        } else if let Some(id) = line.strip_prefix("Tile ") {
            let id = id.strip_suffix(":").unwrap();
            let id = id.parse::<usize>().unwrap();
            tile_id = Some(id);
        } else {
            let row: Vec<Pixel> = line.as_bytes().iter().map(|b| b == &b'#').collect();
            tile.push(row);
        }
    }

    tiles.insert(tile_id.unwrap(), Tile(tile));

    tiles
}

type EdgeId = (TileId, EdgeIdx, bool);

fn part_1(tiles: Tiles) -> usize {
    let mut edges: HashMap<Edge, Vec<EdgeId>> = HashMap::new();

    for (tile_id, tile) in tiles.iter() {
        let iter = tile.edges().into_iter().enumerate().flat_map(|(i, v)| {
            vec![
                (i, v.clone(), false),
                (i, v.into_iter().rev().collect(), true),
            ]
            .into_iter()
        });

        for (i, edge, reversed) in iter {
            let entry = edges.entry(edge).or_insert_with(Vec::new);
            entry.push((*tile_id, i as EdgeIdx, reversed));
        }
    }

    let mut unique = HashMap::new();

    for mut edge_ids in edges
        .values()
        .cloned()
        .into_iter()
        .filter(|edge_ids| edge_ids.len() == 1)
    {
        // Edge with no connections
        let (tile_id, edge_id, _reversed) = edge_ids.pop().unwrap();

        let entry = unique.entry(tile_id).or_insert_with(HashSet::new);
        entry.insert(edge_id);
    }

    unique
        .into_iter()
        .filter(|(_, edge_set)| edge_set.len() == 2)
        .map(|(tile_id, _)| tile_id)
        .product()
}

fn part_2(_tiles: Tiles) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

    #[test]
    fn part_1_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_1(input), 20899048083289);
    }
}
