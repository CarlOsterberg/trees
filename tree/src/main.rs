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
    println!("{}", tree);
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
        assert!(tree.is_balanced())
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
        assert!(tree.is_balanced())
    }
}
