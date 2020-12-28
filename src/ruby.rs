use std::ffi;
use std::fmt;
use std::ptr;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Value(isize);

impl Value {
    #[inline]
    pub fn to_raw(self) -> isize {
        self.0
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RbValue").field(&object_id(*self)).finish()
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Id(usize);

#[repr(C)]
pub struct DataTypeFunction {
    pub dmark: Option<extern "C" fn(_: *const ffi::c_void)>,
    pub dfree: Option<extern "C" fn(_: *mut ffi::c_void)>,
    pub dsize: Option<extern "C" fn(_: *const ffi::c_void) -> usize>,
    pub reserved: [usize; 2],
}

#[repr(usize)]
pub enum DataTypeFlag {
    FreeImmediately = 1,
}

#[repr(C)]
pub struct DataType {
    pub wrap_struct_name: *const u8,
    pub function: DataTypeFunction,
    pub parent: Option<ptr::NonNull<DataType>>,
    pub data: *const ffi::c_void,
    pub flags: DataTypeFlag,
}
unsafe impl Sync for DataType {}

pub type F1 = extern "C" fn(Value) -> Value;
pub type F2 = extern "C" fn(Value, Value) -> Value;
pub type F3 = extern "C" fn(Value, Value, Value) -> Value;
pub type F4 = extern "C" fn(Value, Value, Value, Value) -> Value;

pub trait RubyFunc {
    fn to_raw(self) -> (*const ffi::c_void, usize);
}

impl RubyFunc for F1 {
    #[inline]
    fn to_raw(self) -> (*const ffi::c_void, usize) {
        (self as _, 0)
    }
}

impl RubyFunc for F2 {
    #[inline]
    fn to_raw(self) -> (*const ffi::c_void, usize) {
        (self as _, 1)
    }
}

impl RubyFunc for F3 {
    #[inline]
    fn to_raw(self) -> (*const ffi::c_void, usize) {
        (self as _, 2)
    }
}

impl RubyFunc for F4 {
    #[inline]
    fn to_raw(self) -> (*const ffi::c_void, usize) {
        (self as _, 3)
    }
}

#[cfg(target_pointer_width = "32")]
mod consts {
    pub const FALSE: isize = 0;
    pub const TRUE: isize = 2;
    pub const NIL: isize = 4;
}

#[cfg(target_pointer_width = "64")]
mod consts {
    pub const FALSE: isize = 0;
    pub const TRUE: isize = 0x14;
    pub const NIL: isize = 0x08;
}

pub const FALSE: Value = Value(consts::FALSE);
pub const TRUE: Value = Value(consts::TRUE);
pub const NIL: Value = Value(consts::NIL);

#[cfg(target_pointer_width = "32")]
mod mask {
    pub const IMMEDIATE_MASK: usize = 0x07;
    pub const FIXNUM_FLAG: usize = 0x01;
    pub const FLONUM_MASK: usize = 0x03;
    pub const FLONUM_FLAG: usize = 0x02;
    pub const SYMBOL_FLAG: usize = 0x0c;
}
#[cfg(target_pointer_width = "64")]
mod mask {
    pub const IMMEDIATE_MASK: usize = 0x03;
    pub const FIXNUM_FLAG: usize = 0x01;
    pub const FLONUM_MASK: usize = 0x00;
    pub const FLONUM_FLAG: usize = 0x02;
    pub const SYMBOL_FLAG: usize = 0x0e;
}
pub use mask::*;

extern "C" {
    #[no_mangle]
    pub static rb_cData: Value;

    // void rb_gc_mark(VALUE ptr)
    fn rb_gc_mark(ptr: Value);

    // VALUE rb_obj_id(VALUE obj)
    fn rb_obj_id(obj: Value) -> Value;

    // VALUE rb_define_module(const char *name)
    fn rb_define_module(name: *const i8) -> Value;

    // VALUE rb_define_class(const char *name, VALUE super)
    fn rb_define_class(name: *const i8, super_: Value) -> Value;

    // VALUE rb_define_class_under(VALUE outer, const char *name, VALUE super)
    fn rb_define_class_under(outer: Value, name: *const i8, super_: Value) -> Value;

    // void rb_define_const(VALUE klass, const char *name, VALUE val)
    fn rb_define_const(klass: Value, name: *const i8, val: Value);

    // VALUE rb_str_new(const char *ptr, long len)
    fn rb_str_new(ptr: *const u8, len: usize) -> Value;

    // void rb_define_alloc_func(VALUE klass, VALUE (*func)(VALUE))
    fn rb_define_alloc_func(klass: Value, func: extern "C" fn(Value) -> Value);

    // VALUE rb_data_typed_object_wrap(VALUE klass, void *datap, const rb_data_type_t *type)
    fn rb_data_typed_object_wrap(
        klass: Value,
        datap: *mut ffi::c_void,
        type_: *const DataType,
    ) -> Value;

    // void * rb_check_typeddata(VALUE obj, const rb_data_type_t *data_type)
    fn rb_check_typeddata(obj: Value, data_type: *const DataType) -> *mut ffi::c_void;

    // ID rb_intern2(const char *name, long len)
    fn rb_intern2(name: *const u8, len: usize) -> Id;

    // VALUE rb_id2sym(ID x)
    fn rb_id2sym(x: Id) -> Value;

    // void rb_define_method_id(VALUE klass, ID mid, VALUE (*func)(ANYARGS), int argc)
    fn rb_define_method_id(klass: Value, mid: Id, func: *const ffi::c_void, argc: i32);

    // VALUE rb_funcallv(VALUE recv, ID mid, int argc, const VALUE *argv)
    fn rb_funcallv(recv: Value, mid: Id, argc: i32, argv: *const Value) -> Value;

    // VALUE rb_ary_new_capa(long capa)
    fn rb_ary_new_capa(capa: usize) -> Value;

    // VALUE rb_ary_push(VALUE ary, VALUE item)
    fn rb_ary_push(ary: Value, item: Value) -> Value;

    // void rb_ary_store(VALUE ary, long idx, VALUE val)
    fn rb_ary_store(ary: Value, idx: isize, val: Value);

    // VALUE *rb_ary_ptr_use_start(VALUE ary)
    fn rb_ary_ptr_use_start(ary: Value) -> *mut Value;

    // VALUE rb_obj_method(VALUE obj, VALUE vid)
    fn rb_obj_method(obj: Value, vid: Value) -> Value;

    // VALUE rb_method_call(int argc, const VALUE *argv, VALUE method)
    fn rb_method_call(argc: i32, argv: *const Value, method: Value) -> Value;

    // long rb_num2long(VALUE val)
    fn rb_num2long(val: Value) -> isize;

    // VALUE rb_mod_module_eval(int argc, const VALUE *argv, VALUE mod)
    fn rb_mod_module_eval(argc: i32, argv: *const Value, module: Value) -> Value;
}

#[inline]
pub fn gc_mark(obj: Value) {
    unsafe { rb_gc_mark(obj) }
}

#[inline]
pub fn object_id(obj: Value) -> isize {
    value_to_int(unsafe { rb_obj_id(obj) })
}

#[inline]
pub fn define_module(name: &ffi::CStr) -> Value {
    unsafe { rb_define_module(name.as_ptr()) }
}

#[inline]
pub fn define_class(name: &ffi::CStr, super_: Value) -> Value {
    unsafe { rb_define_class(name.as_ptr(), super_) }
}

#[inline]
pub fn define_class_under(outer: Value, name: &ffi::CStr, super_: Value) -> Value {
    unsafe { rb_define_class_under(outer, name.as_ptr(), super_) }
}

#[inline]
pub fn define_const(klass: Value, name: &ffi::CStr, val: Value) {
    unsafe { rb_define_const(klass, name.as_ptr(), val) }
}

#[inline]
pub fn new_string(str: &str) -> Value {
    unsafe { rb_str_new(str.as_ptr(), str.len()) }
}

#[inline]
pub fn define_alloc_func(klass: Value, func: extern "C" fn(Value) -> Value) {
    unsafe { rb_define_alloc_func(klass, func) }
}

#[inline]
pub fn data_typed_object_wrap<T>(klass: Value, data: Box<T>, type_: &'static DataType) -> Value {
    let data = Box::into_raw(data) as *mut ffi::c_void;
    unsafe { rb_data_typed_object_wrap(klass, data, type_) }
}

#[inline]
pub unsafe fn check_typeddata<'a, T>(obj: Value, data_type: &'static DataType) -> *mut T {
    let ptr = rb_check_typeddata(obj, data_type);
    return ptr as *mut T;
}

#[inline]
pub fn define_method<F: RubyFunc>(klass: Value, name: &str, func: F) {
    unsafe {
        let (ptr, len) = func.to_raw();
        let id = rb_intern2(name.as_ptr(), name.len());
        rb_define_method_id(klass, id, ptr, len as i32);
    }
}

#[inline]
pub fn fun_call(self_: Value, method: &str, args: &[Value]) -> Value {
    unsafe {
        let id = rb_intern2(method.as_ptr(), method.len());
        rb_funcallv(self_, id, args.len() as i32, args.as_ptr())
    }
}

#[inline]
pub fn int_to_value(value: isize) -> Value {
    Value((value << 1) + 1)
}

#[inline]
pub fn value_to_int(value: Value) -> isize {
    if (value.to_raw() as usize) & 1 == 1 {
        // fast path
        value.to_raw() >> 1
    } else {
        unsafe { rb_num2long(value) }
    }
}

#[inline]
pub fn ary_new_capa(len: usize) -> Value {
    unsafe { rb_ary_new_capa(len) }
}

#[inline]
pub fn ary_push(ary: Value, item: Value) -> Value {
    unsafe { rb_ary_push(ary, item) }
}

#[inline]
pub fn ary_store(ary: Value, idx: isize, item: Value) {
    unsafe { rb_ary_store(ary, idx, item) }
}

#[inline]
pub fn ary_ptr_use_start(ary: Value) -> *mut Value {
    unsafe { rb_ary_ptr_use_start(ary) }
}

#[inline]
pub fn obj_method(obj: Value, name: &str) -> Value {
    unsafe {
        let id = rb_intern2(name.as_ptr(), name.len());
        rb_obj_method(obj, rb_id2sym(id))
    }
}

#[inline]
pub fn obj_method_by_symbol(obj: Value, name: Value) -> Value {
    unsafe { rb_obj_method(obj, name) }
}

#[inline]
pub fn method_call(method: Value, args: &[Value]) -> Value {
    unsafe { rb_method_call(args.len() as i32, args.as_ptr(), method) }
}

pub fn module_eval(module: Value, code: &str) -> Value {
    let s = new_string(code);
    let args = [s];
    unsafe { rb_mod_module_eval(args.len() as i32, args.as_ptr(), module) }
}
