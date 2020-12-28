use crate::hashable::Hashable;
use crate::ruby::*;
use std::collections::HashMap;
use std::ffi;
use std::mem;
use std::ptr;

type Map = HashMap<Hashable, Value>;

pub fn define_ruby_class(name: &ffi::CStr, module: Option<Value>) -> Value {
    let super_: Value = unsafe { rb_cData };
    let klass: Value = match module {
        Some(m) => define_class_under(m, name, super_),
        None => define_class(name, super_),
    };
    #[cfg(feature = "method_cache")]
    unsafe {
        let sym_eq = module_eval(klass, "def self.values_eql?(a, b); a.eql?(b); end");
        M_EQ = obj_method_by_symbol(klass, sym_eq);
    }
    define_alloc_func(klass, alloc);
    define_method::<F1>(klass, "size", size);
    define_method::<F3>(klass, "get_with_hash", get_with_hash);
    define_method::<F4>(klass, "insert_with_hash", insert_with_hash);
    define_method::<F3>(klass, "remove_with_hash", remove_with_hash);
    define_method::<F1>(klass, "keys", keys);
    define_method::<F1>(klass, "values", values);
    return klass;
}

static TYPE_NAME: &[u8] = b"std::collections::HashMap\0";
static RUBY_TYPE: DataType = DataType {
    wrap_struct_name: TYPE_NAME.as_ptr(),
    function: DataTypeFunction {
        dmark: Some(mark),
        dfree: Some(free),
        dsize: Some(object_size),
        reserved: [0; 2],
    },
    parent: None as Option<ptr::NonNull<DataType>>,
    data: ptr::null(),
    flags: DataTypeFlag::FreeImmediately,
};

#[cfg(feature = "method_cache")]
pub static mut M_EQ: Value = NIL;

extern "C" fn mark(ptr: *const ffi::c_void) {
    let ptr = ptr as *const Map;
    let map = unsafe { &*ptr };
    for (&key, &val) in map.iter() {
        gc_mark(key.0);
        gc_mark(val);
    }
    #[cfg(feature = "method_cache")]
    unsafe {
        gc_mark(M_EQ);
    }
}

extern "C" fn free(ptr: *mut ffi::c_void) {
    unsafe { Box::from_raw(ptr as *mut Map) };
}

extern "C" fn object_size(ptr: *const ffi::c_void) -> usize {
    let ptr = ptr as *const Map;
    mem::size_of_val(unsafe { &*ptr })
}

extern "C" fn alloc(klass: Value) -> Value {
    let value = Box::new(HashMap::new());
    data_typed_object_wrap::<Map>(klass, value, &RUBY_TYPE)
}

extern "C" fn size(self_: Value) -> Value {
    let map: &Map = unsafe { &*check_typeddata(self_, &RUBY_TYPE) };
    int_to_value(map.len() as isize)
}

extern "C" fn get_with_hash(self_: Value, key: Value, hash: Value) -> Value {
    let map: &Map = unsafe { &*check_typeddata(self_, &RUBY_TYPE) };
    match map.get(&Hashable(key, value_to_int(hash))) {
        Some(val) => *val,
        None => NIL,
    }
}

extern "C" fn insert_with_hash(self_: Value, key: Value, hash: Value, value: Value) -> Value {
    let map: &mut Map = unsafe { &mut *check_typeddata(self_, &RUBY_TYPE) };
    map.insert(Hashable(key, value_to_int(hash)), value);
    return value;
}

extern "C" fn remove_with_hash(self_: Value, key: Value, hash: Value) -> Value {
    let map: &mut Map = unsafe { &mut *check_typeddata(self_, &RUBY_TYPE) };
    match map.remove(&Hashable(key, value_to_int(hash))) {
        Some(val) => val,
        None => NIL,
    }
}

extern "C" fn keys(self_: Value) -> Value {
    let map: &Map = unsafe { &*check_typeddata(self_, &RUBY_TYPE) };
    let ary = ary_new_capa(map.len());
    if map.len() > 0 {
        ary_store(ary, (map.len() - 1) as isize, NIL);
        let ptr = ary_ptr_use_start(ary);
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, map.len()) };
        for (item, (key, _)) in slice.iter_mut().zip(map.iter()) {
            *item = key.0;
        }
    }
    ary
}

extern "C" fn values(self_: Value) -> Value {
    let map: &Map = unsafe { &*check_typeddata(self_, &RUBY_TYPE) };
    let ary = ary_new_capa(map.len());
    if map.len() > 0 {
        ary_store(ary, (map.len() - 1) as isize, NIL);
        let ptr = ary_ptr_use_start(ary);
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, map.len()) };
        for (item, (_, value)) in slice.iter_mut().zip(map.iter()) {
            *item = *value;
        }
    }
    ary
}
