use sea_orm_migration::{MigrationTrait, MigratorTrait};

use crate::model::migrations::m0001_initialize;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m0001_initialize::Migration)]
    }
}