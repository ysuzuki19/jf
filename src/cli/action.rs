pub enum Action {
    Static(Static),
    Configured(Configured),
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
