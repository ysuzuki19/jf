pub enum Static {
    Completion { shell: clap_complete::Shell },
    Help,
}

pub enum Configured {
    List,
    Validate,
    Description { job_name: String },
    Run { job_name: String },
}

pub enum CliBehavior {
    Static(Static),
    Configured(Configured),
}
