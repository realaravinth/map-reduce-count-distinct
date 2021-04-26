pub enum Method {
    Parallel,
    Serial,
    ParallelThreadPool,
}

pub struct Config {
    pub method: Method,
    pub display: bool,
}
