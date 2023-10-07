#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Extensions(());

impl Extensions {
    pub fn new() -> Self {
        Self(())
    }

    // TODO: Add extension manipulations
}

// TODO: Add different storages for extensions, based on serde yes or no

TODO: "Make extensions a trait that depending on serde required Serialize/
Deserialize. Implement extensions as a generic with trait bound Extensions"