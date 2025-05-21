// SPDX-License-Identifier: MPL-2.0
use std::path::PathBuf;

use clap::ValueEnum;

use crate::cfg::cfg_path_gen::CfgPathGen;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
pub enum Mode {
    Empty,
    Template,
}

impl Mode {
    pub async fn render(&self) -> String {
        let cfg_path = CfgPathGen::new(None).gen();
        if exist(cfg_path.clone()).await {
            return format!(r#""{}" already exists."#, cfg_path.display());
        }

        let meta_text = render_meta(env!("CARGO_PKG_VERSION"));
        let job_text = match self {
            Self::Empty => "",
            Self::Template => TEMPLATE,
        };

        let cfg_text = format!("{meta_text}{job_text}");

        tokio::fs::write(cfg_path.clone(), cfg_text).await.unwrap();

        format!(r#"Created "{}""#, cfg_path.display())
    }
}

async fn exist(path: PathBuf) -> bool {
    match tokio::fs::try_exists(path).await {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => true,
    }
}

fn render_meta(version: &str) -> String {
    format!(
        r#"[meta]
version = "{version}"
"#
    )
}

const TEMPLATE: &str = r#"
[job.test]
command = "echo"
args = ["test"]

[job.build]
command = "echo"
args = ["build"]

[job.ci]
mode = "sequential"
jobs = ["test", "build"]

[job."test:live"]
mode = "watch"
job = "test"
watch_list = ["src/**/*"]

[job.run]
command = "echo"
args = ["run"]

[job."run:live"]
mode = "watch"
job = "run"
watch_list = ["src/**/*"]"#;
