use std::cmp::Ordering;

/// This struct implements a Binary Search Tree (BST), which is a
/// simple data structure for storing sorted data
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    /// Create a new, empty BST
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    /// Find a value in this tree. Returns true if value is in this
    /// tree, and false otherwise
    pub fn search(&self, value: T) -> bool {
        match &self.value {
            Some(key) => match key.cmp(&value) {
                Ordering::Equal => true, // key == value
                Ordering::Greater => {
                    // key > value
                    match &self.left {
                        Some(node) => node.search(value),
                        None => false,
                    }
                }
                Ordering::Less => {
                    // key < value
                    match &self.right {
                        Some(node) => node.search(value),
                        None => false,
                    }
                }
            },
            None => false,
        }
    }

    /// Insert a value into the appropriate location in this tree.
    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            match &self.value {
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else if value > *key{
                        &mut self.right
                    } else {
                        return;
                    };
                    match target_node {
                        Some(ref mut node) => {
                            node.insert(value);
                        }
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
                None => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.value {
            Some(ref _node) => {
                assert!(bst.left.is_none());
                assert!(bst.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
