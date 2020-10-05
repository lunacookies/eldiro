use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<String, Val>,
}

impl Env {
    pub(crate) fn store_binding(&mut self, name: String, val: Val) {
        self.bindings.insert(name, val);
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or_else(|| format!("binding with name ‘{}’ does not exist", name))
    }
}
