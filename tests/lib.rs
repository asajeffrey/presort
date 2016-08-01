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
    decendent_count: i32,
    children: Vec<Tree<T>>,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;

trait IncTree<T: PartialEq + Clone> {
    fn new_node(data: T) -> Tree<T>;
	fn dirty_decendents(&self);
	fn dirty_parents(&self);
	fn mark_recount(&self, child_index: usize, new_nodes: i32);
	fn get_data(&self) -> T;
	fn set_data(&self, data: T);
	fn get_child(&self, child_num: usize) -> Tree<T>;
	fn set_child(&self, child_num: usize, child: Tree<T>);
	fn push_child(&self, child: Tree<T>);
	fn pop_child(&self) -> Option<Tree<T>>;
}

impl<T: PartialEq + Clone> IncTree<T> for Tree<T> {
	fn new_node(data: T) -> Tree<T>{
		Rc::new(RefCell::new(TreeNode {
			dirty_val: true,
			dirty_child: false,
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
	//this tree and bubble it up. This updates both
	//`decendent_count` and `stable_children`
	fn mark_recount(&self, child_index: usize, new_nodes: i32) {
		let mut tree = self.borrow_mut();
		tree.decendent_count += new_nodes;
		if tree.stable_children > child_index {
			tree.stable_children = child_index;
		}
		if let Some((ref parent, index)) = tree.parent {
			parent.mark_recount(index, new_nodes)
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
		let mut nodes: i32 = 0;
		{
			//handle a change in the number of decendents
			let ref old_child = tree.children[child_num].borrow();
			if old_child.decendent_count != child.borrow().decendent_count {
				nodes -= old_child.decendent_count;
				nodes += child.borrow().decendent_count;
				child.borrow_mut().stable_children = 0;
				self.mark_recount(child_num, nodes);
			}
		}
		tree.children[child_num] = child;
	}

	fn push_child(&self, child: Tree<T>) {
		//assume all new values are dirty
		child.dirty_decendents();
		child.borrow_mut().parent = Some((self.clone(), self.borrow().children.len())); //Rc clone
		child.dirty_parents();
		let index;
		let nodes;
		{
			let mut tree = self.borrow_mut();
			nodes = 1 + child.borrow().decendent_count;
			index = tree.children.len();
			tree.children.push(child);
		}
		self.mark_recount(index, nodes);
	}

	fn pop_child(&self) -> Option<Tree<T>> {
		//assume all new values are dirty
		let child = self.borrow_mut().children.pop();
		let nodes = match child {
			None => 0,
			Some(ref c) => 1 + c.borrow().decendent_count,
		};
		let index = std::cmp::max(self.borrow().children.len(),0);
		self.mark_recount(index, -nodes);
		child
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
		if vec.len() <= index {
			vec.push(tree.data.clone())
		} else {
			vec.set(index, tree.data.clone());			
		}
		tree.dirty_val = false;
	}
	if !tree.dirty_child {return}
	index += 1;

	//all the children with unchanged decendent counts
	for counted_kid in 0..tree.stable_children{
		update(&tree.children[counted_kid], index, vec);
		index += 1 + tree.children[counted_kid].borrow_mut().decendent_count as usize;
	}
	if tree.stable_children < tree.children.len() {
		{
			//first changed child may have unchanged decendents
			let first_changed = &tree.children[tree.stable_children];
			update(&first_changed, index, vec);
			index += 1 + first_changed.borrow().decendent_count as usize;
		}
		//dump any additional children
		if (tree.stable_children + 1) < tree.children.len() {
			vec.truncate(index);
			for uncounted_kid in (tree.stable_children + 1)..tree.children.len() {
				dump(&tree.children[uncounted_kid], vec);
			}
			tree.stable_children = tree.children.len();
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
	println!("start first dump");
	let main_tree = Tree::new_node(37);
	main_tree.push_child(Tree::new_node(42));
	main_tree.push_child(Tree::new_node(20));
	main_tree.push_child(Tree::new_node(63));
	let mut vec = PermutedVec::new();
	dump(&main_tree, &mut vec);
	println!("finished first dump");
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&20,&37,&42,&63]);

	println!("start first update");
    main_tree.get_child(1).set_data(25);
    update(&main_tree, 0, &mut vec);
	println!("finished first update");
    assert_eq!(vec.sorted_iter().collect::<Vec<&usize>>(), vec![&25,&37,&42,&63]);

	println!("start add branch");
    let branch = main_tree.get_child(2);
    branch.push_child(Tree::new_node(47));
    branch.push_child(Tree::new_node(53));
    branch.push_child(Tree::new_node(61));
    branch.push_child(Tree::new_node(57));
    update(&main_tree, 0, &mut vec);
	println!("finished add branch");
    assert_eq!(
    	vec.sorted_iter().collect::<Vec<&usize>>(),
    	vec![&25,&37,&42,&47,&53,&57,&61,&63]
    );

	println!("start update branch");
    branch.get_child(2).set_data(77);
    update(&main_tree, 0, &mut vec);
	println!("finished update branch");
    assert_eq!(
    	vec.sorted_iter().collect::<Vec<&usize>>(),
    	vec![&25,&37,&42,&47,&53,&57,&63,&77]
    );

    println!("start early insertion");
    let branch = main_tree.get_child(1);
    branch.push_child(Tree::new_node(1));
    update(&main_tree, 0, &mut vec);
    println!("finished early insertion");
    assert_eq!(
    	vec.sorted_iter().collect::<Vec<&usize>>(),
    	vec![&1,&25,&37,&42,&47,&53,&57,&63,&77]
    );

    println!("start deep add");
    let branch = main_tree.get_child(2).get_child(2);
    let new_branch = Tree::new_node(100);
    new_branch.push_child(Tree::new_node(101));
    branch.push_child(new_branch);
    update(&main_tree, 0, &mut vec);
    println!("finished deep add");
    assert_eq!(
    	vec.sorted_iter().collect::<Vec<&usize>>(),
    	vec![&1,&25,&37,&42,&47,&53,&57,&63,&77,&100,&101]
    );

    println!("start deep edit");
    let branch = branch.get_child(0).get_child(0);
    branch.set_data(105);
    update(&main_tree, 0, &mut vec);
    println!("finished deep edit");
    assert_eq!(
    	vec.sorted_iter().collect::<Vec<&usize>>(),
    	vec![&1,&25,&37,&42,&47,&53,&57,&63,&77,&100,&105]
    );

    println!("start pop branch");
    let branch = main_tree.get_child(2).get_child(2);
    branch.pop_child();
    update(&main_tree, 0, &mut vec);
    println!("finished pop branch");
    assert_eq!(
    	vec.sorted_iter().collect::<Vec<&usize>>(),
    	vec![&1,&25,&37,&42,&47,&53,&57,&63,&77]
    );

}
