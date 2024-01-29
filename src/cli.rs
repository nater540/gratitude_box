use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "Gratitude Box Server", version = "0.0.1")]
pub struct Args {
  #[arg(long = "db-host", env = "GB_DB_HOST")]
  pub db_host: String,

  #[arg(long = "db-port", env = "GB_DB_PORT", default_value = "5432")]
  pub db_port: String,

  #[arg(long = "db-user", env = "GB_DB_USER")]
  pub db_user: Option<String>,

  #[arg(long = "db-pass", env = "GB_DB_PASS")]
  pub db_pass: Option<String>,

  #[arg(long = "db-name", env = "GB_DB_NAME")]
  pub db_name: String,

  #[arg(long = "db-pool-min", env = "GB_DB_POOL_MIN", default_value = "8")]
  pub db_pool_min: u32,

  #[arg(long = "db-pool-max", env = "GB_DB_POOL_MAX", default_value = "10")]
  pub db_pool_max: u32,

  #[arg(long = "http-host", env = "GB_HTTP_HOST", default_value = "0.0.0.0")]
  pub http_host: String,

  #[arg(long = "http-port", env = "GB_HTTP_PORT", default_value = "3000")]
  pub http_port: u16,
}

pub fn parse() -> Args {
  Args::parse()
}
