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

fn subtrees_recursive(root: &Rc<RefCell<TreeNode>>) -> Vec<(Vec<String>, Rc<RefCell<TreeNode>>)> {
    let mut sub: Vec<String> = vec![];

    let lt = match root.borrow().left {
        Some(ref left) => subtrees_recursive(left),
        None => vec![],
    };

    if let Some(e) = lt.last() {
        for m in e.0.iter() {
            sub.push(m.clone() + "L");
        }
    }

    sub.push(format!("{}:", root.borrow().val));

    let rt = match root.borrow().right {
        Some(ref right) => subtrees_recursive(right),
        None => vec![],
    };

    if let Some(e) = rt.last() {
        for m in e.0.iter() {
            sub.push(m.clone() + "R");
        }
    }

    // return: last is always top subtree
    [lt, rt, vec![(sub, Rc::clone(&root))]].concat()
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

    let root = TreeNode::new(1);
    let proot = Rc::new(RefCell::new(root));

    let l = TreeNode::new(2);
    let pl = Rc::new(RefCell::new(l));

    let ll = TreeNode::new(4);
    let pll = Rc::new(RefCell::new(ll));

    let r = TreeNode::new(3);
    let pr = Rc::new(RefCell::new(r));

    let rll = TreeNode::new(4);
    let prll = Rc::new(RefCell::new(rll));

    let rlr = TreeNode::new(7);
    let prlr = Rc::new(RefCell::new(rlr));

    let rl = TreeNode::new(2);
    let prl = Rc::new(RefCell::new(rl));

    let rr = TreeNode::new(4);
    let prr = Rc::new(RefCell::new(rr));

    pl.borrow_mut().left = Some(Rc::clone(&pll));
    prl.borrow_mut().left = Some(Rc::clone(&prll));
    prl.borrow_mut().right = Some(Rc::clone(&prlr));
    pr.borrow_mut().left = Some(Rc::clone(&prl));
    pr.borrow_mut().right = Some(Rc::clone(&prr));
    proot.borrow_mut().left = Some(Rc::clone(&pl));
    proot.borrow_mut().right = Some(Rc::clone(&pr));

    let s = traverse_recursive(&proot, "".to_string());
    println!("Traverse: {:?}", s);

    let s = traverse_recursive(&proot, "".to_string());
    println!("Traverse: {:?}", s);

    let s = subtrees_recursive(&proot);
    for e in s.iter() {
        println!("{:?} <- {}", e.0, e.1.as_ref().borrow().val);
    }

    // Create Tree:
    //           0
    //          / \
    //         1   3
    //        /     \
    //       2       4
    //              / \
    //             5   6
    //

    let root = TreeNode::new(0);
    let proot = Rc::new(RefCell::new(root));

    let l = TreeNode::new(1);
    let pl = Rc::new(RefCell::new(l));

    let ll = TreeNode::new(2);
    let pll = Rc::new(RefCell::new(ll));

    let r = TreeNode::new(3);
    let pr = Rc::new(RefCell::new(r));

    let rr = TreeNode::new(4);
    let prr = Rc::new(RefCell::new(rr));

    let rrl = TreeNode::new(5);
    let prrl = Rc::new(RefCell::new(rrl));

    let rrr = TreeNode::new(6);
    let prrr = Rc::new(RefCell::new(rrr));

    proot.borrow_mut().left = Some(Rc::clone(&pl));
    proot.borrow_mut().right = Some(Rc::clone(&pr));
    pl.borrow_mut().left = Some(Rc::clone(&pll));
    pr.borrow_mut().right = Some(Rc::clone(&prr));
    prr.borrow_mut().right = Some(Rc::clone(&prrr));
    prr.borrow_mut().left = Some(Rc::clone(&prrl));

    let s = traverse_recursive(&proot, "".to_string());
    println!("Traverse: {:?}", s);

    let s = traverse_recursive(&proot, "".to_string());
    println!("Traverse: {:?}", s);

    let s = subtrees_recursive(&proot);
    for e in s.iter() {
        println!("{:?} <- {}", e.0, e.1.as_ref().borrow().val);
    }

    // Create Tree:
    //           0
    //          / \
    //         0   0
    //        /     \
    //       0       0
    //                \
    //                 0
    //

    let root = TreeNode::new(0);
    let proot = Rc::new(RefCell::new(root));

    let l = TreeNode::new(0);
    let pl = Rc::new(RefCell::new(l));

    let ll = TreeNode::new(0);
    let pll = Rc::new(RefCell::new(ll));

    let r = TreeNode::new(0);
    let pr = Rc::new(RefCell::new(r));

    let rr = TreeNode::new(0);
    let prr = Rc::new(RefCell::new(rr));

    let rrr = TreeNode::new(0);
    let prrr = Rc::new(RefCell::new(rrr));

    proot.borrow_mut().left = Some(Rc::clone(&pl));
    proot.borrow_mut().right = Some(Rc::clone(&pr));
    pl.borrow_mut().left = Some(Rc::clone(&pll));
    pr.borrow_mut().right = Some(Rc::clone(&prr));
    prr.borrow_mut().right = Some(Rc::clone(&prrr));

    let s = traverse_recursive(&proot, "".to_string());
    println!("Traverse: {:?}", s);

    let s = traverse_recursive(&proot, "".to_string());
    println!("Traverse: {:?}", s);

    let s = subtrees_recursive(&proot);
    for e in s.iter() {
        println!("{:?} <- {}", e.0, e.1.as_ref().borrow().val);
    }
}
