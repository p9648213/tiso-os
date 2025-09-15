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
pub const EXAMPLE_COLORS: [&str; 9] = [
    "radial-gradient(ellipse at top left, #070f2b, #1b1a55, #535c91)",
    "#FCA5A5",
    "#86EFAC",
    "#93C5FD",
    "#FDE68A",
    "#E9D5FF",
    "#A5F3FC",
    "#D1D5DB",
    "#F3F4F6",
];
pub const MAX_BACKGROUND_PICTURE_SIZE: usize = 2097152;
