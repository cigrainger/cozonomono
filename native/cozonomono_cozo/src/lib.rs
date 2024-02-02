mod datatypes;
mod error;

pub use datatypes::{ExDbInstance, ExDbInstanceRef, ExNamedRows};
pub use error::ExError;
use rustler::{Env, Term};

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(ExDbInstanceRef, env);
    rustler::resource!(ExDbInstance, env);
    true
}

#[rustler::nif(schedule = "DirtyCpu")]
fn create_instance(engine: String, path: String) -> Result<ExDbInstance, ExError> {
    let instance = cozo::DbInstance::new(&engine, &path, "")?;
    Ok(ExDbInstance::new(instance, engine, path))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn run_default(instance: ExDbInstance, payload: String) -> Result<ExNamedRows, ExError> {
    Ok(ExNamedRows(instance.run_default(&payload)?))
}

rustler::init!(
    "Elixir.Cozonomono.Native",
    [create_instance, run_default],
    load = on_load
);
