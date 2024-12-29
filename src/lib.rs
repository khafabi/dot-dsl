//! In a single file, with the same tests and purely functional merging.

/// The entire `graph` functionality in one module.
pub mod graph {
    use std::collections::HashMap;

    /// We place Node and Edge types in a nested `graph_items` module to match usage in the tests.
    pub mod graph_items {
        pub mod node {
            // Import the helper from the parent `graph` module.
            use crate::graph::merge_map_and_list;
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    // Merge existing attrs with the new list, purely functional
                    let merged_attrs = merge_map_and_list(&self.attrs, attrs);
                    Node {
                        name: self.name,
                        attrs: merged_attrs,
                    }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }

                pub fn name(&self) -> &str {
                    &self.name
                }
            }
        }

        pub mod edge {
            // Import the helper from the parent `graph` module.
            use crate::graph::merge_map_and_list;
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                node1: String,
                node2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let merged_attrs = merge_map_and_list(&self.attrs, attrs);
                    Edge {
                        node1: self.node1,
                        node2: self.node2,
                        attrs: merged_attrs,
                    }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            // purely functional concatenation
            let merged_nodes = concat_slices(&self.nodes, nodes);
            Graph {
                nodes: merged_nodes,
                edges: self.edges,
                attrs: self.attrs,
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            let merged_edges = concat_slices(&self.edges, edges);
            Graph {
                nodes: self.nodes,
                edges: merged_edges,
                attrs: self.attrs,
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            let merged_attrs = merge_map_and_list(&self.attrs, attrs);
            Graph {
                nodes: self.nodes,
                edges: self.edges,
                attrs: merged_attrs,
            }
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            find_node_by_name(&self.nodes, name)
        }
    }

    // -------------------------------------------------------------------------
    // HELPER FUNCTIONS BELOW (purely functional merging, recursion, etc.)
    // -------------------------------------------------------------------------

    /// Merge an existing `HashMap` of `String->String` with a slice of `(&str, &str)`.
    /// New keys override old ones.
    pub fn merge_map_and_list(
        map: &HashMap<String, String>,
        kvs: &[(&str, &str)],
    ) -> HashMap<String, String> {
        let map_vec = map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        let new_map_from_list = from_kv_list(kvs);

        merge_two_maps(&map_vec, &new_map_from_list)
    }

    /// Recursively build a HashMap<String, String> from a slice of (&str, &str).
    fn from_kv_list(kvs: &[(&str, &str)]) -> HashMap<String, String> {
        match kvs.split_first() {
            None => HashMap::new(),
            Some(((k, v), tail)) => {
                // Build the tail map
                let tail_map = from_kv_list(tail);
                // Insert/overwrite the current (k, v)
                merge_two_maps(&[], &insert_single_kv(&tail_map, k, v))
            }
        }
    }

    /// Return a new HashMap = map + (k, v), with (k, v) overwriting if needed.
    fn insert_single_kv(map: &HashMap<String, String>, k: &str, v: &str) -> HashMap<String, String> {
        let as_vec = map
            .iter()
            .map(|(kk, vv)| (kk.clone(), vv.clone()))
            .collect::<Vec<_>>();
        let appended = concat_slices(&as_vec, &[(k.to_string(), v.to_string())]);
        appended.into_iter().collect()
    }

    /// Merge two "maps" (represented by a Vec of key/value pairs, and a HashMap).
    /// On key collisions, the second_map overrides.
    fn merge_two_maps(
        first: &[(String, String)],
        second_map: &HashMap<String, String>,
    ) -> HashMap<String, String> {
        let second_vec = second_map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();

        let combined = concat_slices(first, &second_vec);
        combined.into_iter().collect()
    }

    /// Purely functional concatenation with recursion (no mutation).
    fn concat_slices<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
        match a.split_first() {
            None => b.to_vec(),
            Some((head, tail)) => {
                let tail_concat = concat_slices(tail, b);
                [vec![head.clone()].as_slice(), &tail_concat].concat()
            }
        }
    }

    /// Recursively find a `Node` by name, returning the first match or None.
    fn find_node_by_name<'a>(nodes: &'a [Node], name: &str) -> Option<&'a Node> {
        match nodes.split_first() {
            None => None,
            Some((head, tail)) => {
                if head.name() == name {
                    Some(head)
                } else {
                    find_node_by_name(tail, name)
                }
            }
        }
    }
}