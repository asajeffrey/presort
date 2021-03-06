// testing for mod inc_tree

extern crate presort;

mod inc_tree;

//use presort::{PresortedVec, PermutedVec};
use inc_tree::{Tree, IncTree, dump, update, update_no_pad};
use inc_tree::sortvec::IntoSortedIterator;

fn main(){
    test_tree();
    test_update();
}

// #[test]
fn test_tree() {
    let main_tree = Tree::new_node(37);
    let mut vec = Vec::new();
    dump(&main_tree, 0, &mut vec);
    assert_eq!(vec.into_sorted_iter().collect::<Vec<&usize>>(), vec![&37]);
}

// #[test]
fn test_update() {

// Structure for this test, children under node
// 37
// 42   20->25   63    
//      1        47  53  61->77  57
//                       100
//                       101->105

    println!("start first dump");
    let main_tree = Tree::new_node(37);
    main_tree.push_child(Tree::new_node(42));
    main_tree.push_child(Tree::new_node(20));
    main_tree.push_child(Tree::new_node(63));
    let mut vec = Vec::new();
    dump(&main_tree, 0, &mut vec);
    println!("finished first dump");
    assert_eq!(vec.into_sorted_iter().collect::<Vec<&usize>>(), vec![&20,&37,&42,&63]);
    println!("success");

    println!("start first update");
    main_tree.get_child(1).set_data(25);
    update(&main_tree, 0, &mut vec);
    println!("finished first update");
    assert_eq!(vec.into_sorted_iter().collect::<Vec<&usize>>(), vec![&25,&37,&42,&63]);
    println!("success");

    println!("start add branch");
    let branch = main_tree.get_child(2);
    branch.push_child(Tree::new_node(47));
    branch.push_child(Tree::new_node(53));
    branch.push_child(Tree::new_node(61));
    branch.push_child(Tree::new_node(57));
    update(&main_tree, 0, &mut vec);
    println!("finished add branch");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&25,&37,&42,&47,&53,&57,&61,&63]
    );
    println!("success");

    println!("start update branch");
    branch.get_child(2).set_data(77);
    update(&main_tree, 0, &mut vec);
    println!("finished update branch");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&25,&37,&42,&47,&53,&57,&63,&77]
    );
    println!("success");

    println!("start early insertion");
    let branch = main_tree.get_child(1);
    branch.push_child(Tree::new_node(1));
    update(&main_tree, 0, &mut vec);
    println!("finished early insertion");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&1,&25,&37,&42,&47,&53,&57,&63,&77]
    );
    println!("success");

    println!("start deep add");
    let branch = main_tree.get_child(2).get_child(2);
    let new_branch = Tree::new_node(100);
    new_branch.push_child(Tree::new_node(101));
    branch.push_child(new_branch);
    update(&main_tree, 0, &mut vec);
    println!("finished deep add");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&1,&25,&37,&42,&47,&53,&57,&63,&77,&100,&101]
    );
    println!("success");

    println!("start deep edit");
    let branch = branch.get_child(0).get_child(0);
    branch.set_data(105);
    update(&main_tree, 0, &mut vec);
    println!("finished deep edit");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&1,&25,&37,&42,&47,&53,&57,&63,&77,&100,&105]
    );
    println!("success");

    println!("start pop branch");
    let branch = main_tree.get_child(2).get_child(2);
    branch.pop_child();
    update(&main_tree, 0, &mut vec);
    println!("finished pop branch");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&0,&0,&1,&25,&37,&42,&47,&53,&57,&63,&77]
    );
    println!("success");

    println!("start remove branch");
    main_tree.remove_child(1);
    update_no_pad(&main_tree, 0, &mut vec);
    println!("finished remove branch");
    assert_eq!(
        vec.into_sorted_iter().collect::<Vec<&usize>>(),
        vec![&37,&42,&47,&53,&57,&63,&77]
    );
    println!("success");
}
