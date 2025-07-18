// ENV
pub const PORT: u16 = 8081;
pub const ALLOW_ORIGIN: &str = "http://localhost:8081";
pub const POSTGRE_UNIX_SOCKET: bool = false;
pub const PG_SOCKET_DIR: &str = "/var/run/postgresql";
pub const PG_HOST: &str = "localhost";
pub const PG_PORT: u16 = 5432;
pub const PG_DBNAME: &str = "tiso";
pub const PG_USER: &str = "postgres";
pub const PG_PASSWORD: &str = "Ad12345#";

// VARS
pub const MIN_COMPRESS_SIZE: u16 = 10240;
pub const MIN_RECTANGLE_WIDTH: u16 = 100;