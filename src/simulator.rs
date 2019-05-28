use crate::node::{Node, PublicIdentity};
use crate::medium;
use petgraph::{Graph, Undirected};
use std::collections::HashMap;

/// The internal identities of nodes used within the simulator.
type NodeIndex = petgraph::graph::NodeIndex<usize>;

/// The simulation state root, owning all other data about a simulation.
/// The simulator tracks nodes by their connections and their cryptographic identities.
pub struct Simulator {
    ticks: u64,
    directory: HashMap<PublicIdentity, NodeIndex>,
    graph: Graph<Node, medium::Medium, Undirected, usize>
}

impl Default for Simulator {
    fn default() -> Self {
        Self {
            ticks: 0,
            directory: HashMap::new(),
            graph: Graph::default()
        }
    }
}

impl Simulator {
    /// Create a new simulator with no state at the beginning of time.
    pub fn new() -> Self { Self::default() }

    /// Advance the simulator state by the given number of ticks.
    pub fn advance(&mut self, ticks: u64) { self.ticks += ticks; }

    /// Return the number of ticks that have been executed by the simulator so far.
    pub fn ticks_so_far(&self) -> u64 { self.ticks }

    /// Create a new host in the network with the given node information.
    /// Returns a `DuplicateCryptographicIdentityError` if the given ID is not unique.
    pub fn add_node(&mut self, node: Node) -> Result<(), SimulatorError> {
        let id = node.get_public_id();
        let index = self.graph.add_node(node);
        if self.directory.contains_key(&id) {
            return Err(SimulatorError::DuplicateCryptographicIdentityError(id));
        } else {
            self.directory.insert(id, index);
            return Ok(());
        }
    }

    /// Get a reference to the host with the given cryptographic ID.
    pub fn get_node(&self, id: PublicIdentity) -> Option<&Node> {
        self.directory.get(&id).map(|index| &self.graph[*index])
    }
}

#[derive(Debug, PartialEq)]
pub enum SimulatorError {
    DuplicateCryptographicIdentityError(PublicIdentity)
}

#[test]
fn run_forward_advances_ticks() {
    let mut s = Simulator::new();
    assert_eq!(s.ticks_so_far(), 0);
    s.advance(1);
    assert_eq!(s.ticks_so_far(), 1);
}

#[test]
fn add_and_retrieve_nodes() {
    let mut s = Simulator::new();
    let n = Node::with_id(0);
    let id = n.get_public_id();
    s.add_node(n.clone()).expect("Could not add node to simulator. Error");
    let r = s.get_node(id).expect("Could not get node from simulator.");
    assert_eq!(&n, r);
}

#[test]
fn cannot_add_duplicate_nodes() {
    let mut s = Simulator::new();
    let n1 = Node::with_id(0);
    let n2 = Node::with_id(0);
    s.add_node(n1).expect("Could not add first node to simulator. Error");
    s.add_node(n2).expect_err("Was able to add second node to simulator.");
}

