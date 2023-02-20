#[derive(Clone, PartialEq)]
pub enum RoutePath {
    Home,
    Data,
    Files,
    Threads,
    NotFound,
}

impl RoutePath {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoutePath::Home => "/",
            RoutePath::Data => "/data",
            RoutePath::Files => "/files",
            RoutePath::Threads => "/threads",
            RoutePath::NotFound => "",
        }
    }
}
