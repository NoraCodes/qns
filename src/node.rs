/// The unique cryptographic identity of each node in the network.
// This is not related to the node index within the Petgraph graph.
pub type Identity = u64;
/// The public component of the unique cryptographic identity of each node in the network.
pub type PublicIdentity = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    identity: Identity
}

impl Node {
    /// Create a new, otherwise empty node with the given cryptographic identity.
    pub fn with_id(id: Identity) -> Self { Self { identity: id } }

    pub fn get_public_id(&self) -> PublicIdentity { self.identity }
}

pub trait NodeRouter {

}

