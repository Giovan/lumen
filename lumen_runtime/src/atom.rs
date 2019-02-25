#![cfg_attr(not(test), allow(dead_code))]

use crate::term::BadArgument;

pub enum Encoding {
    Latin1,
    Unicode,
    Utf8,
}

pub enum Existence {
    DoNotCare,
    Exists,
}

pub struct Index(pub usize);

pub struct Table {
    names: Vec<String>,
}

impl Table {
    pub fn new() -> Table {
        Table { names: Vec::new() }
    }

    pub fn str_to_index(&mut self, name: &str, existence: Existence) -> Result<Index, BadArgument> {
        let existing_position = self
            .names
            .iter()
            .position(|existing_name| existing_name == name);

        match (existing_position, existence) {
            (Some(position), _) => Ok(position),
            (None, Existence::DoNotCare) => {
                self.names.push(name.to_string());
                Ok(self.names.len() - 1)
            }
            (None, Existence::Exists) => Err(BadArgument),
        }
        .map(|found_or_existing_position| Index(found_or_existing_position))
    }

    pub fn name(&self, index: Index) -> String {
        self.names[index.0].clone()
    }
}
