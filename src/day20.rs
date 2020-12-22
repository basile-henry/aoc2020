use std::fmt::Debug;
use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

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

#[derive(Clone)]
struct Tile(Vec<Vec<Pixel>>);

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in self.0.iter() {
            for cell in row {
                f.write_str(if *cell { "#" } else { "." })?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

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
    fn _flip_v(&self) -> Self {
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

    // N, W, S, E
    // N: right to left
    // W: top to bottom
    // S: left to right
    // E: bottom to top
    fn edges(&self) -> Vec<Edge> {
        let n = self.0.len();
        assert_eq!(self.0[0].len(), n);
        assert_eq!(self.0[n - 1].len(), n);

        vec![
            self.0[0].iter().rev().copied().collect(),
            self.0.iter().map(|r| r[0]).collect(),
            self.0[n - 1].clone(),
            self.0.iter().map(|r| r[n - 1]).rev().collect(),
        ]
    }

    fn remove_edges(&mut self) {
        let _ = self.0.pop();
        let _ = self.0.remove(0);

        for row in self.0.iter_mut() {
            let _ = row.pop();
            let _ = row.remove(0);
        }
    }

    fn orient(&mut self, mut top: EdgeIdx, reversed: bool) {
        while top % 4 != 0 {
            *self = self.rotate();
            top += 1;
        }

        if reversed {
            *self = self.flip_h();
        }
    }

    fn find_pattern(&self, pattern: &Tile) -> usize {
        let mut count = 0;

        for y_off in 0..self.0.len() - pattern.0.len() {
            for x_off in 0..self.0[0].len() - pattern.0[0].len() {
                let pat_match = pattern.0.iter().enumerate().all(|(y, row)| {
                    row.iter().enumerate().all(|(x, pat)| {
                        if *pat {
                            self.0[y + y_off][x + x_off]
                        } else {
                            true
                        }
                    })
                });

                if pat_match {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_set_pixels(&self) -> usize {
        self.0
            .iter()
            .map(|r| r.iter().filter(|p| **p).count())
            .sum()
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

fn edge_map(tiles: &Tiles) -> HashMap<Edge, Vec<EdgeId>> {
    let mut edges = HashMap::new();

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

    edges
}

fn unique_edges(edges: &HashMap<Edge, Vec<EdgeId>>) -> HashMap<TileId, HashMap<EdgeIdx, bool>> {
    let mut unique = HashMap::new();

    for mut edge_ids in edges
        .values()
        .cloned()
        .into_iter()
        .filter(|edge_ids| edge_ids.len() == 1)
    {
        // Edge with no connections
        let (tile_id, edge_id, reversed) = edge_ids.pop().unwrap();

        let entry = unique.entry(tile_id).or_insert_with(HashMap::new);
        entry.insert(edge_id, reversed);
    }

    unique
}

fn tiles_with_free_edges(
    unique: &HashMap<TileId, HashMap<EdgeIdx, bool>>,
    free_edge_count: usize,
) -> impl Iterator<Item = (&TileId, &HashMap<EdgeIdx, bool>)> {
    unique
        .iter()
        .filter(move |(_, edge_set)| edge_set.len() == free_edge_count)
}

fn part_1(tiles: Tiles) -> usize {
    let edges = edge_map(&tiles);
    let unique = unique_edges(&edges);

    tiles_with_free_edges(&unique, 2)
        .map(|(tile_id, _)| tile_id)
        .product()
}

fn search_for_edge(
    edges: &HashMap<Edge, Vec<EdgeId>>,
    neighbour_id: &TileId,
    neighbour: &Tile,
    neighbour_edge_ix: &EdgeIdx,
) -> EdgeId {
    // Reversing so that they match side by side (anti-clockwise indexing)
    let common_edge: Edge = Tile::edges(neighbour)[*neighbour_edge_ix as usize]
        .iter()
        .rev()
        .copied()
        .collect();

    *edges
        .get(&common_edge)
        .unwrap()
        .iter()
        .find(|(tile_id, _, _)| tile_id != neighbour_id)
        .unwrap()
}

fn solve_puzzle(tiles: Tiles) -> Tile {
    let edges = edge_map(&tiles);
    let unique = unique_edges(&edges);

    let width = f32::sqrt(tiles.len() as f32) as usize;
    let height = width; // It's a square!

    let mut row = Vec::new();
    let mut grid = Vec::new();

    while grid.len() < height {
        while row.len() < width {
            let left_neighbour = row.last();
            let top_neighbour = grid
                .last()
                .and_then(|r: &Vec<(usize, Tile)>| r.get(row.len()));

            let new_tile = match (left_neighbour, top_neighbour) {
                (None, None) => {
                    // Using an arbitrary corner as the top left corner
                    let (top_left_id, top_left_free_edges) =
                        tiles_with_free_edges(&unique, 2).next().unwrap();
                    let mut top_left = tiles.get(&top_left_id).unwrap().clone();

                    let mut top_left_free_edges: Vec<EdgeIdx> =
                        top_left_free_edges.iter().map(|(e, _)| *e).collect();
                    top_left_free_edges.sort_unstable();

                    // Reorient top_left
                    match top_left_free_edges[..] {
                        [0, 3] => top_left.orient(3, false),
                        [top, _] => top_left.orient(top, false),
                        _ => unreachable!(),
                    }
                    (*top_left_id, top_left)
                }

                // Top row
                (Some((left_neighbour_id, left_neighbour)), None) => {
                    let (tile_id, left_edge_idx, left_reversed) =
                        search_for_edge(&edges, left_neighbour_id, left_neighbour, &3);

                    let mut tile = tiles.get(&tile_id).unwrap().clone();

                    if left_reversed {
                        tile.orient(left_edge_idx + 1, true);
                    } else {
                        tile.orient(left_edge_idx + 3, false);
                    }

                    (tile_id, tile)
                }
                // New row, first element
                (None, Some((top_neighbour_id, top_neighbour))) => {
                    let (tile_id, top_edge_idx, top_reversed) =
                        search_for_edge(&edges, top_neighbour_id, top_neighbour, &2);

                    let mut tile = tiles.get(&tile_id).unwrap().clone();
                    tile.orient(top_edge_idx, top_reversed);

                    (tile_id, tile)
                }
                (
                    Some((left_neighbour_id, left_neighbour)),
                    Some((top_neighbour_id, top_neighbour)),
                ) => {
                    let (tile_id, _, left_reversed) =
                        search_for_edge(&edges, left_neighbour_id, left_neighbour, &3);

                    let (_, top, _) = search_for_edge(&edges, top_neighbour_id, top_neighbour, &2);

                    let mut tile = tiles.get(&tile_id).unwrap().clone();

                    tile.orient(top, left_reversed);

                    (tile_id, tile)
                }
            };

            row.push(new_tile);
        }

        let mut new_row = Vec::new();
        std::mem::swap(&mut new_row, &mut row);
        grid.push(new_row);
    }

    // Remove all tile edges and merge into a big picture
    let mut picture: Vec<Vec<Pixel>> = std::iter::repeat(Vec::new())
        .take(height * (10 - 2))
        .collect();

    for (row_ix, row) in grid.into_iter().enumerate() {
        for (_, mut tile) in row {
            tile.remove_edges();

            for (tile_row_ix, mut tile_row) in tile.0.into_iter().enumerate() {
                picture[row_ix * (10 - 2) + tile_row_ix].append(&mut tile_row);
            }
        }
    }

    Tile(picture)
}

fn part_2(tiles: Tiles) -> usize {
    let mut picture = solve_puzzle(tiles);

    let sea_monster_raw = "                  # \n\
#    ##    ##    ###
 #  #  #  #  #  #   ";

    let sea_monster = Tile(
        sea_monster_raw
            .lines()
            .map(|l| l.as_bytes().iter().map(|b| b == &b'#').collect())
            .collect(),
    );

    let mut max_pattern_count = 0;
    // For each rotation
    for _ in 0..=3 {
        // For each flip
        for _ in 0..=1 {
            max_pattern_count = max_pattern_count.max(picture.find_pattern(&sea_monster));
            picture = picture.flip_h();
        }
        picture = picture.rotate();
    }

    let sea_monster_set_pixel_count = sea_monster.count_set_pixels();
    let picture_set_pixel_count = picture.count_set_pixels();

    picture_set_pixel_count - max_pattern_count * sea_monster_set_pixel_count
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

    #[test]
    fn part_2_example() {
        let input = parse(io::Cursor::new(EXAMPLE));
        assert_eq!(part_2(input), 273);
    }
}
