use std::borrow::Borrow;

use cpython::*;
use linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlEmitter};

fn from_dict_to_yaml(py: Python, dict: &PyDict) -> Yaml {
    let items = dict.items(py);
    let mut hash = LinkedHashMap::new();

    for item in items {
        let k = &item.0;
        let v = &item.1;
        let key = from_python_to_yaml(py, &k);
        let value = from_python_to_yaml(py, &v);

        hash.insert(key, value);
    }

    Yaml::Hash(hash)
}

fn from_string_to_yaml(py: Python, py_str: &PyString) -> Yaml {
    let string = py_str.to_string_lossy(py);
    let borrowed_string: &str = string.borrow();

    Yaml::String(borrowed_string.into())
}

fn from_unicode_to_yaml(py: Python, py_str: &PyUnicode) -> Yaml {
    let string = py_str.to_string_lossy(py);
    let borrowed_string: &str = string.borrow();

    Yaml::String(borrowed_string.into())
}

fn from_int_to_yaml(py: Python, py_int: &PyInt) -> Yaml {
    let int: i64 = py_int.value(py);

    Yaml::Integer(int)
}

fn from_float_to_yaml(py: Python, py_float: &PyFloat) -> Yaml {
    let float: f64 = py_float.value(py);

    Yaml::Real(float.to_string())
}

fn from_bool_to_yaml(_: Python, py_bool: &PyBool) -> Yaml {
    Yaml::Boolean(py_bool.is_true())
}

fn from_list_to_yaml(py: Python, py_list: &PyList) -> Yaml {
    Yaml::Array(
        py_list
            .iter(py)
            .fold(Vec::with_capacity(py_list.len(py)), |mut acc, item| {
                acc.push(from_python_to_yaml(py, &item));

                acc
            }),
    )
}

fn from_python_to_yaml(py: Python, obj: &PyObject) -> Yaml {
    // Order matters
    if let Ok(list) = obj.cast_as::<PyList>(py) {
        return from_list_to_yaml(py, list);
    } else if let Ok(bool) = obj.cast_as::<PyBool>(py) {
        return from_bool_to_yaml(py, bool);
    } else if let Ok(int) = obj.cast_as::<PyInt>(py) {
        return from_int_to_yaml(py, int);
    } else if let Ok(str) = obj.cast_as::<PyString>(py) {
        return from_string_to_yaml(py, str);
    } else if let Ok(unicode) = obj.cast_as::<PyUnicode>(py) {
        return from_unicode_to_yaml(py, unicode);
    } else if let Ok(dict) = obj.cast_as::<PyDict>(py) {
        return from_dict_to_yaml(py, dict);
    } else if let Ok(float) = obj.cast_as::<PyFloat>(py) {
        return from_float_to_yaml(py, float);
    } else if *obj == Python::None(py) {
        return Yaml::Null;
    } else {
        println!("Unimplemented {:?}", obj.get_type(py).name(py));
        return Yaml::Null;
    }
}

pub fn safe_dump1(py: Python, py_data: &PyObject) -> PyResult<PyObject> {
    safe_dump2(py, py_data, None)
}

pub fn safe_dump2(
    py: Python,
    py_data: &PyObject,
    _stream: Option<&PyString>,
) -> PyResult<PyObject> {
    // Convert Python objects into Rust data types
    let data = from_python_to_yaml(py, &py_data);

    // Prepare buffer
    let mut buffer = String::new();

    // Dump data into YAML
    // wrapped into scope to allow to borrow the buffer later again to convert it into a PyString
    {
        let mut emitter = YamlEmitter::new(&mut buffer);

        match emitter.dump(&data) {
            Ok(_) => {}
            Err(e) => panic!("{:?}: {:?}", e, data),
        };
    }

    // Strip document header
    let doc: String = buffer.drain(4..).collect();

    // Convert into a PyString and return it
    let py_buffer = PyString::new(py, &doc);

    Ok(py_buffer.into_object())
}
