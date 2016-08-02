#![feature(test)]

extern crate test;
extern crate rand;
extern crate presort;

use test::Bencher;
use rand::Rng;

use presort::*;
use presort::inc_tree::*;

const NODES: usize = 10000;
const BINARY_DEPTH: usize = 13;
const DEEP_DEPTH: usize = 150;
const WIDE_DEPTH: usize = 6;
const DEPTH: usize = BINARY_DEPTH;

//creates a random tree with the requested parameters
fn build_tree(max_depth: usize, nodes: usize) -> Tree<usize> {
    let mut rng = rand::thread_rng();

    let tree = Tree::new_node(rng.gen());

    for _ in 0..nodes {
        let mut branch = tree.clone();
        let depth = rng.gen::<usize>() % max_depth;
        for _ in 0..depth {
            let num = branch.num_children();
            if num <= 0 {break};
            branch = branch.get_child(rng.gen::<usize>() % num);
        }
        branch.push_child(Tree::new_node(rng.gen()));
    }

    tree
}
// mutates `edits` number of values of `tree`,
// at no more than a max depth of `high_depth`
// shorter branches may have the ends updated more often
fn mutate_vals(tree: &Tree<usize>, high_depth: usize, edits: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..edits {
        let mut branch = tree.clone();
        let depth = rng.gen::<usize>() % high_depth;
        for _ in 0..depth {
            let num = branch.num_children();
            if num <= 0 {break};
            branch = branch.get_child(rng.gen::<usize>() % num);
        }
        branch.set_data(rng.gen());
    }
}

fn add_branches(tree: &Tree<usize>, high_depth: usize, adds: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..adds {
        let mut branch = tree.clone();
        let depth = rng.gen::<usize>() % high_depth;
        for _ in 0..depth {
            let num = branch.num_children();
            if num <= 0 {break};
            branch = branch.get_child(rng.gen::<usize>() % num);
        }
        branch.push_child(Tree::new_node(rng.gen()));
    }
}

#[bench]
fn build(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = PermutedVec::new();
        let tree = build_tree(DEPTH, NODES);
        dump(&tree, &mut vec)
    })
}

#[bench]
fn edit_50_batch(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = PermutedVec::new();
        let tree = build_tree(DEPTH, NODES);
        dump(&tree, &mut vec);
        mutate_vals(&tree, DEPTH, 50);
        update(&tree, 0, &mut vec);
    })
}

#[bench]
fn edit_50_seperate(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = PermutedVec::new();
        let tree = build_tree(DEPTH, NODES);
        dump(&tree, &mut vec);
        for _ in 0..50 {
            mutate_vals(&tree, DEPTH, 1);
            update(&tree, 0, &mut vec);       
        }
    })
}

#[bench]
fn add_50_batch(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = PermutedVec::new();
        let tree = build_tree(DEPTH, NODES);
        dump(&tree, &mut vec);
        add_branches(&tree, DEPTH, 50);
        update(&tree, 0, &mut vec);       
    })
}

#[bench]
fn add_50_seperate(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = PermutedVec::new();
        let tree = build_tree(DEPTH, NODES);
        dump(&tree, &mut vec);
        for _ in 0..50 {
            add_branches(&tree, DEPTH, 50);
            update(&tree, 0, &mut vec);       
        }
    })
}


