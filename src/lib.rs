use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use pyo3::exceptions::PyIOError;
use pyo3::{prelude::*, PyObjectProtocol, PySequenceProtocol};

use memmap::Mmap;

use fst::IntoStreamer;
use fst::Streamer;
use fst::{Map, Set};
use regex_automata::dense;

use indexmap::IndexMap;

#[pyclass]
struct FstSet {
    file_name: Box<str>,
    set: Set<Mmap>,
}

#[pymethods]
impl FstSet {
    #[new]
    fn new(file_name: String) -> PyResult<Self> {
        let file_path = Path::new(file_name.as_str());
        match File::open(file_path) {
            Err(_e) => {
                return Err(PyIOError::new_err(
                    "Error opening ".to_string() + &file_name,
                ))
            }
            Ok(file_handle) => {
                let mmap = unsafe { Mmap::map(&file_handle).unwrap() };
                let set = Set::new(mmap).unwrap();
                return Ok(FstSet {
                    file_name: file_name.into_boxed_str(),
                    set,
                });
            }
        };
    }

    #[getter]
    fn file_name(self_: PyRef<Self>) -> PyResult<String> {
        return Ok(String::from_str(&*self_.file_name)?);
    }

    fn contains(self_: PyRef<Self>, key: String) -> bool {
        return self_.set.contains(key);
    }

    fn search(self_: PyRef<Self>, pattern: String) -> PyResult<Vec<String>> {
        let dfa = dense::Builder::new()
            .anchored(true)
            .build(pattern.as_str())
            .unwrap();
        let mut stream = self_.set.search(&dfa).into_stream();
        let mut result: Vec<String> = Vec::new();
        while let Some(k) = stream.next() {
            result.push(String::from_utf8(k.to_vec())?);
        }
        Ok(result)
    }
}

#[pyproto]
impl PyObjectProtocol for FstSet {
    fn __repr__(&self) -> String {
        return format!(
            "<FstSet file_name={} length={}>",
            self.file_name,
            self.set.len()
        );
    }
}

#[pyproto]
impl PySequenceProtocol for FstSet {
    fn __len__(&'p self) -> usize {
        return self.set.len();
    }
}

#[pyclass]
struct FstMap {
    file_name: Box<str>,
    map: Map<Mmap>,
}

#[pymethods]
impl FstMap {
    #[new]
    fn new(file_name: String) -> PyResult<Self> {
        let file_path = Path::new(file_name.as_str());
        match File::open(file_path) {
            Err(_e) => {
                return Err(PyIOError::new_err(
                    "Error opening ".to_string() + &file_name,
                ))
            }
            Ok(file_handle) => {
                let mmap = unsafe { Mmap::map(&file_handle).unwrap() };
                let map = Map::new(mmap).unwrap();
                return Ok(FstMap {
                    file_name: file_name.into_boxed_str(),
                    map,
                });
            }
        };
    }

    fn get(self_: PyRef<Self>, key: String, default: Option<u64>) -> PyResult<Option<u64>> {
        match self_.map.get(key) {
            Some(value) => return Ok(Some(value)),
            None => {
                return Ok(default);
            }
        }
    }

    fn search(self_: PyRef<Self>, pattern: String) -> PyResult<IndexMap<String, u64>> {
        let dfa = dense::Builder::new()
            .anchored(true)
            .build(pattern.as_str())
            .unwrap();
        let mut stream = self_.map.search(&dfa).into_stream();
        let mut result = IndexMap::new();
        while let Some((k, v)) = stream.next() {
            result.insert(String::from_utf8(k.to_vec())?, v);
        }
        Ok(result)
    }
}

#[pyproto]
impl PyObjectProtocol for FstMap {
    fn __repr__(&self) -> String {
        return format!(
            "<FstMap file_name={} length={}>",
            self.file_name,
            self.map.len()
        );
    }
}

#[pyproto]
impl PySequenceProtocol for FstMap {
    fn __len__(&'p self) -> usize {
        return self.map.len();
    }
}

#[pymodule]
fn pyfst(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FstMap>()?;
    m.add_class::<FstSet>()?;

    Ok(())
}
