pub enum Method {
    Parallel,
    Serial,
    ParallelThreadPool,
    DashMap,
}

pub struct Config {
    pub method: Method,
    pub display: bool,
}
