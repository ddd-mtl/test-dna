use hdk::prelude::*;


/// Entry representing the username of an Agent
#[hdk_entry(id = "Number", visibility = "public")]
#[derive(Clone, PartialEq)]
pub struct Number {
    pub value: u32,
}

/// Entry representing the username of an Agent
#[hdk_entry(id = "Handle", visibility = "public")]
#[derive(Clone, PartialEq)]
pub struct Handle {
    pub name: String,
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: String::new(),
        }
    }

    /// DEBUG
    pub fn dummy() -> Self {
        Self {
            name: "dummy".to_string(),
        }
    }
}
