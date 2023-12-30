pub enum Action {
    Static(Static),
    Configured(Configured),
}

impl From<Static> for Action {
    fn from(s: Static) -> Self {
        Action::Static(s)
    }
}

impl From<Configured> for Action {
    fn from(c: Configured) -> Self {
        Action::Configured(c)
    }
}

// Action without job configuration
pub enum Static {
    Completion(clap_complete::Shell),
    Help,
}

// Action with job configuration
pub enum Configured {
    List,
    Validate,
    Description(String),
    Run(String),
}
