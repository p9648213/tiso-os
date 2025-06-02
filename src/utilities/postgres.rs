use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use super::config::EnvConfig;

fn create_config(config: &EnvConfig) -> tokio_postgres::Config {
    let mut cfg = tokio_postgres::Config::new();
    cfg.host(&config.pg_host);
    cfg.port(config.pg_port);
    cfg.dbname(&config.pg_dbname);
    cfg.user(&config.pg_user);
    cfg.password(&config.pg_password);
    cfg
}

pub fn create_pool(config: &EnvConfig) -> Pool {
    let pg_config = create_config(config);

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let manager = Manager::from_config(pg_config, NoTls, manager_config);

    Pool::builder(manager).max_size(16).build().unwrap()
}
