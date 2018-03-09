![crates.io](https://img.shields.io/crates/v/seqalign.svg)
![docs.rs](https://docs.rs/seqalign/badge.svg)
![Travis CI](https://img.shields.io/travis/sfb833-a3/seqalign.svg)

# seqalign

## Introduction

This crate implements commonly-used sequence alignment methods based on
edit operations. There are multiple crates available to compute edit
distances. However, to my knowledge there was no crate that supports
all of the following seqalign features:

* Works on slices of any type.
* Can return both the edit distance and the edit script/alignment.
* Can be extended with new measures.

## Example

```rust
use seqalign::Align;
use seqalign::measures::LevenshteinDamerau;

let incorrect = &['t', 'p', 'y', 'o'];
let correct = &['t', 'y', 'p', 'o', 's'];

let measure = LevenshteinDamerau::new(1, 1, 1, 1);
let alignment = measure.align(incorrect, correct);

// Get the edit distance
assert_eq!(2, alignment.distance());

// Get the edit script.
use seqalign::measures::LevenshteinDamerauOp;
use seqalign::op::IndexedOperation;

assert_eq!(vec![
  	IndexedOperation::new(LevenshteinDamerauOp::Match, 0, 0),
  	IndexedOperation::new(LevenshteinDamerauOp::Transpose(1), 1, 1),
  	IndexedOperation::new(LevenshteinDamerauOp::Match, 3, 3),
  	IndexedOperation::new(LevenshteinDamerauOp::Insert(1), 4, 4)
  ], alignment.edit_script());
```
