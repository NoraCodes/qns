/// The simulation state root, owning all other data about a simulation.
pub struct Simulator {
    ticks: u64,
}

impl Default for Simulator {
    fn default() -> Self {
        Self {
            ticks: 0
        }
    }
}

impl Simulator {
    /// Create a new simulator with no state at the beginning of time.
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance the simulator state by the given number of ticks.
    pub fn advance(&mut self, ticks: u64) { self.ticks += ticks; }

    /// Return the number of ticks that have been executed by the simulator so far.
    pub fn ticks_so_far(&self) -> u64 { self.ticks }
}

#[test]
fn run_forward_advances_ticks() {
    let mut s = Simulator::new();
    assert_eq!(s.ticks_so_far(), 0);
    s.advance(1);
    assert_eq!(s.ticks_so_far(), 1);
}

