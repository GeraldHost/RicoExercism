// NOTES: surely we should be able to use a trait to implement
// the with_attrs function. I wasn't able to do this as it was
// complaining that it didn't know the size of attrs and self.
// need to do some more investigating :-)
pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(), 
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            let mut hashmap = HashMap::new();
            for (key, value) in attrs {
                hashmap.insert(key.to_string(), value.to_string());
            }
            self.attrs = hashmap;
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.name == name)
        }
    }

    pub mod graph_items {    
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                to: String,
                from: String,
                attrs: HashMap<String, String>
            }
    
            impl Edge {
                pub fn new(to: &str, from: &str) -> Edge {
                    Self { to: to.to_string(), from: from.to_string(), attrs: HashMap::new() }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut hashmap = HashMap::new();
                    for (key, value) in attrs {
                        hashmap.insert(key.to_string(), value.to_string());
                    }
                    self.attrs = hashmap;
                    self
                }
            }
        }
    
        pub mod node {
            use std::collections::HashMap;
            
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>
            }
    
            impl Node {
                pub fn new(name: &str) -> Node {
                    Self { name: name.to_string(), attrs: HashMap::new() }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut hashmap = HashMap::new();
                    for (key, value) in attrs {
                        hashmap.insert(key.to_string(), value.to_string());
                    }
                    self.attrs = hashmap;
                    self
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|x| x.as_str())
                }
            }
        }
    }
}
