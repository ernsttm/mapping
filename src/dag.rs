use std::collections::HashMap;
use std::fmt::{ Debug, Error, Formatter };

pub struct InputNode {
    pub gate_type: u32,
    pub inputs: Vec<u32>,
}

pub struct Node {
    gate_type: u32,
    inputs: Vec<u32>,
    outputs: Vec<u32>,
}

impl Node {
    fn new(gate_type: u32, inputs: Vec<u32>) -> Node {
        Node { gate_type, inputs, outputs: Vec::new() }
    }

    pub fn gate_type(&self) -> u32 {
        self.gate_type
    }

    pub fn inputs(&self) -> &Vec<u32> {
        &self.inputs
    }

    fn add_output(&mut self, node_index: u32) {
        self.outputs.push(node_index);
    }
}

pub struct Tree {
    root: u32,
    data: HashMap<u32, Vec<u32>>,
}

impl Tree {
    pub fn new(root: u32) -> Tree {
        Tree { root, data: HashMap::new() }
    }

    pub fn root(&self) -> u32 {
        self.root
    }

    pub fn children(&self, parent: u32) -> Option<&Vec<u32>> {
        match self.data.get(&parent) {
            Some(x) => {
                if !x.is_empty() {
                    Some(x)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn add_branch(&mut self, root: u32, children: &Vec<u32>) {
        self.data.insert(root, children.clone());
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Tree with root: {}", self.root)?;

        for (key, value) in &self.data {
            writeln!(f, "{} -> {:?}", key, value)?;
        }

        Ok(())
    }
}

pub struct DAG {
    nodes: Vec<Node>,
}

impl DAG {
    pub fn new(input_wires: u32, input_nodes: &mut Vec<InputNode>) -> DAG {
        let mut node_index = 0;
        let mut nodes: Vec<Node> = Vec::new();
        for input_node in input_nodes {
            let mut pruned_inputs: Vec<u32> = Vec::new();
            for input in input_node.inputs.clone() {
                if input >= input_wires {
                    pruned_inputs.push(input - input_wires);
                }
            }

            let node = Node::new(input_node.gate_type, pruned_inputs);

            for wire in &input_node.inputs {
                if input_wires <= *wire {
                    nodes[(*wire - input_wires) as usize].add_output(node_index);
                }
            }

            nodes.push(node);
            node_index += 1;
        }

        DAG { nodes }
    }

    pub fn gate_type(&self, node: u32) -> u32 {
        self.nodes.get(node as usize).unwrap().gate_type()
    }

    pub fn inputs(&self, node: u32) -> &Vec<u32> {
        self.nodes.get(node as usize).unwrap().inputs()
    }

    pub fn partition(&self) -> Vec<Tree> {
        let mut trees: Vec<Tree> = Vec::new();
        for index in 0..self.nodes.len() {
            if self.nodes[index].outputs.is_empty() {
                trees.push(self.find_tree(index as u32));
            }
        }

        trees
    }

    fn find_tree(&self, root_index: u32) -> Tree {
        let mut tree= Tree::new(root_index);
        let mut tree_stack: Vec<u32> = vec![root_index];
        while !tree_stack.is_empty() {
            let root = tree_stack.pop().expect("Invalid tree stack access");
            tree.add_branch(root, &self.nodes[root as usize].inputs);
            for input in self.nodes[root as usize].inputs.clone() {
                tree_stack.push(input);
            }
        }

        tree
    }
}

impl Debug for DAG {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Dag is of size: {}", self.nodes.len())?;

        let mut node_index = 0;
        for node in &self.nodes {
            writeln!(f, "\t{} Node inputs {:?}, outputs: {:?}",
                     node_index, node.inputs, node.outputs)?;
            node_index += 1;
        }

        Ok(())
    }
}