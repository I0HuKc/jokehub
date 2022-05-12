use serde::{Deserialize, Serialize};

use crate::model::shrimp::Paws;

#[derive(Clone, Serialize, Deserialize)]
pub struct Anecdote {
    pub category: String,
    pub text: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NewAnecdote {
    pub text: String,

    pub tags: Vec<String>,
    pub language: String,
}

impl Anecdote {
    pub fn new(na: &NewAnecdote) -> Self {
        Anecdote {
            text: na.text.to_string(),
            category: String::from("anecdote"),
        }
    }
}

impl Paws<Anecdote> for Anecdote {}
