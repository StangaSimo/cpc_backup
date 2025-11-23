use std::cmp;

const LEFT: bool = true;
const RIGHT: bool = false;

fn min_three(a: u32, b: u32, c: u32) -> u32 {
    cmp::min(cmp::min(a, b), c)
}

fn max_three(a: u32, b: u32, c: u32) -> u32 {
    cmp::max(cmp::max(a, b), c)
}

struct Rescheck {
    b: bool,
    min: u32,
    max: u32,
}

struct Res {
    b: bool,
    height: u32,
}

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

struct Tree {
    nodes: Vec<Node>,
}

/// This a representation of a tree.
/// Every node has an implicity id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
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
        self.rec_check_bst(Some(0)).b
    }

    fn rec_check_bst(&self, node_id: Option<usize>) -> Rescheck {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            let res_left: Rescheck = self.rec_check_bst(node.id_left);
            let res_right: Rescheck = self.rec_check_bst(node.id_right);

            if res_left.max > node.key || res_right.min < node.key {
                return Rescheck {
                    b: (false),
                    min: (min_three(node.key, res_left.min, res_right.min)),
                    max: (max_three(node.key, res_left.max, res_right.max)),
                };
            } else {
                return Rescheck {
                    b: (res_left.b && res_right.b),
                    min: (min_three(node.key, res_left.min, res_right.min)),
                    max: (max_three(node.key, res_left.max, res_right.max)),
                };
            }
        }
        Rescheck {
            b: (true),
            min: (u32::MAX),
            max: (u32::MIN),
        }
    }

    pub fn is_balanced(&self) -> bool {
        self.rec_is_balanced(Some(0)).b
    }

    fn rec_is_balanced(&self, node_id: Option<usize>) -> Res {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            let res_left = self.rec_is_balanced(node.id_left);
            let res_right = self.rec_is_balanced(node.id_right);
            let res_height: i32 = res_left.height as i32 - res_right.height as i32;
            let res_b: bool = res_left.b && res_right.b && (i32::abs(res_height) <= 1);
            return Res {
                b: (res_b),
                height: (cmp::max(res_right.height, res_left.height) + 1),
            };
        }
        Res {
            b: (true),
            height: (0),
        }
    }

    pub fn is_maxheap(&self) -> bool {
        self.rec_is_maxheap(Some(0),0)
    }

    fn rec_is_maxheap(&self, node_id: Option<usize>, total: u32) -> bool {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            /* complete */ 
            if total >= self.nodes.len() as u32 {
                return false;
            }

            /* max heap on left leaf */
            if let Some(id_left) = node.id_left {
                let leftnode = &self.nodes[id_left];

                if node.key < leftnode.key {
                    return false;
                }

                /* max heap on right node */
                if let Some(id_right) = node.id_right {

                    let rightnode = &self.nodes[id_right];

                    /* max heap */
                    if node.key < rightnode.key {
                        return false;
                    }
                }
            }
            return self.rec_is_maxheap(node.id_left,total*2+1) 
                        && self.rec_is_maxheap(node.id_right, total*2+2);
        }
        true
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
    fn test_is_balanced() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, LEFT);
        tree.add_node(0, 40, RIGHT);
        tree.add_node(1, 5, LEFT);
        tree.add_node(1, 15, RIGHT);
        tree.add_node(2, 30, LEFT);
        tree.add_node(2, 50, RIGHT);
        assert_eq!(tree.is_balanced(), true);
        tree.add_node(5, 25, LEFT);
        tree.add_node(5, 35, RIGHT);
        tree.add_node(7, 24, LEFT);
        tree.add_node(7, 26, RIGHT);
        assert_eq!(tree.is_balanced(), false);
        tree.add_node(3, 26, RIGHT);
        tree.add_node(3, 20, LEFT);
        tree.add_node(4, 28, LEFT);
        tree.add_node(4, 29, RIGHT);
        tree.add_node(6, 29, RIGHT);
        tree.add_node(6, 29, LEFT);
        assert_eq!(tree.is_balanced(), true);
    }

    #[test]
    fn test_is_maxheap() {
        let mut tree = Tree::with_root(70);
        tree.add_node(0, 60, LEFT);
        tree.add_node(0, 66, RIGHT);

        assert_eq!(tree.is_maxheap(), true);
        tree.add_node(1, 59, LEFT);
        tree.add_node(2, 62, LEFT);
        assert_eq!(tree.is_maxheap(), false);
        tree.add_node(1, 58, RIGHT);
        assert_eq!(tree.is_maxheap(), true);

        tree.add_node(2, 65, RIGHT);
        tree.add_node(3, 58, LEFT);
        tree.add_node(3, 59, RIGHT);
        tree.add_node(5, 50, LEFT);
        tree.add_node(5, 40, RIGHT);
        assert_eq!(tree.is_maxheap(), true);
        tree.add_node(4, 100, RIGHT);
        assert_eq!(tree.is_maxheap(), false);

        //assert_eq!(tree.is_maxheap(), false);

        let mut tree = Tree::with_root(70);
        tree.add_node(0, 60, LEFT);
        tree.add_node(0, 66, RIGHT);
        tree.add_node(1, 59, LEFT);
        tree.add_node(1, 58, RIGHT);
        tree.add_node(2, 62, LEFT);
        tree.add_node(2, 65, RIGHT);
        tree.add_node(3, 58, LEFT);
        tree.add_node(3, 59, RIGHT);
        tree.add_node(4, 50, LEFT);
        tree.add_node(4, 40, RIGHT);
        tree.add_node(5, 61, LEFT);
        tree.add_node(5, 59, RIGHT);
        tree.add_node(6, 64, LEFT);
        tree.add_node(6, 60, RIGHT);

        assert_eq!(tree.is_maxheap(), true);
        tree.add_node(13, 9, RIGHT);
        assert_eq!(tree.is_maxheap(), false);
    }
}
