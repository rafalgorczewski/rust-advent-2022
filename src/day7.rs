use super::utilities::load_file;

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::thread::current;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  parent: Option<Rc<RefCell<TreeNode>>>,
  children: Vec<Rc<RefCell<TreeNode>>>,
  size: Option<i32>,
}

impl TreeNode {
  pub fn new() -> Self {
    TreeNode {
      parent: None,
      children: Vec::new(),
      size: None,
    }
  }

  pub fn new_rc() -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new()))
  }
}

struct Tree {
  root: Rc<RefCell<TreeNode>>,
  dirs: Vec<Rc<RefCell<TreeNode>>>,
}

fn create_tree(filename: &str) -> Tree {
  let lines = load_file(filename);
  let mut root = TreeNode::new_rc();
  let mut current_node = root.clone();
  let mut dirs = Vec::new();
  dirs.push(root.clone());
  for (i, line) in lines.iter().enumerate() {
    if line == "$ cd /" {
      continue;
    }
    if line == "$ cd .." {
      let parent = current_node.borrow().parent.clone().unwrap();
      current_node = parent;
      continue;
    }
    if line.contains("$ cd ") {
      let mut new_node = TreeNode::new_rc();
      new_node.borrow_mut().parent = Some(current_node.clone());
      dirs.push(new_node.clone());
      current_node.borrow_mut().children.push(new_node.clone());
      current_node = new_node;
      continue;
    }
    if line == "$ ls" {
      continue;
    }
    if line.contains("dir ") {
      continue;
    }
    if !line.is_empty() {
      let size = line.split_whitespace().collect::<Vec<_>>()[0]
        .parse::<i32>()
        .unwrap();
      let mut new_node = TreeNode::new_rc();
      new_node.borrow_mut().size = Some(size);
      current_node.borrow_mut().children.push(new_node);
    }
  }
  Tree { root, dirs }
}

fn get_tree_size(root: Rc<RefCell<TreeNode>>) -> i32 {
  if root.borrow().children.is_empty() {
    if let Some(size) = root.borrow().size {
      return size;
    } else {
      return 0;
    }
  }
  root
    .borrow()
    .children
    .iter()
    .map(|x| get_tree_size(x.clone()))
    .sum::<i32>()
}

pub fn first() -> String {
  let dirs = create_tree("inputs/day7.txt").dirs;
  dirs
    .into_iter()
    .map(get_tree_size)
    .filter(|x| *x <= 100000)
    .sum::<i32>()
    .to_string()
}

pub fn second() -> String {
  const TOTAL_DISK_SPACE: i32 = 70000000;
  const SPACE_NEEDED: i32 = 30000000;
  let tree = create_tree("inputs/day7.txt");
  let space_occupied = get_tree_size(tree.root);
  let free_space = TOTAL_DISK_SPACE - space_occupied;
  tree
    .dirs
    .into_iter()
    .map(get_tree_size)
    .filter(|x| free_space + *x >= SPACE_NEEDED)
    .min()
    .unwrap()
    .to_string()
}
