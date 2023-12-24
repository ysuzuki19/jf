use crate::error::JfError;

use super::Args;

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

impl TryFrom<Args> for CliBehavior {
    type Error = JfError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        if let Some(shell) = args.completion {
            Ok(Self::Static(Static::Completion { shell }))
        } else if args.list {
            Ok(Self::Configured(Configured::List))
        } else if args.validate {
            Ok(Self::Configured(Configured::Validate))
        } else if args.description {
            if let Some(job_name) = args.job_name {
                Ok(Self::Configured(Configured::Description { job_name }))
            } else {
                Err(JfError::Custom(
                    "Please input <JOB_NAME> to --description".to_string(),
                ))
            }
        } else if let Some(job_name) = args.job_name {
            Ok(Self::Configured(Configured::Run { job_name }))
        } else {
            Ok(Self::Static(Static::Help))
        }
    }
}
