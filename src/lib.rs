use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

#[repr(C)]
pub struct PyObject {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn Py_BuildValue(format: *const c_char, ...) -> *mut PyObject;
    fn PyArg_ParseTuple(args: *mut PyObject, format: *const c_char, ...) -> c_int;
}

// Python-visible function: add(a, b)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn add(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let mut a: c_int = 0;
    let mut b: c_int = 0;

    let fmt = CString::new("ii").unwrap(); // expects 2 integers
    let ok = unsafe { PyArg_ParseTuple(args, fmt.as_ptr(), &mut a, &mut b) };

    if ok == 0 {
        return ptr::null_mut(); // Python will raise TypeError
    }

    let result = a + b;
    let fmt = CString::new("i").unwrap(); // returns an integer
    unsafe { Py_BuildValue(fmt.as_ptr(), result) }
}

#[repr(C)]
pub struct PyMethodDef {
    ml_name: *const c_char,
    ml_meth: Option<unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject>,
    ml_flags: c_int,
    ml_doc: *const c_char,
}

#[repr(C)]
pub struct PyModuleDefBase {
    ob_refcnt: isize,
    ob_type: *mut (),
    m_init: Option<unsafe extern "C" fn() -> *mut PyObject>,
    m_index: isize,
    m_copy: *mut PyObject,
}

#[repr(C)]
pub struct PyModuleDef {
    m_base: PyModuleDefBase,
    m_name: *const c_char,
    m_doc: *const c_char,
    m_size: isize,
    m_methods: *const PyMethodDef,
    m_slots: *mut (),
    m_traverse: Option<unsafe extern "C" fn()>,
    m_clear: Option<unsafe extern "C" fn()>,
    m_free: Option<unsafe extern "C" fn()>,
}

const METH_VARARGS: c_int = 0x0001;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PyInit_rawffi() -> *mut PyObject {
    static mut METHODS: [PyMethodDef; 2] = [
        PyMethodDef {
            ml_name: b"add\0".as_ptr() as *const c_char,
            ml_meth: Some(add),
            ml_flags: METH_VARARGS,
            ml_doc: b"Add two integers\0".as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: ptr::null(),
            ml_meth: None,
            ml_flags: 0,
            ml_doc: ptr::null(),
        },
    ];

    static mut MODULE_DEF: PyModuleDef = PyModuleDef {
        m_base: PyModuleDefBase {
            ob_refcnt: 1,
            ob_type: ptr::null_mut(),
            m_init: None,
            m_index: 0,
            m_copy: ptr::null_mut(),
        },
        m_name: b"rawffi\0".as_ptr() as *const c_char,
        m_doc: b"Raw Rust Python bindings\0".as_ptr() as *const c_char,
        m_size: -1,
        m_methods: ptr::addr_of!(METHODS).cast(),
        m_slots: ptr::null_mut(),
        m_traverse: None,
        m_clear: None,
        m_free: None,
    };

    unsafe extern "C" {
        fn PyModule_Create2(def: *mut PyModuleDef, module_api_version: c_int) -> *mut PyObject;
    }

    unsafe { PyModule_Create2(&raw mut MODULE_DEF, 1013) }
}
