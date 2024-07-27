use std::{
    error::Error,
    fmt::{Debug, Display},
    usize,
};

#[derive(Debug, PartialEq)]
pub enum MemoryError {
    InvalidAddr(usize),
    OutOfRange(usize, usize),
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::InvalidAddr(address) => {
                write!(
                    f,
                    "Trying to access invalid memory address. with address: {}",
                    address
                )
            }
            MemoryError::OutOfRange(address, offset) => {
                write!(
                    f,
                    "Trying to access range of invalid memory. from address: {}, to address: {}",
                    address,
                    address + offset
                )
            }
        }
    }
}

impl Error for MemoryError {}

pub struct Memory {
    data: Vec<u8>,
}

impl From<&[u8]> for Memory {
    fn from(value: &[u8]) -> Self {
        Self {
            data: Vec::from(value),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Memory {
    fn from(value: &[u8; N]) -> Self {
        Self {
            data: Vec::from(value),
        }
    }
}

impl From<Vec<u8>> for Memory {
    fn from(value: Vec<u8>) -> Self {
        Self { data: value }
    }
}

impl From<usize> for Memory {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl Memory {
    /// Create new instance of memory with perfered capacity and initialize it with zero
    ///
    /// # Examples
    ///
    /// ```
    /// let memory = Memory::new(4);
    /// assert_eq!(Ok(vec![0, 0, 0, 0].as_slice()), memory.mem_gets(0, 4));
    /// ```
    pub fn new(size: usize) -> Self {
        let mut memory = Vec::with_capacity(size);
        for _ in 0..size {
            memory.push(0);
        }
        Self { data: memory }
    }

    /// Returns a u8 of a single byte of memory
    ///
    /// # Examples
    ///
    /// ```
    /// let mut memory = Memory::from(4);
    /// assert_eq!(Ok(1), memory.mem_set(0, 1));
    /// assert_eq!(Ok(5), memory.mem_set(1, 5));
    /// assert_eq!(Ok(7), memory.mem_set(2, 7));
    /// assert_eq!(Ok(vec![1, 5, 7, 0].as_slice()), memory.mem_gets(0, 4));
    /// ```
    pub fn mem_set(&mut self, address: usize, data: u8) -> Result<u8, MemoryError> {
        let a_data = match self.data.get_mut(address) {
            Some(data) => data,
            None => return Err(MemoryError::InvalidAddr(address)),
        };
        *a_data = data;
        return Ok(*a_data);
    }

    /// Set a range of memory
    ///
    /// Return a set memory
    ///
    /// # Examples
    ///
    /// ```
    /// let mut memory = Memory::from(4);
    /// assert_eq!(Ok(vec![2, 1, 2, 3].as_slice()), memory.mem_sets(0, &[2,1,2,3]));
    /// assert_eq!(Ok(vec![2, 1, 2, 3].as_slice()), memory.mem_gets(0, 4));
    /// assert_eq!(Ok(vec![1, 2, 4].as_slice()), memory.mem_sets(1, &[1,2,4]));
    /// assert_eq!(Ok(vec![2, 1, 2, 4].as_slice()), memory.mem_gets(0, 4));
    /// assert_eq!(Err(MemoryError::OutOfRange(1, 4)), memory.mem_sets(1, &[1,2,3,4]));
    /// ```

    pub fn mem_sets<'a>(
        &mut self,
        address: usize,
        datas: &'a [u8],
    ) -> Result<&'a [u8], MemoryError> {
        let a_data = match self.data.get_mut(address..address + datas.len()) {
            Some(data) => data,
            None => return Err(MemoryError::OutOfRange(address, datas.len())),
        };
        for (i, data) in datas.iter().enumerate() {
            a_data[i] = *data;
        }
        return Ok(datas);
    }

    /// Returns a u8 of a single byte of memory
    ///
    /// # Examples
    ///
    /// ```
    /// let memory = Memory::from(&[1, 2, 3, 4]);
    /// assert_eq!(Ok(2), memory.mem_get(1));
    /// assert_eq!(Ok(1), memory.mem_get(0));
    /// assert_eq!(Ok(4), memory.mem_get(3));
    /// assert_eq!(Err(MemoryError::InvalidAddr(4)), memory.mem_get(4));
    /// ```
    pub fn mem_get(&self, address: usize) -> Result<u8, MemoryError> {
        let data = match self.data.get(address) {
            Some(data) => data,
            None => return Err(MemoryError::InvalidAddr(address)),
        };

        return Ok(*data);
    }

    /// Returns a reference to a range of a memory
    ///
    /// # Examples
    ///
    /// ```
    /// let memory = Memory::from(&[1, 2, 3, 4]);
    /// assert_eq!(Ok(vec![1, 2, 3, 4].as_slice()), memory.mem_gets(0, 4));
    /// assert_eq!(Ok(vec![2, 3, 4].as_slice()), memory.mem_gets(1, 3));
    /// assert_eq!(Err(MemoryError::OutOfRange(1, 4)), memory.mem_gets(1, 4));
    /// ```
    pub fn mem_gets(&self, address: usize, size: usize) -> Result<&[u8], MemoryError> {
        let data = match self.data.get(address..address + size) {
            Some(data) => data,
            None => return Err(MemoryError::OutOfRange(address, size)),
        };

        return Ok(data);
    }
}
