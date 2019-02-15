use super::{Cell, Gate};
use super::dag::{DAG, Tree};

pub struct LaydownWalker<'a> {
    gates: &'a Vec<Gate>,
    cells: &'a Vec<Cell>,
    trees: &'a Vec<Tree>,
    dag: &'a DAG,
}

impl<'a> LaydownWalker<'a> {
    pub fn new(gates: &'a Vec<Gate>, cells: &'a Vec<Cell>, trees: &'a Vec<Tree>, dag: &'a DAG) -> LaydownWalker<'a> {
        LaydownWalker { gates, cells, trees, dag }
    }

    pub fn find_min_delay(&self) -> u32 {
        let mut tree_timings: Vec<u32> = Vec::new();
        for tree in self.trees {
            tree_timings.push(self.walk(tree.root(), tree));
        }

        LaydownWalker::vec_max(&tree_timings)
    }

    fn walk(&self, root: u32, tree: &Tree) -> u32 {
        let mut node_timings: Vec<u32> = Vec::new();
        match tree.children(root) {
            Some(x) => {
                // First calculate regular gate mapping.
                let mut child_timings: Vec<u32> = Vec::new();
                for child in x {
                    child_timings.push(self.walk(child.clone(), tree) + self.gate_delay(root));
                }
                node_timings.push(LaydownWalker::vec_max(&child_timings));
            },
            None => (node_timings.push(self.gate_delay(root))),
        }

        // Determine if any complex cells match the tree and then calculate their timings
        let matches = self.cell_matches(root, tree);
        for (cell, child_match) in matches {
            let mut child_timings: Vec<u32> = Vec::new();
            match tree.children(child_match) {
                Some(x) => {
                    for secondary_child in x {
                        let child_time = self.walk(secondary_child.clone(), tree);
                        child_timings.push(child_time + cell.delay_a);
                    }
                }
                None => (child_timings.push(cell.delay_a)),
            }

            match tree.children(root) {
                Some(x) => {
                    for primary_child in x {
                        if primary_child.clone() != child_match {
                            let child_time = self.walk(primary_child.clone(), tree);
                            child_timings.push(child_time + cell.delay_b);
                        }
                    }
                },
                None => (child_timings.push(cell.delay_b)),
            }

            node_timings.push(LaydownWalker::vec_max(&child_timings));
        }

        LaydownWalker::vec_min(&node_timings)
    }

    fn gate_delay(&self, node: u32) -> u32 {
        let gate_type = self.dag.gate_type(node);
        let delay = self.gates.get(gate_type as usize).unwrap().delay();
        delay
    }

    fn cell_matches(&self, root: u32, tree: &Tree) -> Vec<(&Cell, u32)> {
        let mut matches: Vec<(&Cell, u32)> = Vec::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.gate_b == self.dag.gate_type(root) {
                match tree.children(root) {
                    Some(x) => {
                        for child in x {
                            if cell.gate_a == self.dag.gate_type(child.clone()) {
                                matches.push((cell, child.clone()));
                            }
                        }
                    },
                    None => (),
                }
            }
        }

        matches
    }

    fn vec_max(vec: &Vec<u32>) -> u32 {
        let mut max = std::u32::MIN;
        for value in vec {
            let value = value.clone();
            if value > max {
                max = value;
            }
        }

        max
    }

    fn vec_min(vec: &Vec<u32>) -> u32 {
        let mut min = std::u32::MAX;
        for value in vec {
            let value = value.clone();
            if value < min {
                min = value;
            }
        }

        min
    }
}