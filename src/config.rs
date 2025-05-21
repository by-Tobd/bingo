use envconfig::Envconfig;

#[derive(Envconfig, Clone, Debug)]
pub struct Config {
    #[envconfig(from = "DB_HOST", default = "sqlite::memory:")]
    pub db_host: String,

    #[envconfig(from = "DB_NAME", default = "bingo")]
    pub db_name: String,

	#[envconfig(from = "BIND_ADDR", default = "0.0.0.0")]
	pub bind_addr: String,

	#[envconfig(from = "PORT", default = "8080")]
	pub port: u16,

    #[envconfig(from = "MIN_PASSWORD_LENGTH", default = "8")]
    pub min_password_length: usize,

    #[envconfig(from = "JWT_SECRET", default = "unsecure")]
    pub jwt_secret: String
}
