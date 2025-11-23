#![allow(unused)]
use std::cmp;

const LEFT: bool = true;
const RIGHT: bool = false;

struct Node {
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

struct Res_check_bst {
    res: bool,
    min: u32,
    max: u32,
}
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
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

    pub fn check_bst(&self) -> bool {
        self.rec_check_bst(Some(0)).res
    }

    fn rec_check_bst(&self, node_id: Option<usize>) -> Res_check_bst {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            /* left and right recursive call */
            let res_right = self.rec_check_bst(node.id_right);
            let res_left = self.rec_check_bst(node.id_left);

            /* bad case where one of the sub tree doesn't respect the bst rule */
            if node.key > res_right.min || node.key < res_left.max {
                return Res_check_bst {
                    res: false,
                    min: u32::MIN,
                    max: u32::MAX,
                };
            }

            /* good case and we propagate the results */
            return Res_check_bst {
                res: res_right.res && res_left.res,
                min: cmp::min(cmp::min(node.key, res_left.min), res_right.min),
                max: cmp::max(cmp::max(node.key, res_left.max), res_right.max),
            };
        }

        /* child doesn't exists */
        Res_check_bst {
            res: true,
            min: u32::MAX, /* max in min and min in max so we force to skip the check */
            max: u32::MIN,
        }
    }

    pub fn max_path_sum(&self) -> u32 {
        let mut max = u32::MIN;
        self.rec_max_path_sum(Some(0), &mut max);
        max
    }

    fn rec_max_path_sum(&self, node_id: Option<usize>, max: &mut u32) -> u32 {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            let res_right = self.rec_max_path_sum(node.id_right, max);
            let res_left = self.rec_max_path_sum(node.id_left, max);

            /*
               Initialy i was checking all the cases (leaf, one sub tree ecc...)
               but it was all the same code, we propagate 0, and we don't have
               negative path, so we simply propagate the best path from the sub tree
            */

            let current_path_sum = node.key + res_left + res_right;
            let res = cmp::max(node.key + res_left, node.key + res_right);

            if current_path_sum > *max {
                *max = current_path_sum
            }

            return res;
        }

        /* child doesn't exist return 0 */
        u32::MIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, LEFT);
        tree.add_node(0, 22, RIGHT);

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, LEFT);
        tree.add_node(2, 20, RIGHT);

        assert_eq!(tree.sum(), 64);

        let mut sum: u32 = 64;
        for i in 4..1000 {
            tree.add_node(i, i as u32 + 18, RIGHT);
            sum = sum + i as u32 + 18;
        }

        assert_eq!(tree.sum(), sum);
    }

    #[test]
    fn test_check_bst() {
        let mut tree = Tree::with_root(20);

        tree.add_node(0, 10, LEFT);
        tree.add_node(0, 40, RIGHT);

        assert_eq!(tree.check_bst(), true);

        tree.add_node(1, 5, LEFT);
        tree.add_node(1, 15, RIGHT);

        assert_eq!(tree.check_bst(), true);

        tree.add_node(2, 30, LEFT);
        tree.add_node(2, 50, RIGHT);

        assert_eq!(tree.check_bst(), true);

        tree.add_node(5, 25, LEFT);
        tree.add_node(5, 35, RIGHT);

        assert_eq!(tree.check_bst(), true);

        tree.add_node(7, 24, LEFT);
        tree.add_node(7, 26, RIGHT);
        tree.add_node(10, 22, RIGHT);

        assert_eq!(tree.check_bst(), false);

        let mut tree = Tree::with_root(20);
        tree.add_node(0, 10, LEFT);
        tree.add_node(0, 40, RIGHT);
        tree.add_node(1, 5, LEFT);
        tree.add_node(1, 15, RIGHT);
        tree.add_node(2, 30, LEFT);
        tree.add_node(2, 50, RIGHT);
        tree.add_node(5, 25, LEFT);
        tree.add_node(5, 35, RIGHT);
        tree.add_node(7, 24, LEFT);
        tree.add_node(7, 26, RIGHT);
        tree.add_node(8, 38, RIGHT);
        tree.add_node(11, 39, RIGHT);
        tree.add_node(11, 37, LEFT);
        assert_eq!(tree.check_bst(), true);
    }

    #[test]
    fn test_check_max_path_sum() {
        let mut tree = Tree::with_root(3);

        tree.add_node(0, 4, LEFT);
        tree.add_node(0, 7, RIGHT);

        assert_eq!(tree.max_path_sum(), 14);

        tree.add_node(1, 1, LEFT);
        tree.add_node(1, 6, RIGHT);

        assert_eq!(tree.max_path_sum(), 20);

        let mut tree_1 = Tree::with_root(10);
        assert_eq!(tree_1.max_path_sum(), 10);

        tree_1.add_node(0, 4, LEFT);
        tree_1.add_node(0, 7, RIGHT);
        tree_1.add_node(1, 3, LEFT);
        tree_1.add_node(1, 5, RIGHT);
        tree_1.add_node(4, 6, LEFT);
        tree_1.add_node(4, 8, RIGHT);
        assert_eq!(tree_1.max_path_sum(), 34);

        tree_1.add_node(5, 20, LEFT);
        tree_1.add_node(5, 10, RIGHT);

        tree_1.add_node(8, 40, RIGHT);
        assert_eq!(tree_1.max_path_sum(), 82);

        tree_1.add_node(7, 10, LEFT);
        assert_eq!(tree_1.max_path_sum(), 86);
    }
}
