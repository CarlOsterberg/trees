use crate::tree::Tree;

mod tree;

fn main() {
    let mut tree = Tree::mktree();
    tree = tree.insert(1);
    tree = tree.insert(2);
    tree = tree.insert(3);
    tree = tree.insert(4);
    tree = tree.insert(5);
    tree = tree.insert(6);
    tree = tree.delete(4).unwrap();
    tree = tree.delete(1).unwrap();
    println!("{}", tree);
    println!("{:?}", tree.is_binary_tree());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut tree = Tree::mktree();
        tree = tree.insert(1);
        tree = tree.insert(2);
        tree = tree.insert(3);
        tree = tree.insert(4);
        tree = tree.insert(5);
        tree = tree.insert(6);
        assert!(tree.clone().is_balanced() && tree.is_binary_tree())
    }

    #[test]
    fn delete() {
        let mut tree = Tree::mktree();
        tree = tree.insert(1);
        tree = tree.insert(2);
        tree = tree.insert(3);
        tree = tree.insert(4);
        tree = tree.insert(5);
        tree = tree.insert(6);
        tree = tree.delete(4).unwrap();
        tree = tree.delete(1).unwrap();
        assert!(tree.clone().is_balanced() && tree.is_binary_tree())
    }

    #[test]
    fn inorder_walk() {
        let mut tree = Tree::mktree();
        tree = tree.insert(11);
        tree = tree.insert(2);
        tree = tree.insert(33);
        tree = tree.insert(42);
        tree = tree.insert(50);
        tree = tree.insert(63);
        tree = tree.insert(13);
        assert_eq!(tree.inorder_walk(),[2,11,13,33,42,50,63])
    }
}
