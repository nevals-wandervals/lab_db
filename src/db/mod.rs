use std::collections::HashMap;

use crate::db::{method::Method, pacient::Pacient, research::Research};

pub mod analysis_results;
pub mod method;
pub mod pacient;
pub mod research;

type Error = DataBaseError;
pub type Id = usize;

#[derive(Debug, Clone, Copy)]
pub enum DataBaseError {
    PatientNotFound,
    ThereNoPatients,
    CouldNotAddPatient,

    MethodNotFound,
    ThereNoMethods,
    CouldNotAddMethod,

    ResearchNotFound,
    ThereNoResearches,
    CouldNotAddResearch,
}

pub struct DataBase {
    pub pacients: HashMap<Id, Pacient>,
    pub methods: HashMap<Id, Method>,
    pub researches: HashMap<Id, Research>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            pacients: HashMap::new(),
            methods: HashMap::new(),
            researches: HashMap::new(),
        }
    }

    pub fn add_pacient(&mut self, new_pacient: Pacient) -> Result<(), Error> {
        let status = self.pacients.insert(new_pacient.id, new_pacient);
        match status {
            Some(_) => Err(Error::CouldNotAddPatient),
            None => Ok(()),
        }
    }

    pub fn get_pacients(&self) -> Result<&HashMap<Id, Pacient>, Error> {
        if self.pacients.is_empty() {
            return Ok(&self.pacients);
        }

        Err(Error::ThereNoPatients)
    }

    pub fn get_mut_pacients(&mut self) -> Result<&mut HashMap<Id, Pacient>, Error> {
        if self.pacients.is_empty() {
            return Ok(&mut self.pacients);
        }

        Err(Error::ThereNoPatients)
    }

    pub fn get_pacient_id(&self, id: Id) -> Result<&Pacient, Error> {
        if let Some(p) = self.pacients.get(&id) {
            return Ok(p);
        }

        Err(Error::PatientNotFound)
    }

    pub fn get_mut_pacient_id(&mut self, id: Id) -> Result<&mut Pacient, Error> {
        if let Some(p) = self.pacients.get_mut(&id) {
            return Ok(p);
        }

        Err(Error::PatientNotFound)
    }

    pub fn add_method(&mut self, new_method: Method) -> Result<(), Error> {
        let status = self.methods.insert(new_method.id, new_method);
        match status {
            Some(_) => Err(Error::CouldNotAddMethod),
            None => Ok(()),
        }
    }

    pub fn get_methods(&self) -> Result<&HashMap<Id, Method>, Error> {
        if self.methods.is_empty() {
            return Ok(&self.methods);
        }

        Err(Error::ThereNoMethods)
    }

    pub fn get_mut_methods(&mut self) -> Result<&mut HashMap<Id, Method>, Error> {
        if self.methods.is_empty() {
            return Ok(&mut self.methods);
        }

        Err(Error::ThereNoMethods)
    }

    pub fn get_method_id(&self, id: Id) -> Result<&Method, Error> {
        if let Some(m) = self.methods.get(&id) {
            return Ok(m);
        }

        Err(Error::MethodNotFound)
    }

    pub fn get_mut_method_id(&mut self, id: Id) -> Result<&mut Method, Error> {
        if let Some(m) = self.methods.get_mut(&id) {
            return Ok(m);
        }

        Err(Error::MethodNotFound)
    }

    pub fn add_research(&mut self, new_research: Research) -> Result<(), Error> {
        let status = self.researches.insert(new_research.id, new_research);
        match status {
            Some(_) => Err(Error::CouldNotAddResearch),
            None => Ok(()),
        }
    }

    pub fn get_researches(&self) -> Result<&HashMap<Id, Research>, Error> {
        if self.researches.is_empty() {
            return Ok(&self.researches);
        }

        Err(Error::ThereNoResearches)
    }

    pub fn get_mut_researches(&mut self) -> Result<&mut HashMap<Id, Research>, Error> {
        if self.researches.is_empty() {
            return Ok(&mut self.researches);
        }

        Err(Error::ThereNoResearches)
    }

    pub fn get_research_id(&self, id: Id) -> Result<&Research, Error> {
        if let Some(r) = self.researches.get(&id) {
            return Ok(r);
        }

        Err(Error::ResearchNotFound)
    }

    pub fn get_mut_research_id(&mut self, id: Id) -> Result<&mut Research, Error> {
        if let Some(r) = self.researches.get_mut(&id) {
            return Ok(r);
        }

        Err(Error::ResearchNotFound)
    }
}
