pub enum Method {
    Parallel,
    Serial,
}

pub struct Config {
    pub method: Method,
    pub display: bool,
}
