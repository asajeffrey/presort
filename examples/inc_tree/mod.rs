// TODO: use Weak<> pointers to avoid memory leaks
extern crate presort;

pub mod sortvec;

use std::rc::Rc;
use std::cell::RefCell; 
use inc_tree::sortvec::SortVec;

pub struct TreeNode<T> {
    // whether this node's data has changed
    dirty_val: bool,
    // whether this node or decendents have changed
    needs_update: bool,
    // the index into the output vec
    vec_index: usize,
    // parent and index in parent's child vec
    parent: Option<(Tree<T>, usize)>,
    // this node's data
    data: T,
    children: Vec<Tree<T>>,
}

pub type Tree<T> = Rc<RefCell<TreeNode<T>>>;

pub trait IncTree<T: PartialEq + Clone> {
    fn new_node(data: T) -> Tree<T>;
    fn flag_as_new(&self);
    fn flag_as_updated(&self);
    fn get_data(&self) -> T;
    fn set_data(&self, data: T);
    fn get_child(&self, child_num: usize) -> Tree<T>;
    fn set_child(&self, child_num: usize, child: Tree<T>);
    fn push_child(&self, child: Tree<T>);
    fn pop_child(&self) -> Option<Tree<T>>;
    fn remove_child(&self, child: usize);
    fn num_children(&self) -> usize;
    fn total_nodes(&self) -> usize;
}

impl<T: PartialEq + Clone> IncTree<T> for Tree<T> {
    fn new_node(data: T) -> Tree<T>{
        Rc::new(RefCell::new(TreeNode {
            dirty_val: true,
            needs_update: true,
            vec_index: 0,
            parent: None,
            data: data,
            children: vec![]
        }))
    }

    //sets dirty flags on all children
    fn flag_as_new(&self) {
        let mut tree = self.borrow_mut();
        tree.dirty_val = true;
        tree.needs_update = true;
        for kid in &tree.children {
            kid.flag_as_new();
        }
    }

    //sets dirty flags on self and parents
    fn flag_as_updated(&self){
            let mut node = self.borrow_mut();
            node.needs_update = true;
            if let Some((ref parent, _)) = node.parent {
                parent.flag_as_updated();
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
        self.flag_as_updated();
    }

    fn get_child(&self, child_num: usize) -> Tree<T> {
        self.borrow().children[child_num].clone() //Rc clone
    }

    fn set_child(&self, child_num: usize, child: Tree<T>) {
        child.flag_as_new();
        child.borrow_mut().parent = Some((self.clone(), child_num)); //Rc clone
        child.flag_as_updated();
        self.borrow_mut().children[child_num] = child;
    }

    fn push_child(&self, child: Tree<T>) {
        child.flag_as_new();
        child.borrow_mut().parent = Some((self.clone(), self.borrow().children.len())); //Rc clone
        child.flag_as_updated();
        self.borrow_mut().children.push(child);
    }

    fn pop_child(&self) -> Option<Tree<T>> {
        let child = self.borrow_mut().children.pop();
        if let Some(ref child) = child {
            self.flag_as_updated();
            child.borrow_mut().parent = None;
        }
        child
    }

    fn remove_child(&self, child: usize) {
        {
            let tree = self.borrow_mut();
            let mut child = tree.children[child].borrow_mut();
            child.parent = None;
        }
        self.flag_as_updated();
        self.borrow_mut().children.remove(child);
    }

    fn num_children(&self) -> usize {
        self.borrow().children.len()
    }

    fn total_nodes(&self) -> usize {
        let mut cnt = 1;
        for child in &self.borrow().children {
            cnt += child.total_nodes();
        }
        cnt
    }

}

// writes the data in tree to vec, starting at free_index,
// assuming the vec.len() is free_index,
// returning the final length of the vec
pub fn dump<T: Ord+Clone, V: SortVec<T>>(tree: &Tree<T>, free_index: usize, vec: &mut V) -> usize {
    let mut tree = tree.borrow_mut();
    tree.vec_index = free_index;
    vec.push(tree.data.clone());
    tree.dirty_val = false;
    let mut next_free_index = free_index + 1;
    for kid in &tree.children { next_free_index = dump(&kid, next_free_index, vec); }
    tree.needs_update = false;
    next_free_index
}

// Updates vec with data in tree,
// assuming prior tree data is stored there,
// and assuming vec.len() is at least free_index,
// returning the next free index (= length of vec at data end).
// Truncating vec to the return value is suggested at top level.
// TODO: Pad final values if data shrinks
pub fn update<T: Ord+Clone+Default, V: SortVec<T>>(tree: &Tree<T>, free_index: usize, vec: &mut V) -> usize {
    let mut tree = tree.borrow_mut();
    // move if there's no space
    if tree.vec_index < free_index {
        tree.vec_index = free_index;
        tree.needs_update = true;
        tree.dirty_val = true;
    } else if tree.vec_index > free_index {
        // extend with padding if there's extra space
        // TODO: consider limiting this padding
        for i in free_index..tree.vec_index {vec.set(i, T::default())}
    }
    let mut next_free_index = tree.vec_index + 1;
    if tree.needs_update {
        // update value
        if vec.len() == tree.vec_index {
            vec.push(tree.data.clone())
        } else if tree.dirty_val {
            vec.set(tree.vec_index, tree.data.clone())
        }
        tree.dirty_val = false;
        // update kids
        for kid in &tree.children {
            next_free_index = update(&kid, next_free_index, vec);
        }
        tree.needs_update = false;
    } else {
        // find last index
        let num_children = tree.children.len();
        if num_children > 0 {
            next_free_index = tree.children[num_children - 1].borrow().vec_index + 1;
        }
    }
    next_free_index
}

// similar to update, but compacts the data and truncates the vec
pub fn update_no_pad<T: Ord+Clone, V: SortVec<T>>(tree: &Tree<T>, free_index: usize, vec: &mut V) -> usize {
    let length = update_no_pad_internal(tree, free_index, vec);
    vec.truncate(length);
    length
}

fn update_no_pad_internal<T: Ord+Clone, V: SortVec<T>>(tree: &Tree<T>, free_index: usize, vec: &mut V) -> usize {
    let mut tree = tree.borrow_mut();
    // move if wrong place
    if tree.vec_index != free_index {
        tree.vec_index = free_index;
        tree.needs_update = true;
        tree.dirty_val = true;
    }
    let mut next_free_index = tree.vec_index + 1;
    if tree.needs_update {
        // update value
        if vec.len() == tree.vec_index {
            vec.push(tree.data.clone())
        } else if tree.dirty_val {
            vec.set(tree.vec_index, tree.data.clone())
        }
        tree.dirty_val = false;
        // update kids
        for kid in &tree.children {
            next_free_index = update_no_pad_internal(&kid, next_free_index, vec);
        }
        tree.needs_update = false;
    } else {
        // find last index
        let num_children = tree.children.len();
        if num_children > 0 {
            next_free_index = tree.children[num_children - 1].borrow().vec_index + 1;
        }
    }
    next_free_index
}
