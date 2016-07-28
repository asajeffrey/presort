extern crate presort;

use presort::PermutedVec;
use std::rc::Rc;
use std::cell::RefCell; 

struct TreeNode<T> {
	//whether this node's data has changed
	dirty_val: bool,
	//whether some child node's data has changed
	dirty_child: bool,
	//number of children with unchanged decendent counts
	stable_children: usize,
	//parent and index in parent's child vec
	parent: Option<(Tree<T>, usize)>,
	//this node's data
    data: T,
    decendent_count: usize,
    children: Vec<Tree<T>>,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;

trait IncTree<T: PartialEq + Clone> {
    fn new_node(data: T) -> Tree<T>;
	fn dirty_decendents(&self);
	fn dirty_parents(&self);
	fn mark_recount(&self, child_index: usize);
	fn get_data(&self) -> T;
	fn set_data(&self, data: T);
	fn get_child(&self, child_num: usize) -> Tree<T>;
	fn set_child(&self, child_num: usize, child: Tree<T>);
	fn push_child(&self, child: Tree<T>);
}

impl<T: PartialEq + Clone> IncTree<T> for Tree<T> {
	fn new_node(data: T) -> Tree<T>{
		Rc::new(RefCell::new(TreeNode {
			dirty_val: true,
			dirty_child: true,
			stable_children: 0,
			parent: None,
			data: data,
			decendent_count: 0,
			children: vec![]
		}))
	}

	//sets dirty flags, but leaves counts
	fn dirty_decendents(&self) {
		let mut tree = self.borrow_mut();
		tree.dirty_val = true;
		tree.dirty_child = true;
		for kid in &tree.children {
			kid.dirty_decendents();
		}
	}

	fn dirty_parents(&self){
		let mut parent = None;
		if let Some((ref p_tree,_)) = self.borrow().parent {
			parent = Some(p_tree.clone()); // Rc clone
		}
		if let Some(parent) = parent {
			parent.borrow_mut().dirty_child = true;
			parent.dirty_parents();
		}
	}

	//when a child changes number of decendents, we mark it in
	//this tree and bubble it up. This does not effect `decendent_count`,
	//it only effects `stable_children`
	fn mark_recount(&self, child_index: usize) {
		let mut tree = self.borrow_mut();
		if tree.stable_children > child_index {
			tree.stable_children = child_index;
			if let Some((ref parent, index)) = tree.parent {
				parent.mark_recount(index)
			}
		}
	}

	fn get_data(&self) -> T {
		self.borrow().data.clone()
	}

	fn set_data(&self, data: T) {
		{
			let mut tree = self.borrow_mut();
			if tree.data == data {return}
			tree.data = data;
			tree.dirty_val = true;
		}
		self.dirty_parents();
	}

	fn get_child(&self, child_num: usize) -> Tree<T> {
		self.borrow().children[child_num].clone() //Rc clone
	}

	fn set_child(&self, child_num: usize, child: Tree<T>) {
		let mut tree = self.borrow_mut();
		//assume all new values are dirty
		child.dirty_decendents();
		child.borrow_mut().parent = Some((self.clone(), child_num)); //Rc clone
		child.dirty_parents();
		let old_child = tree.children.swap_remove(child_num);
		//handle a change in the number of decendents
		if old_child.borrow().decendent_count != child.borrow().decendent_count {
			tree.decendent_count -= child.borrow().decendent_count;
			tree.decendent_count += child.borrow().decendent_count;
			child.borrow_mut().stable_children = 0;
			self.mark_recount(child_num);
		}
		tree.children.push(child);
	}

	fn push_child(&self, child: Tree<T>) {
		//assume all new values are dirty
		child.dirty_decendents();
		child.borrow_mut().parent = Some((self.clone(), self.borrow().children.len())); //Rc clone
		child.dirty_parents();
		let mut tree = self.borrow_mut();
		tree.decendent_count += child.borrow().decendent_count + 1;
		tree.children.push(child);
		if let Some((ref parent, index)) = tree.parent {
			parent.mark_recount(index);
		}
	}
}

fn dump<T: Clone>(tree: &Tree<T>, vec: &mut PermutedVec<T>) {
	let mut tree = tree.borrow_mut();
    vec.push(tree.data.clone());
    tree.dirty_val = false;
    for kid in &tree.children { dump(&kid, vec); }
    tree.dirty_child = false;
    tree.stable_children = tree.children.len();
}

fn update<T: Clone>(tree: &Tree<T>, start_index: usize, vec: &mut PermutedVec<T>) {
	let mut tree = tree.borrow_mut();
	let mut index = start_index;
	if tree.dirty_val {
		vec.set(index, tree.data.clone());
		tree.dirty_val = false;
	}
	if !tree.dirty_child {return}
	index += 1;

	//all the children with unchanged decendent counts
	for counted_kid in 0..tree.stable_children{
		update(&tree.children[counted_kid], index, vec);
		index += 1 + tree.children[counted_kid].borrow_mut().decendent_count;
	}
	if tree.stable_children < tree.children.len() {
		//first changed child may have unchanged decendents
		let first_changed = &tree.children[tree.stable_children];
		update(&first_changed, index, vec);
		index += 1 + first_changed.borrow().decendent_count;
		//dump all the rest
		vec.truncate(index);
		for uncounted_kid in (tree.stable_children + 1)..tree.children.len() {
			dump(&tree.children[uncounted_kid], vec);
		}
	}
}

#[test]
fn test_tree() {
    let main_tree = Tree::new_node(37);
    let mut vec = PermutedVec::new();
    dump(&main_tree, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&37]);
}

#[test]
fn test_update() {
	let main_tree = Tree::new_node(37);
	main_tree.push_child(Tree::new_node(42));
	main_tree.push_child(Tree::new_node(20));
	main_tree.push_child(Tree::new_node(63));
	let mut vec = PermutedVec::new();
	dump(&main_tree, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&20,&37,&42,&63]);

    main_tree.get_child(1).set_data(25);
    update(&main_tree, 0, &mut vec);
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&25,&37,&42,&63]);
}
