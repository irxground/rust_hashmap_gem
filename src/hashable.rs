use crate::ruby;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Hashable(pub ruby::Value, pub isize);

impl PartialEq for Hashable {
    fn eq(&self, other: &Self) -> bool {
        let raw = self.0.to_raw() as usize;
        if raw & ruby::IMMEDIATE_MASK == 0 || raw & ruby::FLONUM_MASK == ruby::FLONUM_FLAG {
            eq(self.0, other.0) == ruby::TRUE
        } else {
            self.0 == other.0
        }
    }
}
impl Eq for Hashable {}

#[cfg(feature = "method_cache")]
#[inline]
fn eq(left: ruby::Value, right: ruby::Value) -> ruby::Value {
    let method = unsafe { crate::hashmap::M_EQ };
    ruby::method_call(method, &[left, right])
}

#[cfg(not(feature = "method_cache"))]
#[inline]
fn eq(left: ruby::Value, right: ruby::Value) -> ruby::Value {
    ruby::fun_call(left, "eql?", &[right])
}

impl Hash for Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}
