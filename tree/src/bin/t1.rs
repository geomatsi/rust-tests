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

fn traverse_recursive(root: &Rc<RefCell<TreeNode>>, path: String) -> Vec<String> {
    let mut lp: Vec<String> = vec![];
    let mut rp: Vec<String> = vec![];

    if let Some(ref left) = root.borrow().left {
        lp = traverse_recursive(left, path.clone() + "L");
    }

    if let Some(ref right) = root.borrow().right {
        rp = traverse_recursive(right, path.clone() + "R");
    }

    [lp, vec![format!("{}:{}", root.borrow().val, path)], rp].concat()
}

fn subtrees_recursive(root: &Rc<RefCell<TreeNode>>) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut ms: Vec<Vec<String>> = vec![];

    // left: (deeper subtrees, top subtrees)
    let (lo, lt) = match root.borrow().left {
        Some(ref left) => subtrees_recursive(left),
        None => (vec![], vec![]),
    };

    // right: (deeper subtrees, top subtrees)
    let (ro, rt) = match root.borrow().right {
        Some(ref right) => subtrees_recursive(right),
        None => (vec![], vec![]),
    };

    for e in lt.iter() {
        let mut x: Vec<String> = vec![];

        for m in e.iter() {
            x.push(m.clone() + "L");
        }

        x.push(format!("{}:", root.borrow().val));
        ms.push(x);
    }

    for e in rt.iter() {
        let mut x: Vec<String> = vec![];

        for m in e.iter() {
            x.push(m.clone() + "R");
        }

        x.push(format!("{}:", root.borrow().val));
        ms.push(x);
    }

    if ms.is_empty() {
        ms.push(vec![format!("{}:", root.borrow().val)]);
    }

    // return: (deeper subtrees, top subtrees)
    ([lo, lt, ro, rt].concat(), ms)
}

fn main() {
    // Create Tree:
    //           1
    //          / \
    //         2   3
    //        /   / \
    //       4   2   4
    //          / \
    //         4   7
    //

    let mut root = TreeNode::new(1);
    let mut l = TreeNode::new(2);
    let ll = TreeNode::new(4);
    let mut r = TreeNode::new(3);
    let rll = TreeNode::new(4);
    let rlr = TreeNode::new(7);
    let mut rl = TreeNode::new(2);
    let rr = TreeNode::new(4);

    l.left = Some(Rc::new(RefCell::new(ll)));
    rl.left = Some(Rc::new(RefCell::new(rll)));
    rl.right = Some(Rc::new(RefCell::new(rlr)));
    r.left = Some(Rc::new(RefCell::new(rl)));
    r.right = Some(Rc::new(RefCell::new(rr)));
    root.left = Some(Rc::new(RefCell::new(l)));
    root.right = Some(Rc::new(RefCell::new(r)));

    let xroot = Rc::new(RefCell::new(root));

    let s = traverse_recursive(&xroot, "".to_string());
    println!("{:?}", s);

    let s = traverse_recursive(&Rc::clone(&xroot), "".to_string());
    println!("{:?}", s);

    let s = subtrees_recursive(&Rc::clone(&xroot));
    println!("{:?}", s);
}
