use std::error::Error;

pub mod types;

pub fn get_app_config() -> Result<types::Config, Box<dyn Error>> {
    Ok(Default::default())
}
