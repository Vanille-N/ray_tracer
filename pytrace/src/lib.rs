use pyo3::prelude::*;
use pytrace_core::internal;

mod external;

#[pymodule]
fn pytrace(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<external::Cfg>().unwrap();
    m.add_class::<external::Camera>().unwrap();
    m.add_class::<external::Vec>().unwrap();
    m.add_class::<external::Sky>().unwrap();
    m.add_class::<external::RGB>().unwrap();
    m.add_class::<external::Texture>().unwrap();
    m.add_class::<external::Sphere>().unwrap();
    m.add_class::<external::InfinitePlane>().unwrap();
    m.add_class::<external::Triangle>().unwrap();
    m.add_class::<external::Parallelogram>().unwrap();
    m.add_class::<external::Rhomboid>().unwrap();
    m.add_class::<external::EmptyCylinder>().unwrap();
    m.add_class::<external::Disc>().unwrap();
    m.add_class::<external::Cylinder>().unwrap();
    m.add_class::<external::EmptyCone>().unwrap();
    m.add_class::<external::Cone>().unwrap();
    m.add_class::<external::Axes>().unwrap();
    m.add_class::<external::Cradle>().unwrap();
    Ok(())
}
