use std::collections::HashMap;

pub mod puzzle11a;
pub mod puzzle11b;

#[derive(Debug)]
pub struct Node {
    name: String,
    neighbors: Vec<usize>,
}

impl Node {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            neighbors: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn parse(input: &str) -> Self {
        // first figure out the set of nodes
        let mut nodes: Vec<Node> = input
            .lines()
            .map(|line| {
                let (name, _) = line.split_once(':').unwrap();
                Node::new(name)
            })
            .collect();
        nodes.push(Node::new("out"));

        // now scan through and compute connections
        for (idx, line) in input.lines().enumerate() {
            // skip "xxx: " at the start of each line
            for nbr_name in line[5..].split(' ') {
                let nbr_idx = nodes.iter().position(|n| n.name == nbr_name).unwrap();
                nodes[idx].neighbors.push(nbr_idx);
            }
        }

        Self { nodes }
    }

    pub fn find_node(&self, name: &str) -> usize {
        self.nodes.iter().position(|n| n.name == name).unwrap()
    }

    pub fn count_paths(&self, start: usize, end: usize) -> usize {
        let mut memo = HashMap::new();

        self.count_paths_helper(start, end, &mut memo)
    }

    pub fn count_paths_helper(
        &self,
        start: usize,
        end: usize,
        memo: &mut HashMap<usize, usize>,
    ) -> usize {
        if start == end {
            return 1;
        }

        if let Some(count) = memo.get(&start) {
            return *count;
        }

        let start_node = &self.nodes[start];
        let count = start_node
            .neighbors
            .iter()
            .map(|nbr_idx| self.count_paths_helper(*nbr_idx, end, memo))
            .sum();

        memo.insert(start, count);

        count
    }
}
