use hdk::prelude::*;


entry_defs![
   Handle::entry_def(),
   Real::entry_def(),
    Number::entry_def(),
   Thing::entry_def()
];

#[hdk_entry(id = "Thing", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Thing {
    pub value: u32,
}

#[hdk_entry(id = "Number", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Number {
    pub value: u32,
}


#[hdk_entry(id = "Real", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct Real {
    pub value: f32,
}


#[hdk_entry(id = "Handle", visibility = "private")]
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
