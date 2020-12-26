use crate::ruby;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Hashable(pub ruby::Value);

impl PartialEq for Hashable {
    fn eq(&self, other: &Self) -> bool {
        let val = ruby::fun_call(self.0, "eql?", &[other.0]);
        val != ruby::FALSE
    }
}
impl Eq for Hashable {}

impl Hash for Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let val = ruby::fun_call(self.0, "hash", &[]);
        val.to_raw().hash(state);
    }
}
