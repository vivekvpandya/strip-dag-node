use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParsingError;
impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParsingError")
    }
}

// NOTE: In below hashmap key represents vertex and value is all nodes which has edge from vertex to that node.
//		 A hashmap entry with empty value is treated as unconnected vertex in graph
#[derive(Debug, PartialEq, Eq)]
pub struct Graph {
    data: HashMap<char, HashSet<char>>,
}

//NOTE : With current representaion delete_node() function becomes very hard to understand,
//	     if we simplify delete_node() then to_string() needs to handle complexity of printing
//		 nodes which does not have any edge.
//		 It seems that using adjacancy matrix can simplify things.

impl Graph {
    pub fn new() -> Self {
        Graph {
            data: HashMap::new(),
        }
    }
    // return true if node existed and deleted
    // return false if node do not exist
    pub fn delete_node(&mut self, node: char) -> bool {
        // remove edge which is incoming to given node, and create a vec of source nodes.
        let mut nodes: Vec<char> = Vec::new();
        for (key, val) in self.data.iter_mut() {
            if val.remove(&node) {
                nodes.push(*key);
            }
        }

        // Remove given node from HashMap
        let set = self.data.remove(&node);
        match set {
            Some(s) => {
                // create an edge from all source node collected above to
                // all nodes which has edge from given node
                for k in &nodes {
                    let val = self.data.get_mut(&k).unwrap();
                    for v in &s {
                        val.insert(*v);
                    }
                }

                // if removing given node makes any node unconnected, then add it to HashMap
                for v in &s {
                    //assert!(self.data.contains_key(v));

                    let mut unconnected = true;
                    for (_, val) in self.data.iter() {
                        if val.contains(&v) {
                            unconnected = false;
                            break;
                        }
                    }
                    if unconnected && !self.data.contains_key(v) {
                        // v is a node which does not have anyother incoming edge.
                        #[allow(unused_mut)]
                        let mut set = HashSet::new();
                        self.data.insert(*v, set);
                    }
                }
                true
            }
            None => {
                if !nodes.is_empty() {
				// if any node was poiting to given node only then it may become
				// unconnected. Check for that and update the graph accordingly.
                    for v in &nodes {
                        let mut unconnected = true;
                        for (_, val) in self.data.iter() {
                            if val.contains(&v) {
                                unconnected = false;
                                break;
                            }
                        }
                        if unconnected && !self.data.contains_key(v) {
                            // v is a node which does not have anyother incoming edge.
                            #[allow(unused_mut)]
                            let mut set = HashSet::new();
                            self.data.insert(*v, set);
                        }
                        if !unconnected
                            && self.data.contains_key(v)
                            && self.data.get(&v).unwrap().is_empty()
                        {
                            self.data.remove(&v);
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl FromStr for Graph {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = Graph::new();
        let edges: Vec<&str> = s.trim().split(',').collect();
        for edge in edges {
            let nodes = edge.split('-').flat_map(|s| s.chars()).collect::<Vec<_>>();
            if nodes.len() > 2 || nodes.is_empty() {
                return Err(ParsingError);
            }
            if nodes.len() == 2 {
                let val = g.data.get_mut(&nodes[0]);
                match val {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(nodes[1]);
                        g.data.insert(nodes[0], set);
                    }
                    Some(set) => {
                        set.insert(nodes[1]);
                    }
                }
                // #[allow(unused_mut)]
                // let mut set = HashSet::new();
                // g.data.entry(nodes[1]).or_insert(set);
            } else {
                // node without any edge
                #[allow(unused_mut)]
                let mut set = HashSet::new();
                g.data.insert(nodes[0], set);
            }
        }
        //TODO: After creating Graph, check for cycles
        Ok(g)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Iterate over keyvalue pair and print
        let mut index = 0;
        let stop = self.data.len();
        for (key, value) in self.data.iter() {
            index += 1;
            let mut val_index = 0;
            if value.is_empty() {
                // node without any edge
                write!(f, "{}", key)?;
            } else {
                for v in value {
                    val_index += 1;
                    write!(f, "{}-{}", key, v)?;
                    if val_index != value.len() {
                        write!(f, ",")?;
                    }
                }
            }
            if index != stop {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Graph;

    #[test]
    fn create_from_str() {
        use std::str::FromStr;
        let g = Graph::from_str("a-b,b-c,c-d");
        assert!(g.is_ok());

        let g = Graph::from_str("a-b,b-c,f,c-d,k");
        assert!(g.is_ok());

        let g = Graph::from_str("f,k");
        assert!(g.is_ok());

        let g = Graph::from_str("f");
        assert!(g.is_ok());

        let g = Graph::from_str("");
        assert!(g.is_err());
    }

    #[test]
    fn delete_node_test() {
        use std::str::FromStr;
        let g = Graph::from_str("a-b,b-c,c-d");
        let g_ref = &mut g.unwrap();
        assert!(g_ref.delete_node('a'));
        println!("Before deleting 'a' {}", g_ref.to_string());
        assert_eq!(g_ref.delete_node('k'), false);
        println!("After deleting 'a' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-c,f,c-d,k");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'f' {}", g_ref.to_string());
        assert!(g_ref.delete_node('f'));
        println!("After deleting 'f' {}", g_ref.to_string());

        let g = Graph::from_str("f,k");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'f' {}", g_ref.to_string());
        assert!(g_ref.delete_node('f'));
        println!("After deleting 'f' {}", g_ref.to_string());

        let g = Graph::from_str("f");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'f' {}", g_ref.to_string());
        assert!(g_ref.delete_node('f'));
        println!("After deleting 'f' {}", g_ref.to_string());

        let g = Graph::from_str("b-a,c-a,d-a,e-a");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'a' {}", g_ref.to_string());
        assert!(g_ref.delete_node('a'));
        println!("After deleting 'a' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,a-c,a-d,a-e");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'a' {}", g_ref.to_string());
        assert!(g_ref.delete_node('a'));
        println!("After deleting 'a' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-c,c-d,d-e");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'a' {}", g_ref.to_string());
        assert!(g_ref.delete_node('a'));
        println!("After deleting 'a' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-c,c-d,d-e");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'e' {}", g_ref.to_string());
        assert!(g_ref.delete_node('e'));
        println!("After deleting 'e' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-c,c-d,d-e");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'c' {}", g_ref.to_string());
        assert!(g_ref.delete_node('c'));
        println!("After deleting 'c' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-d,c-d,a-c");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'c' {}", g_ref.to_string());
        assert!(g_ref.delete_node('c'));
        println!("After deleting 'c' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-d,c-d,a-c");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'a' {}", g_ref.to_string());
        assert!(g_ref.delete_node('a'));
        println!("After deleting 'a' {}", g_ref.to_string());

        let g = Graph::from_str("a-b,b-d,c-d,a-c");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'd' {}", g_ref.to_string());
        assert!(g_ref.delete_node('d'));
        println!("After deleting 'd' {}", g_ref.to_string());

		let g = Graph::from_str("a-b,c-d");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'a' {}", g_ref.to_string());
        assert!(g_ref.delete_node('a'));
        println!("After deleting 'a' {}", g_ref.to_string());

		let g = Graph::from_str("a-b,c-d");
        assert!(g.is_ok());
        let g_ref = &mut g.unwrap();
        println!("Before deleting 'd' {}", g_ref.to_string());
        assert!(g_ref.delete_node('d'));
        println!("After deleting 'd' {}", g_ref.to_string());
    }

    #[test]
    fn to_string_test() {
        use std::str::FromStr;
        let g = Graph::from_str("a-b,b-c,c-d").unwrap();
        println!("{}", g.to_string());
        let g = Graph::from_str(&g.to_string());
        assert!(g.is_ok());

        let g = Graph::from_str("a-b,b-c,f,c-d,k").unwrap();
        println!("{}", g.to_string());
        let g = Graph::from_str(&g.to_string());
        assert!(g.is_ok());

        let g = Graph::from_str("f,k").unwrap();
        println!("{}", g.to_string());
        let g = Graph::from_str(&g.to_string());
        assert!(g.is_ok());

        let g = Graph::from_str("f").unwrap();
        println!("{}", g.to_string());
        let g = Graph::from_str(&g.to_string());
        assert!(g.is_ok());
    }
}
