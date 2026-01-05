use std::collections::HashMap;

use crate::db::Id;

type Error = PatientError;

#[derive(Debug, Clone, Copy)]
pub enum PatientError {
    CouldNotAddResearch,
    ThereNoResearches,
    ResearchNotFound,
}

#[derive(Debug, Clone)]
pub struct Pacient {
    pub id: Id,
    pub last_name: String,
    pub first_name: String,
    pub patronymic: String,
    pub birthdate: chrono::NaiveDate,
    pub gender: Gender,
    pub researches: HashMap<Id, ()>,
}

impl Pacient {
    pub fn add_research(&mut self, id: Id) -> Result<(), Error> {
        let status = self.researches.insert(id, ());
        match status {
            Some(_) => Ok(()),
            None => Err(Error::CouldNotAddResearch),
        }
    }

    pub fn get_researches(&self) -> Result<&HashMap<Id, ()>, Error> {
        if self.researches.is_empty() {
            return Ok(&self.researches);
        }

        Err(Error::ThereNoResearches)
    }

    pub fn get_mut_researches(&mut self) -> Result<&mut HashMap<Id, ()>, Error> {
        if self.researches.is_empty() {
            return Ok(&mut self.researches);
        }

        Err(Error::ThereNoResearches)
    }

    pub fn find_research_id(&self, id: Id) -> Result<(), Error> {
        if self.researches.contains_key(&id) {
            return Ok(());
        }

        Err(Error::ResearchNotFound)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Gender {
    M,
    W,
}
