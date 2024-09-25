#![feature(map_try_insert)]

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

pub mod compiler;
pub mod error;
pub mod lexer;
pub mod manager;
pub mod token;

#[derive(Debug)]
pub struct WithLocation<T> {
    value: T,
    location: Location,
}

#[derive(Debug, Default, Clone)]
pub struct Location {
    file: PathBuf,
    row: usize,
    column: usize,
}

impl<T> Deref for WithLocation<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<T> DerefMut for WithLocation<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_mut()
    }
}

impl<T> WithLocation<T> {
    pub fn new(value: T, location: Location) -> Self {
        Self { value, location }
    }

    pub fn value(&self) -> &T {
        return &self.value;
    }

    pub fn value_mut(&mut self) -> &mut T {
        return &mut self.value;
    }

    pub fn value_owned(self) -> T {
        self.value
    }

    pub fn location(&self) -> &Location {
        return &self.location;
    }
}

impl<T: Clone> Clone for WithLocation<T> {
    fn clone(&self) -> Self {
        WithLocation::new(self.value.clone(), self.location.clone())
    }
}

impl Location {
    pub fn new(row: usize, column: usize, file: PathBuf) -> Self {
        Self { column, row, file }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.file().display(),
            self.column(),
            self.row()
        )
    }
}
