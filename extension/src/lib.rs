#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate cpython;
extern crate yaml_rust;

use std::collections::BTreeMap;
use cpython::{PyString, Python, PyResult, PyObject, PyDict, PyList, PythonObject, ToPyObject};
use yaml_rust::{Yaml, YamlLoader};

fn convert_yaml_to_dict(py: Python, yaml: &BTreeMap<Yaml, Yaml>) -> PyDict {
    let dict = PyDict::new(py);

    for (k, v) in yaml.iter() {
        let key: PyObject = match from_yaml_to_python(py, k) {
            Some(o) => o,
            None => panic!("Error converting key {:?}", k)
        };
        let value: PyObject = match from_yaml_to_python(py, v) {
            Some(o) => o,
            None => panic!("Error converting value {:?}", v)
        };

        let _ = dict.set_item(py, key, value);
    }

    dict
}

fn convert_yaml_to_list(py: Python, yaml: &Vec<Yaml>) -> PyList {
    let vec: Vec<PyObject> = yaml.iter()
        .map(|e| from_yaml_to_python(py, e).unwrap())
        .collect();

    PyList::new(py, &vec)
}


fn from_yaml_to_python(py: Python, yaml: &Yaml) -> Option<PyObject> {
    // None
    if yaml.is_null() {
        return Some(py.None())
    }

    // Hash
    let hash = yaml.as_hash();
    if hash.is_some() {
        let dict = convert_yaml_to_dict(py, hash.unwrap());
        return Some(dict.into_object())
    }

    // String
    let string = yaml.as_str();
    if string.is_some() {
        let pystring = PyString::new(py, string.unwrap());
        return Some(pystring.into_object())
    }

    // Integer
    let int = yaml.as_i64();
    if int.is_some() {
        return Some(int.to_py_object(py).into_object())
    }

    // List
    let vec = yaml.as_vec();
    if vec.is_some() {
        let list = convert_yaml_to_list(py, vec.unwrap());
        return Some(list.into_object())
    }

    // Float
    let float = yaml.as_f64();
    if float.is_some() {
        return Some(float.to_py_object(py).into_object())
    }

    // Boolean
    let boolean = yaml.as_bool();
    if boolean.is_some() {
        match boolean.unwrap() {
            true => return Some(py.True().into_object()),
            false => return Some(py.False().into_object())
        }
    }

    // Default
    return None
}

fn safe_load(py: Python, stream: PyString) -> PyResult<PyObject> {
    // Convert stream into Rust string
    let native_stream = match stream.to_string(py) {
        Ok(s) => s,
        Err(e) => panic!("Cannot convert Python string into &str: {:?}", e)
    };

    // // Load first doc from stream
    let docs = match YamlLoader::load_from_str(&native_stream) {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e)
    };
    let doc = &docs[0];

    // Convert into proper Python's objects
    let result: PyObject = match from_yaml_to_python(py, doc) {
        Some(o) => o,
        None => py.None()
    };

    Ok(result)
}

py_module_initializer!(ryaml, initryaml, PyInit_ryaml, |py, m| {
    try!(m.add(py, "safe_load", py_fn!(py, safe_load(stream: PyString))));
    Ok(())
});
