#![feature(test)]

extern crate rand;
extern crate test;
extern crate seqalign;

use test::Bencher;

use rand::{Rng, weak_rng};
use seqalign::Align;
use seqalign::measures::Levenshtein;

static BENCH_ALPHABET: &[char] = &['a', 'b', 'c', 'd', 'e'];

fn random_string<R>(rng: &mut R, len: usize) -> Vec<char> where R: Rng {
	(0..len).map(|_| rng.choose(BENCH_ALPHABET).unwrap()).cloned().collect()
}

fn random_pair<R>(rng: &mut R) -> (Vec<char>, Vec<char>) where R: Rng {
	let s1_len = rng.gen_range(0, 20);
	let s2_len = rng.gen_range(0, 20);

	let s1 = random_string(rng, s1_len);
	let s2 = random_string(rng, s2_len);

	(s1, s2)
}

#[bench]
fn levenshtein_distance_1000(b: &mut Bencher) {
	let mut rng = weak_rng();

	let mut pairs = Vec::new();
	for _ in 0..1000 {
		pairs.push(random_pair(&mut rng));

	}

	let levensthein = Levenshtein::new(1, 1, 1);

	b.iter(move || {
		for &(ref s1, ref s2) in &pairs {
			levensthein.align(s1, s2);
		}
    })
}