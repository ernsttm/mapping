mod dag;
mod error;
mod laydown_walker;

use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

use dag::{DAG, InputNode, Tree};
use error::MappingError;
use laydown_walker::LaydownWalker;

/// The basic configuration options required to execute the mapping algorithm
pub struct Config {
    /// The input file
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, MappingError> {
        if 2 != args.len() {
            return Err(MappingError { why: "Too few arguments" });
        }

        let file = args[1].clone();

        Ok(Config { file })
    }
}

pub struct Gate {
    inputs: u32,
    delay: u32,
}

impl Gate {
    pub fn delay(&self) -> u32 {
        self.delay
    }
}

pub struct Cell {
    gate_a: u32,
    gate_b: u32,
    delay_a: u32,
    delay_b: u32,
}

fn read_expected_line(reader: &mut BufReader<&File>) -> Result<String, Box<dyn Error>> {
    let mut line = String::new();
    let num_bytes = reader.read_line(&mut line)?;
    if 0 == num_bytes {
        return Err(MappingError { why: "File contains no more lines" }.into());
    }

    // Remove the strings line ending
    line.pop();
    Ok(line)
}

pub fn process_input(filename: &String) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(&file);


    let gates = process_gates(&mut reader)?;
    let cells = process_cells(&mut reader)?;
    let  (input_wires, mut nodes) = process_dag(&mut reader)?;

    let dag = DAG::new(input_wires, &mut nodes);
    println!("DAG: {:?}", dag);

    let trees = dag.partition();
    println!("Tree: {:?}", trees);

    let walker = LaydownWalker::new(&gates, &cells, &trees, &dag);
    println!("{}", walker.find_min_delay());

    Ok(())
}

fn process_gates(reader: &mut BufReader<&File>) -> Result<Vec<Gate>, Box<dyn Error>> {
    let gate_number = read_expected_line(reader)?;
    let gate_number: u32 = gate_number.parse()?;

    let mut gates: Vec<Gate> = Vec::new();
    for index in 0..gate_number {
        let gate_line = read_expected_line(reader)?;
        let mut gate_line = gate_line.split_whitespace();
        let inputs = match gate_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Gate input not configured."}.into()),
        };
        let inputs = inputs.parse()?;
        let delay = match gate_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Gate delay not configured."}.into()),
        };
        let delay = delay.parse()?;

        gates.push(Gate { inputs, delay });
    }

    Ok(gates)
}

fn process_cells(reader: &mut BufReader<&File>) -> Result<Vec<Cell>, Box<dyn Error>> {
    let cell_number: u32 = read_expected_line(reader)?.parse()?;

    let mut cells: Vec<Cell> = Vec::new();
    for index in 0..cell_number {
        let cell_line = read_expected_line(reader)?;
        let mut cell_line = cell_line.split_whitespace();
        let gate_a = match cell_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Gate a not configured" }.into()),
        };
        let gate_a = gate_a.parse()?;
        let gate_b = match cell_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Gate b not configured" }.into()),
        };
        let gate_b = gate_b.parse()?;
        let delay_a = match cell_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Delay a not configured" }.into()),
        };
        let delay_a = delay_a.parse()?;
        let delay_b = match cell_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Delay b not configured" }.into()),
        };
        let delay_b = delay_b.parse()?;

        cells.push(Cell { gate_a, gate_b, delay_a, delay_b });
    }

    Ok(cells)
}

fn process_dag(reader: &mut BufReader<&File>) -> Result<(u32, Vec<InputNode>), Box<dyn Error>> {
    let dag_definition = read_expected_line(reader)?;
    let mut dag_definition = dag_definition.split_whitespace();
    let inputs = match dag_definition.next() {
        Some(x) => x,
        None => return Err(MappingError { why: "DAG inputs not configured."}.into()),
    };
    let inputs = inputs.parse()?;
    let node_num = match dag_definition.next() {
        Some(x) => x,
        None => return Err(MappingError { why: "DAG number of nodes not configured."}.into()),
    };
    let node_num = node_num.parse()?;

    let mut input_nodes: Vec<InputNode> = Vec::new();
    for index in 0..node_num {
        let node_line = read_expected_line(reader)?;
        let mut node_line = node_line.split_whitespace();
        let gate_type = match node_line.next() {
            Some(x) => x,
            None => return Err(MappingError { why: "Node gate type not configured." }.into()),
        };
        let gate_type = gate_type.parse()?;

        let mut inputs: Vec<u32> = Vec::new();
        loop {
            match node_line.next() {
                Some(x) => inputs.push(x.parse()?),
                None => break,
            }
        }

        input_nodes.push(InputNode { gate_type, inputs });
    }

    Ok((inputs, input_nodes))
}
