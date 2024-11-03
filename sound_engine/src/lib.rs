mod device;
mod engine;
mod player;

use pyo3::prelude::*;

#[pymodule]
fn sound_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<engine::AudioEngine>()?;
    m.add_class::<player::AudioPlayer>()?;

    Ok(())
}
