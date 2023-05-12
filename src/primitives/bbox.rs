use pyo3::{pyclass, pymethods, Py, PyAny};
use rkyv::{Archive, Deserialize, Serialize};

#[pyclass]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[archive(check_bytes)]
pub struct BBox {
    #[pyo3(get, set)]
    pub xc: f64,
    #[pyo3(get, set)]
    pub yc: f64,
    #[pyo3(get, set)]
    pub width: f64,
    #[pyo3(get, set)]
    pub height: f64,
    #[pyo3(get, set)]
    pub angle: Option<f64>,
}

#[pymethods]
impl BBox {
    #[classattr]
    const __hash__: Option<Py<PyAny>> = None;

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    #[new]
    pub fn new(xc: f64, yc: f64, width: f64, height: f64, angle: Option<f64>) -> Self {
        Self {
            xc,
            yc,
            width,
            height,
            angle,
        }
    }
}