use std::{
    fmt::{Display, Error},
    fs::File,
    io::Read,
};

use std::fmt::Write;

use inline_colorization::*;

use crate::Location;

pub struct ErrorGenerator<'a, T: AsRef<str> + Display> {
    message: T,
    identation: usize,
    file: String,
    location: &'a Location,
    buffer: String,
}

impl<'a, T: AsRef<str> + Display> ErrorGenerator<'a, T> {
    pub fn new(location: &'a Location, message: T, identation: usize) -> Self {
        let mut file_buf = String::new();
        File::open(&location.file)
            .expect("Cannot open file for error generation")
            .read_to_string(&mut file_buf)
            .unwrap();
        Self {
            message,
            file: file_buf,
            identation,
            buffer: String::new(),
            location,
        }
    }

    pub fn vertical_pipe<E: AsRef<str> + Display>(mut self, message: E) -> Result<Self, Error> {
        write!(
            self.buffer,
            "{color_blue}{style_bold}{message}{style_reset}{color_reset} "
        )?;
        self.write_indentation(self.identation - message.as_ref().len())?;
        write!(
            self.buffer,
            "{color_blue}{style_bold}|{style_reset}{color_reset}"
        )?;
        return Ok(self);
    }

    fn write_indentation(&mut self, count: usize) -> Result<(), Error> {
        for _ in 0..count {
            write!(self.buffer, " ")?;
        }
        return Ok(());
    }

    pub fn write_line(mut self, column: usize) -> Result<Self, Error> {
        write!(
            self.buffer,
            " {}",
            self.file
                .lines()
                .nth(column - 1)
                .expect("The errored file is not valid")
        )?;
        return Ok(self);
    }

    pub fn pointer<E: AsRef<str> + Display>(
        mut self,
        identation: usize,
        message: E,
        pointer: char,
        color: &'static str,
    ) -> Result<Self, Error> {
        self.write_indentation(identation)?;
        write!(
            self.buffer,
            "{color}{style_bold}{}{}{style_reset}{color_reset}",
            pointer, message
        )?;
        return Ok(self);
    }

    pub fn ident_string<E: AsRef<str> + Display>(
        mut self,
        identation: usize,
        value: E,
        color: &'static str,
    ) -> Result<Self, Error> {
        self.write_indentation(identation)?;
        write!(
            self.buffer,
            "{color}{style_bold}{}{style_reset}{color_reset}",
            value
        )?;
        return Ok(self);
    }

    pub fn new_line(mut self) -> Result<Self, Error> {
        write!(self.buffer, "\n")?;
        return Ok(self);
    }

    pub fn build(self) -> String {
        format!(
            "{color_red}{style_bold}error{style_reset}{color_reset}: {}\n {}{color_blue}{style_bold}---->{style_reset}{color_reset} {}\n{}",
            self.message, {
                let mut buffer = String::new();
                for _ in 2..self.identation {
                    buffer.push(' ');
                }
                buffer
            }, self.location, self.buffer
        )
    }
}
