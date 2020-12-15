use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn solve(input: impl BufRead, part: u8) -> io::Result<()> {
    let input = parse(input);

    let solution = match part {
        1 => part_1(&input),
        2 => part_2(&input),
        _ => unimplemented!(),
    };

    println!("{}", solution);

    Ok(())
}

fn parse(mut input: impl BufRead) -> Vec<usize> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();

    input_str.pop(); // Remove newline
    input_str
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part_1(input: &[usize]) -> usize {
    simulate_to::<HashMap<usize, usize>>(2020, input)
}

fn part_2(input: &[usize]) -> usize {
    simulate_to::<VecMap<usize>>(30000000, input)
}

trait Insertable {
    type Key;
    type Item;
    fn insert(&mut self, k: Self::Key, v: Self::Item) -> Option<Self::Item>;
}

impl<K: Eq + Hash, T> Insertable for HashMap<K, T> {
    type Key = K;
    type Item = T;
    fn insert(&mut self, k: K, v: T) -> Option<T> {
        HashMap::insert(self, k, v)
    }
}

fn simulate_to<M>(nth: usize, input: &[usize]) -> usize
where
    M: FromIterator<(usize, usize)> + Insertable<Key = usize, Item = usize>,
{
    let last_start_index = input.len() - 1;

    // Populate with starting numbers
    let mut last_seen: M = input[0..last_start_index]
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect();
    let mut current = *input.last().unwrap();

    for i in last_start_index..(nth - 1) {
        match last_seen.insert(current, i) {
            None => current = 0,
            Some(prev) => current = i - prev,
        }
    }

    current
}

struct VecMap<T>(Vec<Option<T>>);

impl<T> Insertable for VecMap<T> {
    type Key = usize;
    type Item = T;

    fn insert(&mut self, k: usize, x: T) -> Option<T> {
        while self.0.len() <= k {
            self.0.push(None);
        }

        let out = self.0[k].take();
        self.0[k] = Some(x);
        out
    }
}

impl<T> FromIterator<(usize, T)> for VecMap<T> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut map = Self(Vec::new());
        for (i, x) in iter {
            map.insert(i, x);
        }

        map
    }
}

struct SizedMap<T, const N: usize>([Option<T>; N]);

impl<T, const N: usize> Insertable for SizedMap<T, N> {
    type Key = usize;
    type Item = T;

    fn insert(&mut self, k: usize, x: T) -> Option<T> {
        let k = k % N; // ¯\_(ツ)_/¯
        let out = self.0[k].take();
        self.0[k] = Some(x);
        out
    }
}

impl<T: Copy, const N: usize> FromIterator<(usize, T)> for SizedMap<T, N> {
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mut map = Self([None; N]);
        for (i, x) in iter {
            map.insert(i, x);
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn simulate_examples() {
        assert_eq!(simulate_to::<HashMap<usize, usize>>(2020, &[1, 3, 2]), 1);
        assert_eq!(simulate_to::<HashMap<usize, usize>>(2020, &[2, 1, 3]), 10);
        assert_eq!(simulate_to::<HashMap<usize, usize>>(2020, &[1, 2, 3]), 27);
        assert_eq!(simulate_to::<HashMap<usize, usize>>(2020, &[2, 3, 1]), 78);
        assert_eq!(simulate_to::<HashMap<usize, usize>>(2020, &[3, 2, 1]), 438);
        assert_eq!(simulate_to::<HashMap<usize, usize>>(2020, &[3, 1, 2]), 1836);
    }

    #[test]
    fn simulate_examples_vec_map() {
        assert_eq!(simulate_to::<VecMap<usize>>(2020, &[1, 3, 2]), 1);
        assert_eq!(simulate_to::<VecMap<usize>>(2020, &[2, 1, 3]), 10);
        assert_eq!(simulate_to::<VecMap<usize>>(2020, &[1, 2, 3]), 27);
        assert_eq!(simulate_to::<VecMap<usize>>(2020, &[2, 3, 1]), 78);
        assert_eq!(simulate_to::<VecMap<usize>>(2020, &[3, 2, 1]), 438);
        assert_eq!(simulate_to::<VecMap<usize>>(2020, &[3, 1, 2]), 1836);
    }

    #[test]
    fn simulate_examples_sized_map() {
        assert_eq!(simulate_to::<SizedMap<usize, 2048>>(2020, &[1, 3, 2]), 1);
        assert_eq!(simulate_to::<SizedMap<usize, 2048>>(2020, &[2, 1, 3]), 10);
        assert_eq!(simulate_to::<SizedMap<usize, 2048>>(2020, &[1, 2, 3]), 27);
        assert_eq!(simulate_to::<SizedMap<usize, 2048>>(2020, &[2, 3, 1]), 78);
        assert_eq!(simulate_to::<SizedMap<usize, 2048>>(2020, &[3, 2, 1]), 438);
        assert_eq!(simulate_to::<SizedMap<usize, 2048>>(2020, &[3, 1, 2]), 1836);
    }

    #[bench]
    fn bench_hash_map(b: &mut Bencher) {
        b.iter(|| simulate_to::<HashMap<usize, usize>>(2020, &[3, 1, 2]));
    }

    #[bench]
    fn bench_vec_map(b: &mut Bencher) {
        b.iter(|| simulate_to::<VecMap<usize>>(2020, &[3, 1, 2]));
    }

    #[bench]
    fn bench_size_map(b: &mut Bencher) {
        b.iter(|| simulate_to::<SizedMap<usize, 2048>>(2020, &[3, 1, 2]));
    }
}
