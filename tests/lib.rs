extern crate presort;

use presort::PermutedVec;
use std::rc::Rc;
use std::cell::RefCell; 

struct TreeNode<T> {
	//whether this node's data has changed
	dirty_val: bool,
	//whether some descendant is dirty
	dirty_descendant: bool,
	//the old size of the tree
	old_size: usize,
	//parent and index in parent's child vec
	parent: Option<(Tree<T>, usize)>,
	//this node's data
    data: T,
    children: Vec<Tree<T>>,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;

trait IncTree<T: PartialEq + Clone> {
    fn new_node(data: T) -> Tree<T>;
	fn dirty_decendents(&self);
	fn dirty_ancestors(&self);
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
			dirty_descendant: true,
			old_size: 0,
			parent: None,
			data: data,
			children: vec![]
		}))
	}

	//sets dirty flags, but leaves counts
	fn dirty_decendents(&self) {
		let mut tree = self.borrow_mut();
		tree.dirty_val = true;
		tree.dirty_descendant = true;
		for kid in &tree.children {
			kid.dirty_decendents();
		}
	}

        fn dirty_ancestors(&self){
                let mut node = self.borrow_mut();
                node.dirty_descendant = true;
                if let Some((ref parent, _)) = node.parent {
                    parent.dirty_ancestors();
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
		self.dirty_ancestors();
	}

	fn get_child(&self, child_num: usize) -> Tree<T> {
		self.borrow().children[child_num].clone() //Rc clone
	}

	fn set_child(&self, child_num: usize, child: Tree<T>) {
		//assume all new values are dirty
		child.dirty_decendents();
		child.borrow_mut().parent = Some((self.clone(), child_num)); //Rc clone
		child.dirty_ancestors();
		self.borrow_mut().children[child_num] = child;
	}

	fn push_child(&self, child: Tree<T>) {
		//assume all new values are dirty
		child.dirty_decendents();
		child.borrow_mut().parent = Some((self.clone(), self.borrow().children.len())); //Rc clone
		child.dirty_ancestors();
		self.borrow_mut().children.push(child);
	}

	fn pop_child(&self) -> Option<Tree<T>> {
		let child = self.borrow_mut().children.pop();
		if let Some(ref child) = child {
			self.borrow_mut().dirty_val = true;
			self.dirty_ancestors();
			child.borrow_mut().parent = None;
		}
		child
	}
}

fn dump<T: Clone>(tree: &Tree<T>, vec: &mut PermutedVec<T>) -> usize {
	let mut tree = tree.borrow_mut();
	let mut size = 1;
	vec.push(tree.data.clone());
	tree.dirty_val = false;
	for kid in &tree.children { size += dump(&kid, vec); }
	tree.dirty_descendant = false;
	tree.old_size = size;
	size
}

fn update<T: Clone>(tree: &Tree<T>, start_index: usize, vec: &mut PermutedVec<T>) -> usize {
	let mut tree = tree.borrow_mut();	
	let mut size = tree.old_size;	
	if tree.dirty_descendant || vec.len() < start_index + size {
		if vec.len() <= start_index {
			vec.push(tree.data.clone());
		} else if tree.dirty_val {
			vec.set(start_index, tree.data.clone());
		}
		tree.dirty_val = false;
		size = 1;
		for kid in &tree.children { size += update(&kid, start_index + size, vec); }
		tree.dirty_descendant = false;
		if tree.old_size != size {
			vec.truncate(start_index + size);
			tree.old_size = size;
		}
	}
	size
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
