/* Given the root to a binary tree, implement serialize(root), which serializes the tree into a string, and deserialize(s), which deserializes the string back into the tree.
# For example, given the following Node class
1
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

# The following test should pass:
node = Node('root', Node('left', Node('left.left')), Node('right'))
assert deserialize(serialize(node)).left.left.val == 'left.left'
*/

#[derive(Debug, PartialEq, Eq)]
struct Node {
    value: &'static str,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

trait Serde {
    fn serialize(&self) -> Result<String, &'static str>;
    fn deserialize(s: &mut impl Iterator<Item = &'static str>) -> Option<Box<Node>>;
}

impl Serde for Node {
    fn serialize(&self) -> Result<String, &'static str> {
        let mut binary_tree_preorder = Vec::<String>::new();

        if self.value.contains("-") {
            panic!("The '-' character is not acceptable for node value.");
        }

        // preorder walk
        binary_tree_preorder.push(self.value.to_owned());
        for node in [&self.left, &self.right] {
            match node {
                Some(n) => {
                    binary_tree_preorder.push(n.serialize()?);
                }
                None => {
                    binary_tree_preorder.push(String::from("*"));
                }
            };
        }

        Ok(binary_tree_preorder.join("-"))
    }

    fn deserialize(s: &mut impl Iterator<Item = &'static str>) -> Option<Box<Node>> {
        let value = s.next();

        match value {
            Some("*") | None => {
                return None;
            }
            Some(val) => {
                let node = Some(Box::new(Node {
                    value: val,
                    left: Node::deserialize(s),
                    right: Node::deserialize(s),
                }));

                return node;
            }
        }
    }
}

// let mut binary_tree_preorder: Vec<&str> = s.split('-').collect();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let node_left2 = Some(Box::new(Node {
            value: "left.left",
            left: None,
            right: None,
        }));

        let node_left1 = Some(Box::new(Node {
            value: "left",
            left: node_left2,
            right: None,
        }));

        let node_right1 = Some(Box::new(Node {
            value: "right",
            right: None,
            left: None,
        }));

        let root = Node {
            value: "root",
            left: node_left1,
            right: node_right1,
        };

        match root.serialize() {
            Ok(s) => assert_eq!(s, "root-left-left.left-*-*-*-right-*-*"),
            Err(e) => panic!("{}", e),
        };
    }

    #[test]
    #[should_panic]
    fn test_serialize_panic() {
        let root = Node {
            value: "root-",
            left: None,
            right: None,
        };

        let _ = root.serialize();
    }

    #[test]
    fn test_deserialize() {
        let serialized_text: Vec<&str> = "root-left-left.left-*-*-*-right-*-*".split('-').collect();
        let expected = Some(Box::new(Node {
            value: "root",
            left: Some(Box::new(Node {
                value: "left",
                left: Some(Box::new(Node {
                    value: "left.left",
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(Node {
                value: "right",
                left: None,
                right: None,
            })),
        }));

        let result = Node::deserialize(&mut serialized_text.into_iter());

        assert_eq!(result, expected)
    }
}
