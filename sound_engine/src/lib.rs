mod player;
mod sound;

use pyo3::prelude::*;

#[pymodule]
fn sound_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<player::AudioPlayer>()?;
    m.add_class::<sound::Sound>()?;

    Ok(())
}
