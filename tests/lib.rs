extern crate presort;

use presort::PermutedVec;

struct TreeNode<T> {
	//dirty flag for this node, not children
	dirty: bool,
    data: T,
    children: Vec<Tree<T>>,
}

type Tree<T> = Box<TreeNode<T>>;

fn tree<T>(data: T, children: Vec<Tree<T>>) -> Tree<T> {
    Box::new(TreeNode { data: data, dirty: true, children: children })
}

fn dump<T: Clone+Ord>(tree: &mut Tree<T>, vec: &mut PermutedVec<T>) {
    vec.push(tree.data.clone());
    tree.dirty = false;
    for mut kid in &mut tree.children { dump(&mut kid, vec); }
}

// incrementally updates the vec, returning the number of nodes considered
fn update<T: Clone>(tree: &mut Tree<T>, start_index: usize, vec: &mut PermutedVec<T>) -> usize {
	let mut i = start_index;
	if tree.dirty { vec.set(i,tree.data.clone()); tree.dirty = false; }
	i += 1;
	for mut kid in &mut tree.children {
		i += update(&mut kid, i, vec);
	}
	i - start_index
}

#[test]
fn test_tree() {
    let mut main_tree = tree(37, vec![]);
    let mut vec = PermutedVec::new();
    dump(&mut main_tree, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&37]);
}

#[test]
fn test_update() {
	let c1 = tree(42, vec![]);
	let c2 = tree(20, vec![]);
	let c3 = tree(63, vec![]);
	let mut main_tree = tree(37, vec![c1,c2,c3]);
	let mut vec = PermutedVec::new();
	dump(&mut main_tree, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&20,&37,&42,&63]);

    main_tree.children[1].data = 25;
    main_tree.children[1].dirty = true;
    update(&mut main_tree, 0, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&25,&37,&42,&63]);
}
