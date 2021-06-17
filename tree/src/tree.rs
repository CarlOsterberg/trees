use std::fmt;
use std::cmp;

#[derive(Debug,PartialEq,Clone)]
pub enum Tree {
    Null,
    Node(i32,Box<Tree>,Box<Tree>)
}

impl Tree {
    pub fn mktree() -> Tree {
        Tree::Null
    }
    pub fn insert(self, new_node:i32) -> Tree {
        match self {
            Tree::Null => Tree::Node(new_node,Box::new(Tree::Null),Box::new(Tree::Null)),
            Tree::Node(current,left,right) => {
                if new_node < current {
                    Tree::Node(current,Box::new(left.insert(new_node)),right).balance()
                }
                else {
                    Tree::Node(current,left,Box::new(right.insert(new_node))).balance()
                }
            }

        }
    }
    pub fn delete(self, delete_node:i32) -> Result<Tree,String> {
        match self {
            Tree::Node(current,left,right) => {
                if delete_node == current {
                    Ok(left.join(*right).balance())
                }
                else if delete_node<current {
                    Ok(Tree::Node(current,Box::new(left.delete(delete_node).unwrap()),right).balance())
                }
                else {
                    Ok(Tree::Node(current,left,Box::new(right.delete(delete_node).unwrap())).balance())
                }
            }
            Tree::Null => Err("Node doesnt exist".to_string())
        }
    }
    fn join(self, tree:Tree) -> Tree {
        match self.clone() {
            Tree::Node(value, left, right) => {
                if tree.clone().get_value().unwrap() > value {
                    Tree::Node(value, left, Box::new(right.join(tree)))
                }
                else {
                    Tree::Node(value,Box::new(left.join(tree)),right)
                }
            }
            Tree::Null => tree
        }
    }
    fn left_shift(self) -> Tree {
        match self {
            Tree::Node(a,alpha,right) => {
                let b = right.clone().get_value();
                if b.is_none() {
                    Tree::Node(a,alpha,right)
                }
                else {
                    let gamma = Box::new(right.clone().get_right().unwrap());
                    let beta = Box::new(right.clone().get_left().unwrap());
                    Tree::Node(b.unwrap(), Box::new(Tree::Node(a,alpha,beta)),gamma)   
                }
            },
            Tree::Null => Tree::Null,
        }
    }
    fn right_shift(self) -> Tree {
        match self {
            Tree::Node(b,left,gamma) => {
                let a = left.clone().get_value();
                if a.is_none() {
                    Tree::Node(b,left,gamma)
                }
                else {
                    let alpha = Box::new(left.clone().get_left().unwrap());
                    let beta = Box::new(left.clone().get_right().unwrap());
                    Tree::Node(a.unwrap(), alpha,Box::new(Tree::Node(b,beta,gamma)))   
                }
            },
            Tree::Null => Tree::Null,
        }
    }
    pub fn is_balanced(self) -> bool {
        match self {
            Tree::Node(_,left,right) => 
                ((left.depth()-right.depth()).abs() <= 1),
            Tree::Null => true
        }
    }
    fn balance(self) -> Tree {
        if self.clone().is_balanced() {
            self
        }
        else {
            match self.clone() {
                Tree::Node(a,left,right) => {
                    if left.clone().depth() > right.clone().depth() {
                        Tree::Node(a,left,right).right_shift().balance()
                    }
                    else {
                        Tree::Node(a,left,right).left_shift().balance()
                    }
                }
                Tree::Null => Tree::Null
            }
        }
    }
    fn get_left(self) -> Option<Tree> {
        match self {
            Tree::Node(_,left,_) => Some(*left),
            Tree::Null => None
        }
    }
    fn get_right(self) -> Option<Tree> {
        match self {
            Tree::Node(_,_,right) => Some(*right),
            Tree::Null => None
        }
    }
    fn depth(self) -> i32 {
        match self {
            Tree::Null => 0,
            Tree::Node(_,left,right) => 1 + cmp::max(left.depth(), right.depth())
        }
    }
    fn get_value(self) -> Option<i32> {
        match self {
            Tree::Node(value,_,_) => Some(value),
            Tree::Null => None
        }
    }
    pub fn inorder_walk(self) -> Vec<i32> {
        match self {
            Tree::Node(value,left,right) => {
                let mut x = left.inorder_walk();
                x.push(value);
                x.append(&mut right.inorder_walk());
                x
            }
            Tree::Null => {
                vec![]
            }
        }
    }
    pub fn is_binary_tree(self) -> bool {
        match self {
            Tree::Node(value,left,right) => {
                let l = left.clone().get_value();
                let r = right.clone().get_value();
                if l.is_some() && r.is_some() {
                    l.unwrap() <= value && value <= r.unwrap() && left.is_binary_tree() && right.is_binary_tree()   
                }
                else if l.is_some() && r.is_none() {
                    l.unwrap() <= value && left.is_binary_tree() && right.is_binary_tree()  
                }
                else if l.is_none() && r.is_some(){
                    value <= r.unwrap() && left.is_binary_tree() && right.is_binary_tree()  
                }
                else {
                    left.is_binary_tree() && right.is_binary_tree() 
                }
            }
            Tree::Null => true
        }
    }
}
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tree::Node(value,left,right) => write!(f, "({}{}{})", value, left, right)?,
            Tree::Null => ()
        }
        Ok(())
    }
}