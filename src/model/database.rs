use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, Statement};
use sea_orm_migration::MigratorTrait;

use crate::config::Config;

use super::migrations::migrator::Migrator;

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: DatabaseConnection
}

impl Database {
    pub async fn new(config: &Config) -> Result<Database, DbErr> {
        println!("{:?}", config);
        let db = sea_orm::Database::connect(&config.db_host).await.unwrap();

        let db = match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", config.db_name),
                ))
                .await?;

                let url = format!("{}/{}", config.db_host, config.db_name);
                sea_orm::Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", config.db_name),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", config.db_name),
                ))
                .await?;

                let url = format!("{}/{}", config.db_host, config.db_name);
                sea_orm::Database::connect(&url).await?
            }
            DbBackend::Sqlite => db,
        };

        Migrator::up(&db, None).await?;

        Ok(Database {
            connection: db    
        })
    }
}
