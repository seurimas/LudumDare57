use crate::prelude::*;

#[derive(Clone, Debug, Default, Reflect, PartialEq, Eq, Hash)]
pub struct PlayerId(pub usize);

#[derive(Clone, Debug, Reflect)]
pub struct Agent {
    pub name: String,
    pub artifacts: Vec<Artifact>,
    pub affinity: Option<PlayerId>,
}

#[derive(Clone, Debug, Reflect)]
pub enum Artifact {
    AvoidIre,      // Avoids a drawn ire, and then is discarded.
    BankPower,     // Do not lose power when ire is drawn.
    PickFromTwo,   // Picks the best of two drawn cards, once per turn.
    PickFromThree, // Picks the best of three drawn cards, once per turn.
}
