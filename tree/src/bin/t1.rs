use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn traverse_recursive(root: &Rc<RefCell<TreeNode>>, path: String) {
    if let Some(ref left) = root.borrow().left {
        traverse_recursive(left, path.clone() + "L");
    }

    println!("{}: {}", root.borrow().val, path);

    if let Some(ref right) = root.borrow().right {
        traverse_recursive(right, path + "R");
    }
}

fn main() {
    // Create Tree:
    //           1
    //          / \
    //         2   3
    //        /   / \
    //       4   2   4
    //          /
    //         4
    //

    let mut root = TreeNode::new(1);
    let mut l = TreeNode::new(2);
    let ll = TreeNode::new(4);
    let mut r = TreeNode::new(3);
    let rll = TreeNode::new(4);
    let mut rl = TreeNode::new(2);
    let rr = TreeNode::new(4);

    l.left = Some(Rc::new(RefCell::new(ll)));
    rl.left = Some(Rc::new(RefCell::new(rll)));
    r.left = Some(Rc::new(RefCell::new(rl)));
    r.right = Some(Rc::new(RefCell::new(rr)));
    root.left = Some(Rc::new(RefCell::new(l)));
    root.right = Some(Rc::new(RefCell::new(r)));

    traverse_recursive(&Rc::new(RefCell::new(root)), "".to_string());
}
