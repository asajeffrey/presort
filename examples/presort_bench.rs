extern crate rand;
extern crate time;
#[macro_use]
extern crate clap;
extern crate presort;

mod inc_tree;

use std::fs::OpenOptions;
use std::io::Write;
use rand::Rng;
use time::Duration;
use clap::{App, Arg};
use presort::{PresortedVec, PermutedVec};
use inc_tree::{Tree, IncTree, dump, update, update_no_pad};
use inc_tree::sortvec::SortVec;

fn main() {
    //command-line
    let args = App::new("presort_bench")
    .arg(Arg::with_name("vec")
        .long("vec")
        .help("Use the default vector implementation")
        .conflicts_with_all(&["presort","presort_pad","permut","permut_pad"])
    ).arg(Arg::with_name("presort")
        .long("presort")
        .help("Use the presorted vector implementation")
        .conflicts_with_all(&["vec","presort_pad","permut","permut_pad"])
    ).arg(Arg::with_name("presort_pad")
        .long("presort_pad")
        .help("Use the persorted vector implementation with padding")
        .conflicts_with_all(&["vec","presort","permut","permut_pad"])
    ).arg(Arg::with_name("permut")
        .long("permut")
        .help("Use the permuted vector implementation")
        .conflicts_with_all(&["vec","presort","persort_pad","permut_pad"])
    ).arg(Arg::with_name("permut_pad")
        .long("permut_pad")
        .help("Use the permuted vector implementation with padding")
        .conflicts_with_all(&["vec","presort","persort_pad","permut"])
    ).args_from_usage("\
        --tag [tag]                                 'max depth of initial tree'
        [data_size] -b [data_size]                  'data size in bytes (unused)'
        [initial_max_depth] -d [initial_max_depth]  'max depth of initial tree'
        [initial_nodes] -n [initial_nodes]          'nodes of initial tree'
        [edits] -e [edits]                          'nodes to modify'
        [shape] -s [shape]                          'chance for a shape edit'
        [add] -a [add]                              'chance to add a branch (rather than remove)'
        [change] -c [change]                        'chance to change sort order'
        [trials] -t [trials]                        'repetitions to average'
        [outfile] -o [outfile]                      'append output to this file'
        [header] -h                                 'write out a header to the results' ")
    .get_matches();
    let tag = match args.value_of("tag") {
        None => "none",
        Some(t) => t
    };
    let b = value_t!(args.value_of("data_size"), usize).unwrap_or(4);
    let d = value_t!(args.value_of("initial_max_depth"), usize).unwrap_or(13);
    let n = value_t!(args.value_of("initial_nodes"), usize).unwrap_or(10000);
    let e = value_t!(args.value_of("edits"), usize).unwrap_or(0);
    let s = value_t!(args.value_of("shape"), f32).unwrap_or(0.0);
    let a = value_t!(args.value_of("add"), f32).unwrap_or(0.5);
    let c = value_t!(args.value_of("change"), f32).unwrap_or(0.5);
    let t = value_t!(args.value_of("trials"), usize).unwrap_or(1);
    let mut o: Box<Write> = if let Some(f) = args.value_of("outfile") {
        Box::new(
            OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(f)
            .unwrap()
        )
    } else { Box::new(std::io::stdout()) };
    
    //write out header
    if args.is_present("header"){
        writeln!(o, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            "timestamp",
            "user_tag",
            "vec_type",
            "data_size",
            "depth",
            "nodes",
            "edits",
            "chance_shape",
            "chance_add",
            "chance_reorder",
            "time_dump",
            "time_init_sort",
            "time_modification",
            "time_update",
            "time_sort"
        ).unwrap();
    }
    if t == 0 {return}

    // average multiple trials
    let mut dur_dump = Duration::seconds(0);
    let mut dur_init_sort = Duration::seconds(0);
    let mut dur_modify = Duration::seconds(0);
    let mut dur_update = Duration::seconds(0);
    let mut dur_sort = Duration::seconds(0);

    for _ in 0..t {
        //create target vector
        let mut vec =
            if args.is_present("vec") {
                VecVersion::Vec(Vec::new())
            } else if args.is_present("presort") {
                VecVersion::Presort(PresortedVec::new())
            } else if args.is_present("presort_pad") {
                VecVersion::PrePad(PresortedVec::new())
            } else if args.is_present("permuted") {
                VecVersion::Permut(PermutedVec::new())
            } else if args.is_present("permuted_pad") {
                VecVersion::PerPad(PermutedVec::new())
            } else {
                VecVersion::Vec(Vec::new())
            };

        //initial tree creation
        let tree = build_tree(d,n);

        //initial dump
        dur_dump = dur_dump + Duration::span(||{
            match vec {
                VecVersion::Vec(ref mut v) => dump(&tree, v),
                VecVersion::Presort(ref mut v) => dump(&tree, v),
                VecVersion::PrePad(ref mut v) => dump(&tree, v),
                VecVersion::Permut(ref mut v) => dump(&tree, v),
                VecVersion::PerPad(ref mut v) => dump(&tree, v),
            };
        });

        //initial sort
        dur_init_sort = dur_init_sort + Duration::span(||{
            match vec {
                VecVersion::Vec(ref mut v) => v.sort(),
                VecVersion::Presort(ref mut v) => v.sort(),
                VecVersion::PrePad(ref mut v) => v.sort(),
                VecVersion::Permut(ref mut v) => v.sort(),
                VecVersion::PerPad(ref mut v) => v.sort(),
            };
        });

        //modify tree
        dur_modify = dur_modify + Duration::span(||{
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < s {
                if rng.gen::<f32>() < a {
                    add_branches(&tree, d, e);
                } else {
                    remove_branches(&tree, d, e);
                }
            } else {
                if rng.gen::<f32>() < c {
                    mutate_vals(&tree, d, e);
                } else {
                    incr_vals(&tree, d, e);
                }
            }
        });

        //update tree
        dur_update = dur_update + Duration::span(||{
            match vec {
                VecVersion::Vec(ref mut v) => update_no_pad(&tree, 0, v),
                VecVersion::Presort(ref mut v) => update_no_pad(&tree, 0, v),
                VecVersion::PrePad(ref mut v) => update(&tree, 0, v),
                VecVersion::Permut(ref mut v) => update_no_pad(&tree, 0, v),
                VecVersion::PerPad(ref mut v) => update(&tree, 0, v),
            };
        });

        //sort tree
        dur_sort = dur_sort + Duration::span(||{
            match vec {
                VecVersion::Vec(ref mut v) => v.sort(),
                VecVersion::Presort(ref mut v) => v.sort(),
                VecVersion::PrePad(ref mut v) => v.sort(),
                VecVersion::Permut(ref mut v) => v.sort(),
                VecVersion::PerPad(ref mut v) => v.sort(),
            };
        });
    }

    //write out results
    let t = t as i64;
    let vec =
        if args.is_present("vec") {"vec"}
        else if args.is_present("presort") {"presort"}
        else if args.is_present("presort_pad") {"presort_pad"}
        else if args.is_present("permuted") {"permuted"}
        else if args.is_present("permuted_pad") {"permuted_pad"}
        else {"vec"};

    writeln!(o, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        time::precise_time_s() as usize,
        tag,vec,b,d,n,e,s,a,c,
        dur_dump.num_nanoseconds().unwrap()/t,
        dur_init_sort.num_nanoseconds().unwrap()/t,
        dur_modify.num_nanoseconds().unwrap()/t,
        dur_update.num_nanoseconds().unwrap()/t,
        dur_sort.num_nanoseconds().unwrap()/t
    ).unwrap();
}

enum VecVersion<T: Ord> {
    Vec(Vec<T>),
    Presort(PresortedVec<T>),
    PrePad(PresortedVec<T>),
    Permut(PermutedVec<T>),
    PerPad(PermutedVec<T>),
}

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

fn random_subtree(tree: &Tree<usize>, high_depth: usize) -> Tree<usize> {
    let mut rng = rand::thread_rng();
    let mut subtree = tree.clone();
    let depth = rng.gen::<usize>() % high_depth;
    for _ in 0..depth {
        let num = subtree.num_children();
        if num <= 0 { return subtree; }
        subtree = subtree.get_child(rng.gen::<usize>() % num);
    }
    return subtree;
}

fn mutate_vals(tree: &Tree<usize>, high_depth: usize, edits: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..edits {
        random_subtree(tree, high_depth).set_data(rng.gen());
    }
}

fn incr_vals(tree: &Tree<usize>, high_depth: usize, edits: usize) {
    for _ in 0..edits {
        let subtree = random_subtree(tree, high_depth);
        subtree.set_data(subtree.get_data() + 1);
    }
}

fn add_branches(tree: &Tree<usize>, high_depth: usize, adds: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..adds {
        random_subtree(tree, high_depth).push_child(Tree::new_node(rng.gen()));
    }
}

fn remove_branches(tree: &Tree<usize>, high_depth: usize, removes: usize) {
    let mut i = 0;
    while i<removes {
        if let Some(_) = random_subtree(tree, high_depth).pop_child() {
            i = i + 1;
        }
    }
}

