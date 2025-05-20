use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,

	#[envconfig(from = "BIND_ADDR", default = "0.0.0.0")]
	pub bind_addr: String,

	#[envconfig(from = "PORT", default = "8080")]
	pub port: u16,
}
