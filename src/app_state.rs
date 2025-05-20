use crate::{config::Config, model::database::Database};

#[derive(Clone)]
pub struct AppData {
    pub db: Database,
    pub _config: Config
}