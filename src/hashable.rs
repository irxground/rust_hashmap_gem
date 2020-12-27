use crate::ruby;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Hashable(pub ruby::Value);

impl PartialEq for Hashable {
    #[cfg(feature = "method_cache")]
    fn eq(&self, other: &Self) -> bool {
        let method = unsafe { crate::hashmap::M_EQ };
        let val = ruby::method_call(method, &[self.0, other.0]);
        val == ruby::TRUE
    }

    #[cfg(not(feature = "method_cache"))]
    fn eq(&self, other: &Self) -> bool {
        let val = ruby::fun_call(self.0, "eql?", &[other.0]);
        val == ruby::TRUE
    }
}
impl Eq for Hashable {}

impl Hash for Hashable {
    #[cfg(feature = "method_cache")]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let method = unsafe { crate::hashmap::M_HASH };
        let val = ruby::method_call(method, &[self.0]);
        ruby::value_to_int(val).hash(state);
    }

    #[cfg(not(feature = "method_cache"))]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let val = ruby::fun_call(self.0, "hash", &[]);
        ruby::value_to_int(val).hash(state);
    }
}
