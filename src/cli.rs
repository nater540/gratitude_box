use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "Gratitude Box Server", version = "0.0.1")]
pub struct Args {
  #[arg(long = "db-host", env = "GB_DB_HOST")]
  pub db_host: String,

  #[arg(long = "db-user", env = "GB_DB_USER")]
  pub db_user: String,

  #[arg(long = "db-pass", env = "GB_DB_PASS")]
  pub db_pass: String,

  #[arg(long = "db-name", env = "GB_DB_NAME")]
  pub db_name: String
}

pub fn parse() -> Args {
  Args::parse()
}
