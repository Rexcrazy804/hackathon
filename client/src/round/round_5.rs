// Binary Search Tree Borders
/*
* each line contains an input which you are to use to generate a simple binary search tree, with
* the first number as the tree's root. Once the tree has been established, you must find sum of
* values of border (or perimetre) elements of the binary tree and find its product of digits.
*
* do this for each line and return the grand sum
*/
mod tree {
    use std::{cell::RefCell, rc::Rc};

    pub enum Child {
        Left,
        Right,
    }

    #[derive(Debug, Clone)]
    pub struct TreeCell {
        pub val: u32,
        pub left: Option<TreeCellRef>,
        pub right: Option<TreeCellRef>,
    }

    impl TreeCell {
        pub fn new(val: u32) -> Self {
            Self {
                val,
                left: None,
                right: None,
            }
        }
        pub fn attach(&mut self, node: TreeCellRef, child: Child) {
            match child {
                Child::Left => self.left = Some(node.clone()),
                Child::Right => self.right = Some(node.clone()),
            }
        }
        pub fn getrefcell(&self) -> Option<TreeCellRef> {
            Some(Rc::new(RefCell::new(self.clone())))
        }
    }

    pub type TreeCellRef = Rc<RefCell<TreeCell>>;

}

use tree::{
    TreeCellRef,
    Child,
    TreeCell,
};


pub(super) fn compute(input_string: &str) -> u32 {
    let mut grand_sum = 0;
    for line in input_string.lines() {
        let nums = line
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let root = TreeCell::new(*nums.first().unwrap()).getrefcell();

        for num in nums.iter().skip(1) {
            bind_to_tree(root.as_ref(), *num);
        }

        // loner nodes are nodes with less than 2 children :D
        let mut perimetre_nodes = Vec::new();
        get_loner_nodes(root.as_ref(), &mut perimetre_nodes);
        get_umbrella_nodes(root.as_ref(), &mut perimetre_nodes);

        let sum_of_values = perimetre_nodes
            .iter()
            .sum::<u32>();

        grand_sum += sum_of_values
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .product::<u32>();
    }
    grand_sum
}

fn bind_to_tree(root: Option<&TreeCellRef>, num: u32) {
    let mut root = root.unwrap().borrow_mut();
    if root.val < num {
        if root.right.is_none() {
            root.attach(TreeCell::new(num).getrefcell().unwrap(), Child::Right)
        } else {
            bind_to_tree(root.right.as_ref(), num)
        }
    } else if root.left.is_none() {
        root.attach(TreeCell::new(num).getrefcell().unwrap(), Child::Left)
    } else {
        bind_to_tree(root.left.as_ref(), num)
    }
}

fn get_loner_nodes(root: Option<&TreeCellRef>, list: &mut Vec<u32>) {
    let Some(root) = root else { return };
    let root = root.borrow();

    let (left, right) = (root.left.as_ref(), root.right.as_ref());
    match (left, right) {
        (Some(_), Some(_)) => {
            get_loner_nodes(left, list);
            get_loner_nodes(right, list);
        },

        (Some(_), None) => {
            list.push(root.val);
            get_loner_nodes(left, list);
        },

        (None, Some(_)) => {
            list.push(root.val);
            get_loner_nodes(right, list);
        },

        (None, None) => list.push(root.val),
    }
}

fn get_umbrella_nodes(root: Option<&TreeCellRef>, list: &mut Vec<u32>) {
    let Some(root) = root else { return };
    let root = root.borrow();
    let root_val = root.val;

    if !list.contains(&root_val) {
        list.push(root.val);
    }

    get_umbrella_nodes(root.left.as_ref(), list);
    get_umbrella_nodes(root.right.as_ref(), list);
}
