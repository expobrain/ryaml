#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;
extern crate linked_hash_map;
extern crate yaml_rust;

mod load;
mod dump;

use cpython::{PyObject, PyString};

use load::*;
use dump::*;

py_module_initializer!(ryaml, initryaml, PyInit_ryaml, |py, m| {
    try!(m.add(py, "safe_load", py_fn!(py, safe_load(stream: PyString))));
    try!(m.add(py, "safe_dump", py_fn!(py, safe_dump2(data: PyObject, stream: Option<PyString>))));
    try!(m.add(py, "safe_dump", py_fn!(py, safe_dump1(data: PyObject))));
    Ok(())
});
