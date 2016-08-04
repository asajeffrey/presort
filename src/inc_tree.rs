use presorted_vec::PresortedVec;
use std::rc::Rc;
use std::cell::RefCell; 

pub struct TreeNode<T> {
    //whether this node's data has changed
    dirty_val: bool,
    //whether this node or decendents have changed
    needs_update: bool,
    //the old size of the tree
    old_size: usize,
    //parent and index in parent's child vec
    parent: Option<(Tree<T>, usize)>,
    //this node's data
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
    fn num_children(&self) -> usize;
}

impl<T: PartialEq + Clone> IncTree<T> for Tree<T> {
    fn new_node(data: T) -> Tree<T>{
        Rc::new(RefCell::new(TreeNode {
            dirty_val: true,
            needs_update: true,
            old_size: 0,
            parent: None,
            data: data,
            children: vec![]
        }))
    }

    //sets dirty flags on all children and parents
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
            self.borrow_mut().dirty_val = true;
            self.flag_as_updated();
            child.borrow_mut().parent = None;
        }
        child
    }

    fn num_children(&self) -> usize {
        self.borrow().children.len()
    }
}

pub fn dump<T: Ord+Clone>(tree: &Tree<T>, vec: &mut PresortedVec<T>) -> usize {
    let mut tree = tree.borrow_mut();
    let mut size = 1;
    vec.push(tree.data.clone());
    tree.dirty_val = false;
    for kid in &tree.children { size += dump(&kid, vec); }
    tree.needs_update = false;
    tree.old_size = size;
    size
}

pub fn update<T: Ord+Clone>(tree: &Tree<T>, start_index: usize, vec: &mut PresortedVec<T>) -> usize {
    let mut tree = tree.borrow_mut();    
    let mut size = tree.old_size;    
    if tree.needs_update || vec.len() < start_index + size {
        if vec.len() <= start_index {
            vec.push(tree.data.clone());
        } else if tree.dirty_val {
            vec.set(start_index, tree.data.clone());
        }
        tree.dirty_val = false;
        size = 1;
        for kid in &tree.children { size += update(&kid, start_index + size, vec); }
        tree.needs_update = false;
        if tree.old_size != size {
            vec.truncate(start_index + size);
            tree.old_size = size;
        }
    }
    size
}