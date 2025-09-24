use pyo3::prelude::*;

pub mod algorithms;
pub mod datastructures;

pub fn register_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let algorithms_mod = PyModule::new(m.py(), "algorithms")?;
    algorithms::register_algorithms(&algorithms_mod)?;
    m.add_submodule(&algorithms_mod)?;

    // m.py().import("sys")?.getattr("modules")?.set_item("rustdsa.algorithms", algorithms_mod)?;

    let datastructures_mod = PyModule::new(m.py(), "datastructures")?;
    datastructures::register_datastructures(&datastructures_mod)?;
    m.add_submodule(&datastructures_mod)?;

    // m
    //     .py()
    //     .import("sys")?
    //     .getattr("modules")?
    //     .set_item("rustdsa.datastructures", datastructures_mod)?;

    Ok(())
}
