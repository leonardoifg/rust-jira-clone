#![allow(unused_variables, dead_code)]
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum Status {
    Pending,
    InProgress,
    Resolved,
    Completed,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::InProgress => write!(f, "In progress"),
            Self::Resolved => write!(f, "Resolved"),
            Self::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Display for Epic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let str = format!("{} - {} (Epic)", self.name, self.status);
        write!(f, "{str}")
    }
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Pending,
            stories: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Display for Story {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let str = format!("{} - {} (Story)", self.name, self.status);
        write!(f, "{str}")
    }
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Pending,
        }
    }
}

pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
