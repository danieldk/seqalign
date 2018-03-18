#![feature(test)]

extern crate rand;
extern crate seqalign;
extern crate test;

use test::Bencher;

use rand::{weak_rng, Rng};
use seqalign::{Align, Measure};
use seqalign::measures::{Levenshtein, LevenshteinDamerau, LCS};

static BENCH_ALPHABET: &[char] = &['a', 'b', 'c', 'd', 'e'];

fn random_string<R>(rng: &mut R, len: usize) -> Vec<char>
where
    R: Rng,
{
    (0..len)
        .map(|_| rng.choose(BENCH_ALPHABET).unwrap())
        .cloned()
        .collect()
}

fn random_pair<R>(rng: &mut R) -> (Vec<char>, Vec<char>)
where
    R: Rng,
{
    let s1_len = rng.gen_range(0, 20);
    let s2_len = rng.gen_range(0, 20);

    let s1 = random_string(rng, s1_len);
    let s2 = random_string(rng, s2_len);

    (s1, s2)
}

fn distance_bench<M>(b: &mut Bencher, measure: M)
where
    M: Measure<char>,
{
    let mut rng = weak_rng();

    let mut pairs = Vec::new();
    for _ in 0..1000 {
        pairs.push(random_pair(&mut rng));
    }

    b.iter(move || {
        for &(ref s1, ref s2) in &pairs {
            measure.align(s1, s2);
        }
    })
}

#[bench]
fn lcs_distance_1000(b: &mut Bencher) {
    distance_bench(b, LCS::new(1, 1));
}

#[bench]
fn levenshtein_damerau_distance_1000(b: &mut Bencher) {
    distance_bench(b, LevenshteinDamerau::new(1, 1, 1, 1));
}

#[bench]
fn levenshtein_distance_1000(b: &mut Bencher) {
    distance_bench(b, Levenshtein::new(1, 1, 1));
}
