use std::{collections::HashMap, sync::Arc};

use super::{Agent, Jobdef};
use crate::{
    error::{InternalError, JfError, JfResult},
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

    pub fn list_public(&self) -> Vec<String> {
        self.map
            .values()
            .filter_map(|jobdef| {
                if jobdef.visibility().is_public() {
                    Some(jobdef.name().to_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn validate(&self) -> JfResult<()> {
        let errs = self
            .map
            .values()
            .map(|jobdef| jobdef.build(self.clone(), Agent::Job))
            .filter_map(|res| match res {
                Ok(_) => None,
                Err(e) => Some(e),
            })
            .collect::<Vec<_>>();
        if errs.is_empty() {
            Ok(())
        } else {
            Err(JfError::Multi(errs))
        }
    }

    fn get(&self, job_name: String) -> JfResult<&Jobdef> {
        self.map
            .get(&job_name)
            .ok_or(InternalError::JobdefNotFound(job_name).into())
    }

    pub fn build(&self, job_name: String, agent: Agent) -> JfResult<Job> {
        self.get(job_name)?.build(self.clone(), agent)
    }

    pub fn description(&self, job_name: String) -> JfResult<&String> {
        Ok(self.get(job_name)?.description())
    }
}

#[cfg(test)]
impl crate::testutil::TryFixture for JobdefPool {
    #[cfg_attr(coverage, coverage(off))]
    fn try_gen() -> JfResult<Self> {
        let jobdef = crate::testutil::TryFixture::try_gen()?;
        Ok(Self::new(vec![jobdef]))
    }
}
