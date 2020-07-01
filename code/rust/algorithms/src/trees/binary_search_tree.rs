pub struct BinarySearchTree<T> where T: Ord+Copy {
    value: T,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> BinarySearchTree<T> where T: Ord+Copy {
    pub fn new_from_value(value: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn new_from_slice(sl: &[T]) -> Option<BinarySearchTree<T>> {
        if sl.len() == 0 {
            return None;
        }

        let mut root = BinarySearchTree {
            value: sl[0],
            left: None,
            right: None,
        };

        for i in sl.iter().skip(1) {
            root.insert(*i)
        };

        Some(root)
    }

    pub fn insert(&mut self, value: T) {
        let tree = BinarySearchTree {
            value,
            left: None,
            right: None,
        };

        self._insert(tree)
    }

    pub fn find(&self, value: T) -> Option<&BinarySearchTree<T>> {
        if self.value == value {
            return Some(self)
        } else if value < self.value {
            if let Some(left) = &self.left {
                return left.find(value);
            } else {
                return None;
            }
        } else {
            if let Some(right) = &self.right {
                return right.find(value);
            } else {
                return None;
            }
        }
    }

    pub fn infix_traverse(&self) -> Vec<T> {
        let mut result = Vec::new();
        self._infix_traverse(&self, &mut result);
        result
    }

    pub fn prefix_traverse(&self) -> Vec<T> {
        let mut result = Vec::new();
        self._prefix_traverse(&self, &mut result);
        result
    }

    pub fn postfix_traverse(&self) -> Vec<T> {
        let mut result = Vec::new();
        self._postfix_traverse(&self, &mut result);
        result
    }

    fn _infix_traverse(&self, tree: &BinarySearchTree<T>, result: &mut Vec<T>) {
        if let Some(left) = &tree.left {
            self._infix_traverse(left, result)
        }

        result.push(tree.value);

        if let Some(right) = &tree.right {
            self._infix_traverse(right, result)
        }
    }

    fn _prefix_traverse(&self, tree: &BinarySearchTree<T>, result: &mut Vec<T>) {
        result.push(tree.value);

        if let Some(left) = &tree.left {
            self._prefix_traverse(left, result)
        }

        if let Some(right) = &tree.right {
            self._prefix_traverse(right, result)
        }
    }

    fn _postfix_traverse(&self, tree: &BinarySearchTree<T>, result: &mut Vec<T>) {
        if let Some(left) = &tree.left {
            self._postfix_traverse(left, result)
        }

        if let Some(right) = &tree.right {
            self._postfix_traverse(right, result)
        }

        result.push(tree.value);
    }

    fn _insert(&mut self, tree: BinarySearchTree<T>) {
        if tree.value < self.value {
            if let Some(left) = &mut self.left {
                left._insert(tree)
            } else {
                self.left = Some(Box::new(tree));
            }
        } else {
            if let Some(right) = &mut self.right {
                right._insert(tree)
            } else {
                self.right = Some(Box::new(tree));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn binary_tree_traverses_test() {
        let arr = vec![100,200,300,150,20,30,10];
        let tree = BinarySearchTree::new_from_slice(&arr).unwrap();

        let infix = tree.infix_traverse();
        let prefix = tree.prefix_traverse();
        let postfix = tree.postfix_traverse();

        assert_eq!(infix, vec![10,20,30,100,150,200,300]);
        assert_eq!(prefix, vec![100,20,10,30,200,150,300]);
        assert_eq!(postfix, vec![10,30,20,150,300,200,100]);
    }

    #[test]
    fn tree_find_test() {
        let arr = vec![100,200,300,150,20,30,10];
        let tree = BinarySearchTree::new_from_slice(&arr).unwrap();

        let finded = tree.find(100);
        let not_finded = tree.find(12);

        assert_eq!(finded.unwrap().value, 100);
        assert_eq!(not_finded.is_some(), false);
    }
}