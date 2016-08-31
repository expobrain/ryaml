#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;

use cpython::{PyUnicode, Python, PyResult};

pub fn hello(py: Python) -> PyResult<PyUnicode> {
    return Ok(PyUnicode::new(py, "Hello"))
}

fn val(_: Python) -> PyResult<i32> {
    Ok(42)
}

py_module_initializer!(ryaml, |py, m| {
    // try!(m.add(py, "hello", py_fn!(py, hello())));
    try!(m.add(py, "val", py_fn!(val())));
    Ok(())
});
