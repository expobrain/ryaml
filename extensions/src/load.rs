use std::collections::BTreeMap;
use cpython::{PyString, Python, PyResult, PyObject, PyDict, PyList, PythonObject, ToPyObject};
use yaml_rust::{Yaml, YamlLoader};

fn convert_yaml_to_dict(py: Python, yaml: &BTreeMap<Yaml, Yaml>) -> PyDict {
    let dict = PyDict::new(py);

    for (k, v) in yaml.iter() {
        let key = from_yaml_to_python(py, k);
        let value = from_yaml_to_python(py, v);
        let _ = dict.set_item(py, key, value);
    }

    dict
}

fn convert_yaml_to_list(py: Python, yaml: &Vec<Yaml>) -> PyList {
    let vec: Vec<PyObject> = yaml.iter().map(|e| from_yaml_to_python(py, e)).collect();

    PyList::new(py, &vec)
}


fn from_yaml_to_python(py: Python, yaml: &Yaml) -> PyObject {
    match yaml {
        &Yaml::Null => py.None(),
        &Yaml::Hash(_) => convert_yaml_to_dict(py, yaml.as_hash().unwrap()).into_object(),
        &Yaml::String(_) => PyString::new(py, yaml.as_str().unwrap()).into_object(),
        &Yaml::Integer(_) => yaml.as_i64().unwrap().to_py_object(py).into_object(),
        &Yaml::Real(_) => yaml.as_f64().unwrap().to_py_object(py).into_object(),
        &Yaml::Array(_) => convert_yaml_to_list(py, yaml.as_vec().unwrap()).into_object(),
        &Yaml::Boolean(b) => match b {
            true => py.True().into_object(),
            false => py.False().into_object()
        },
        &Yaml::Alias(_) => unimplemented!(),  // Not supported yet http://chyh1990.github.io/yaml-rust/doc/yaml_rust/yaml/enum.Yaml.html#variant.Alias
        &Yaml::BadValue => panic!("Bad value converting {:?}", yaml)
    }
}

pub fn safe_load(py: Python, stream: PyString) -> PyResult<PyObject> {
    // Convert stream into Rust string
    let native_stream = match stream.to_string(py) {
        Ok(s) => s,
        Err(e) => panic!("Cannot convert Python string into &str: {:?}", e)
    };

    // Load first doc from stream
    let docs = match YamlLoader::load_from_str(&native_stream) {
        Ok(d) => d,
        Err(e) => panic!("{:?}", e)
    };
    let doc = &docs[0];

    // Convert into proper Python's objects
    let result = from_yaml_to_python(py, doc);

    Ok(result)
}
