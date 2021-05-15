use pyo3::prelude::*;
use pyo3::types::{PyTuple};
use pyo3::wrap_pyfunction;
use pyo3::exceptions::{PyValueError,PyRuntimeError};

use randomprime;

use randomprime::patch_config::PatchConfig;

struct ProgressNotifier
{
    python_object: PyObject
}

impl ProgressNotifier
{
    fn new(python_object: PyObject) -> ProgressNotifier
    {
        ProgressNotifier {
            python_object: python_object,
        }
    }
    
    fn handle_pyresult(&mut self, py: Python, result: PyResult<PyObject>)
    {
        if !result.is_ok() {
            result.unwrap_err().print(py)
        }
    }
}

impl randomprime::structs::ProgressNotifier for ProgressNotifier
{
    fn notify_total_bytes(&mut self, total_size: usize)
    {
        let gil = Python::acquire_gil(); 
        let py = gil.python();
        let args = PyTuple::new(py, &[total_size]);
        self.handle_pyresult(py, self.python_object.call_method1(py, "notify_total_bytes", args));
    }

    fn notify_writing_file(&mut self, file_name: &randomprime::reader_writer::CStr, file_bytes: usize)
    {
        let gil = Python::acquire_gil(); 
        let py = gil.python();
        let args = (file_name.to_bytes(), file_bytes);
        self.handle_pyresult(py, self.python_object.call_method1(py, "notify_writing_file", args));
    }

    fn notify_writing_header(&mut self)
    {
        let gil = Python::acquire_gil(); 
        let py = gil.python();
        self.handle_pyresult(py, self.python_object.call_method0(py, "notify_writing_header"));
    }

    fn notify_flushing_to_disk(&mut self)
    {
        let gil = Python::acquire_gil(); 
        let py = gil.python();
        self.handle_pyresult(py, self.python_object.call_method0(py, "notify_flushing_to_disk"));
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
#[text_signature = "(config_json, progress_notifier, /)"]
fn patch_iso(config_json: String, progress_notifier: PyObject) -> PyResult<()> {
    let patch_config = PatchConfig::from_json(config_json.as_str())
        .map_err(|e| PyValueError::new_err(e))?;
    
    let pn = ProgressNotifier::new(progress_notifier);
    randomprime::patches::patch_iso(patch_config, pn)
        .map_err(|e| PyRuntimeError::new_err(e))?;

    Ok(())
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(patch_iso, m)?)?;

    Ok(())
}