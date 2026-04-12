use std::cmp::max;

pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
    max_path_ll: Option<u64>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
            max_path_ll: None,
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert_eq!(
                self.nodes[parent_id].id_left, None,
                "Parent node has the left child already set"
            );
        } else {
            assert_eq!(
                self.nodes[parent_id].id_right, None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }
    pub fn check_bts(&mut self, use_rec: bool) -> bool {
        if use_rec {
            return self.check_bts_rec(Some(0usize), None, None);
        }

        let mut curr = Some(0usize);
        let mut prev: Option<usize> = None;
        let mut is_bts = true;

        while let Some(curr_id) = curr {
            if let Some(left_id) = self.nodes[curr_id].id_left {
                let mut pred = Some(left_id);
                while let Some(prev_id) = pred
                    && let Some(prev_right_id) = self.nodes[prev_id].id_right
                    && prev_right_id != curr_id
                {
                    pred = Some(prev_right_id);
                }
                if let Some(prev_id) = pred
                    && let Some(_) = self.nodes[prev_id].id_right
                {
                    if self.nodes[prev_id].key >= self.nodes[curr_id].key {
                        is_bts = false;
                    }
                    self.nodes[prev_id].id_right = None;
                    curr = self.nodes[curr_id].id_right;
                    prev = Some(curr_id);
                } else if let Some(prev_id) = pred {
                    self.nodes[prev_id].id_right = curr;
                    curr = self.nodes[curr_id].id_left;
                }
            } else {
                if let Some(prev_id) = prev
                    && self.nodes[prev_id].key >= self.nodes[curr_id].key
                {
                    is_bts = false;
                }
                curr = self.nodes[curr_id].id_right;
                prev = Some(curr_id);
            }
        }
        is_bts
    }

    fn check_bts_rec(&self, node_id: Option<usize>, min: Option<u32>, max: Option<u32>) -> bool {
        if let Some(node_id) = node_id {
            let key = self.nodes[node_id].key;
            if let Some(min_) = min
                && min_ >= key
            {
                return false;
            }
            if let Some(max_) = max
                && max_ <= key
            {
                return false;
            }
            self.check_bts_rec(self.nodes[node_id].id_left, min, Some(key))
                && self.check_bts_rec(self.nodes[node_id].id_right, Some(key), max)
        } else {
            true
        }
    }

    pub fn max_path_sum(&mut self) -> Option<u64> {
        self.max_path_ll = None;
        self.max_path_sum_rec(0);
        self.max_path_ll
    }
    fn max_path_sum_rec(&mut self, node_id: usize) -> u64 {
        let curr_key = self.nodes[node_id].key as u64;

        if let Some(left_id) = self.nodes[node_id].id_left
            && let Some(right_id) = self.nodes[node_id].id_right
        {
            let left_key = self.max_path_sum_rec(left_id);
            let right_key = self.max_path_sum_rec(right_id);
            self.max_path_ll = max(
                self.max_path_ll,
                Some(left_key.saturating_add(right_key).saturating_add(curr_key)),
            );
            return max(left_key, right_key).saturating_add(curr_key);
        } else if let Some(left_id) = self.nodes[node_id].id_left {
            return curr_key.saturating_add(self.max_path_sum_rec(left_id));
        } else if let Some(right_id) = self.nodes[node_id].id_right {
            return curr_key.saturating_add(self.max_path_sum_rec(right_id));
        }

        curr_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bst_methods_agree<F>(root_key: u32, expected: bool, build: F)
    where
        F: Fn(&mut Tree),
    {
        let mut recursive_tree = Tree::with_root(root_key);
        build(&mut recursive_tree);
        let recursive = recursive_tree.check_bts(true);

        let mut morris_tree = Tree::with_root(root_key);
        build(&mut morris_tree);
        let morris = morris_tree.check_bts(false);

        assert_eq!(
            recursive, expected,
            "Recursive BST check returned an unexpected result"
        );
        assert_eq!(
            morris, expected,
            "Morris BST check returned an unexpected result"
        );
        assert_eq!(recursive, morris, "Recursive and Morris checks must agree");
    }

    #[test]
    fn test_basic_tree_sum() {
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.sum(), 10);

        let id_1 = tree.add_node(0, 5, true);
        let id_2 = tree.add_node(0, 22, false);
        assert_eq!(tree.sum(), 37);

        tree.add_node(id_1, 7, false);
        tree.add_node(id_2, 20, true);
        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_check_bts_agrees_on_single_node_tree() {
        assert_bst_methods_agree(42, true, |_| {});
    }

    #[test]
    fn test_check_bts_agrees_on_sparse_valid_tree() {
        // Structure:
        //         8
        //       /   \
        //      3     10
        //       \      \
        //        6      14
        //       /      /
        //      4      13
        assert_bst_methods_agree(8, true, |tree| {
            let left_id = tree.add_node(0, 3, true);
            let right_id = tree.add_node(0, 10, false);
            let six_id = tree.add_node(left_id, 6, false);
            tree.add_node(six_id, 4, true);
            let fourteen_id = tree.add_node(right_id, 14, false);
            tree.add_node(fourteen_id, 13, true);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_right_skewed_tree_with_boundaries() {
        assert_bst_methods_agree(0, true, |tree| {
            let id_1 = tree.add_node(0, 5, false);
            let id_2 = tree.add_node(id_1, 11, false);
            tree.add_node(id_2, u32::MAX, false);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_left_skewed_tree_with_boundaries() {
        assert_bst_methods_agree(u32::MAX, true, |tree| {
            let id_1 = tree.add_node(0, 100, true);
            let id_2 = tree.add_node(id_1, 10, true);
            tree.add_node(id_2, 0, true);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_immediate_child_violation() {
        assert_bst_methods_agree(10, false, |tree| {
            tree.add_node(0, 5, false);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_deep_right_subtree_violation() {
        // The node 7 is in the right subtree of 10, so this is invalid.
        assert_bst_methods_agree(10, false, |tree| {
            tree.add_node(0, 5, true);
            let right_id = tree.add_node(0, 22, false);
            tree.add_node(right_id, 7, true);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_deep_left_subtree_violation() {
        // The node 12 is in the left subtree of 10, so this is invalid.
        assert_bst_methods_agree(10, false, |tree| {
            let left_id = tree.add_node(0, 6, true);
            tree.add_node(left_id, 12, false);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_duplicate_keys() {
        assert_bst_methods_agree(10, false, |tree| {
            tree.add_node(0, 10, true);
        });
    }

    #[test]
    fn test_check_bts_agrees_on_boundary_values() {
        assert_bst_methods_agree(1, true, |tree| {
            tree.add_node(0, 0, true);
            let right_id = tree.add_node(0, u32::MAX, false);
            tree.add_node(right_id, 2, true);
        });
    }

    #[test]
    fn test_max_path_sum_standard() {
        // Structure:
        //       10
        //      /  \
        //     5    22
        //      \  /  \
        //       7 20 25
        // Leaves: 7, 20, 25.
        // Paths between leaves:
        // 7 to 20: 7 + 5 + 10 + 22 + 20 = 64
        // 7 to 25: 7 + 5 + 10 + 22 + 25 = 69  <-- Maximum
        // 20 to 25: 20 + 22 + 25 = 67
        let mut tree = Tree::with_root(10);
        let id_1 = tree.add_node(0, 5, true);
        let id_2 = tree.add_node(0, 22, false);
        tree.add_node(id_1, 7, false);
        tree.add_node(id_2, 20, true);
        tree.add_node(id_2, 25, false);

        assert_eq!(tree.max_path_sum(), Some(69));
    }

    #[test]
    fn test_max_path_sum_single_node_tree() {
        let mut tree = Tree::with_root(10);

        assert_eq!(
            tree.max_path_sum(),
            None,
            "A single-node tree does not contain a path between two leaves"
        );
    }

    #[test]
    fn test_max_path_sum_no_leaf_to_leaf_path() {
        // Structure: Skewed Tree
        //     10
        //    /
        //   5
        //  /
        // 2
        // There is only ONE leaf (2). Therefore, no path connecting TWO leaves exists.
        let mut tree = Tree::with_root(10);
        let id_1 = tree.add_node(0, 5, true);
        tree.add_node(id_1, 2, true);

        assert_eq!(
            tree.max_path_sum(),
            None,
            "Should return None as there are not two leaves to connect"
        );
    }

    #[test]
    fn test_max_path_sum_bridge_is_not_root() {
        // Structure:
        //         10
        //        /
        //       5
        //      / \
        //     3   8
        // Leaves: 3, 8.
        // The max path should only go up to 5 and back down: 3 + 5 + 8 = 16.
        // The 10 cannot be included because it has no right child to reach another leaf.
        let mut tree = Tree::with_root(10);
        let id_1 = tree.add_node(0, 5, true);
        tree.add_node(id_1, 3, true);
        tree.add_node(id_1, 8, false);

        assert_eq!(tree.max_path_sum(), Some(16));
    }

    #[test]
    fn test_max_path_sum_large_values() {
        // Testing that the logic handles larger `u32` keys correctly and returns a `u64`.
        // Structure:
        //        u32::MAX
        //         /    \
        //     u32::MAX u32::MAX
        let mut tree = Tree::with_root(u32::MAX);
        tree.add_node(0, u32::MAX, true);
        tree.add_node(0, u32::MAX, false);

        let expected_sum = (u32::MAX as u64) * 3;
        assert_eq!(tree.max_path_sum(), Some(expected_sum));
    }

    #[test]
    fn test_max_path_sum_prefers_best_internal_subtree() {
        // Structure:
        //          1
        //        /   \
        //      50     2
        //     /  \
        //   40    60
        //
        // The best path is entirely inside the left subtree:
        // 40 + 50 + 60 = 150.
        // Any path using the root is smaller.
        let mut tree = Tree::with_root(1);
        let left_id = tree.add_node(0, 50, true);
        tree.add_node(0, 2, false);
        tree.add_node(left_id, 40, true);
        tree.add_node(left_id, 60, false);

        assert_eq!(tree.max_path_sum(), Some(150));
    }

    #[test]
    #[should_panic(expected = "Parent node id does not exist")]
    fn test_add_node_panics_for_invalid_parent() {
        let mut tree = Tree::with_root(10);

        tree.add_node(99, 5, true);
    }

    #[test]
    #[should_panic(expected = "Parent node has the left child already set")]
    fn test_add_node_panics_when_left_child_already_exists() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);

        tree.add_node(0, 7, true);
    }
}

fn main() {
    let mut tree = Tree::with_root(10);
    let id_1 = tree.add_node(0, 5, true);
    let id_2 = tree.add_node(0, 22, false);

    tree.add_node(id_1, 7, false);
    tree.add_node(id_2, 20, true);
    tree.add_node(id_2, 25, false);

    let recursive_is_bst = tree.check_bts(true);
    let morris_is_bst = tree.check_bts(false);

    assert_eq!(recursive_is_bst, morris_is_bst);
    assert!(recursive_is_bst, "Tree should be a valid BST");
}
