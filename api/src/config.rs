pub struct Config {
    host: String,
    port: i32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            host: std::env::var("BACKEND_HOST").unwrap_or("127.0.0.1".to_string()),
            port: std::env::var("BACKEND_PORT").unwrap_or("3000".to_string()).parse().unwrap(),
        }
    }

    pub fn hostname(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
