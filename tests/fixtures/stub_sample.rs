//! Sample file for stub detection testing.

use std::fmt;

// TODO: Implement proper error handling
pub fn process_data(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    todo!()
}

// FIXME: This is a workaround
pub fn workaround() -> i32 {
    42
}

// HACK: Temporary solution until upstream fix
pub fn temp_solution() {
    unimplemented!()
}

/// Default implementation - subtraits should override.
pub trait Processor {
    /// Process a single item.
    fn process(&self, item: &str) -> String {
        unimplemented!()
    }
}

pub fn handle_status(code: u32) -> &'static str {
    match code {
        200 => "OK",
        404 => "Not Found",
        _ => unreachable!(),
    }
}
