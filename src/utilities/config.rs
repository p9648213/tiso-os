#[derive(clap::Parser, Clone, Debug)]
pub struct EnvConfig {
    #[clap(long, env)]
    pub pg_host: String,
    #[clap(long, env)]
    pub pg_port: u16,
    #[clap(long, env)]
    pub pg_dbname: String,
    #[clap(long, env)]
    pub pg_user: String,
    #[clap(long, env)]
    pub pg_password: String,
    #[clap(long, env)]
    pub port: String,
    #[clap(long, env)]
    pub allow_origin: String,
    #[clap(long, env)]
    pub redis_url: String,
}
