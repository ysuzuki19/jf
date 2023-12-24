use std::{collections::HashMap, sync::Arc};

use super::{Agent, Jobdef};
use crate::{
    error::{JfError, JfResult},
    job::Job,
};

#[derive(Clone)]
pub struct JobdefPool {
    map: Arc<HashMap<String, Jobdef>>,
}

impl JobdefPool {
    pub fn new(jobdef_vec: Vec<Jobdef>) -> Self {
        let mut map = HashMap::new();
        for jobdef in jobdef_vec {
            map.insert(jobdef.name().to_owned(), jobdef);
        }
        Self { map: Arc::new(map) }
    }

    fn get(&self, job_name: String) -> JfResult<&Jobdef> {
        self.map
            .get(&job_name)
            .ok_or(JfError::JobdefNotFound(job_name))
    }

    pub fn list(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn validate(&self) -> JfResult<()> {
        let errs = self
            .map
            .iter()
            .map(|(_, jobdef)| jobdef.build(self.clone(), Agent::Job))
            .filter_map(|r| if let Err(e) = r { Some(e) } else { None })
            .collect::<Vec<_>>();
        if errs.is_empty() {
            Ok(())
        } else {
            Err(JfError::Multi(errs))
        }
    }

    pub fn build(&self, job_name: String, agent: Agent) -> JfResult<Job> {
        self.get(job_name)?.build(self.clone(), agent)
    }

    pub fn description(&self, job_name: String) -> JfResult<&String> {
        Ok(self.get(job_name)?.description())
    }
}
