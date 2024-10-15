use crate::memory::buffer_reader::BufferReader;

use super::SinError;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub hash_name: u64,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub hash_name: u64,
    pub start: u64,
    pub size: u64,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    pub hash_name: u64,
    pub fields: Vec<Field>,
    pub procedures: Vec<Procedure>,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VProcedure {
    pub hash_name: u64,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    pub hash_name: u64,
    pub vprocedures: Vec<VProcedure>,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    Public,
    Private,
    Implemented(u64),
    Contain(u64),
    Static,
    Accept(Vec<u64>),
    Return(u64),
    Overwrite(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SinSection {
    Procedure(Procedure),
    Structure(Structure),
    Interface(Interface),
}

impl Attribute {
    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        match reader.read_u8().ok_or(SinError::InvalidSin)? {
            1 => Ok(Self::Public),
            2 => Ok(Self::Private),
            3 => Ok(Self::Static),
            4 => Ok(Self::Contain(
                reader.read_u64().ok_or(SinError::InvalidSin)?,
            )),
            5 => Ok(Self::Implemented(
                reader.read_u64().ok_or(SinError::InvalidSin)?,
            )),
            7 => Ok(Self::Return(reader.read_u64().ok_or(SinError::InvalidSin)?)),
            8 => Ok(Self::Overwrite(
                reader.read_u64().ok_or(SinError::InvalidSin)?,
            )),
            6 => Ok(Self::Accept({
                let mut res = Vec::new();
                for _ in 0..reader.read_u64().ok_or(SinError::InvalidSin)? {
                    res.push(reader.read_u64().ok_or(SinError::InvalidSin)?);
                }
                res
            })),
            unknown_attr => Err(SinError::UnknownAttribute(unknown_attr)),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        match self {
            Self::Public => buffer.push(1),
            Self::Private => buffer.push(2),
            Self::Static => buffer.push(3),
            Self::Contain(..) => buffer.push(4),
            Self::Implemented(..) => buffer.push(5),
            Self::Accept(..) => buffer.push(6),
            Self::Return(..) => buffer.push(7),
            Self::Overwrite(..) => buffer.push(8),
        }
        match self {
            Self::Contain(hash)
            | Self::Implemented(hash)
            | Self::Return(hash)
            | Self::Overwrite(hash) => buffer.extend_from_slice(&hash.to_le_bytes()),
            Self::Accept(types) => {
                buffer.extend_from_slice(&(types.len() as u64).to_le_bytes());
                types
                    .iter()
                    .for_each(|e| buffer.extend_from_slice(&e.to_le_bytes()));
            }
            _ => {}
        }
        buffer
    }
}

impl Attributes {
    pub fn new(attributes: Vec<Attribute>) -> Self {
        Self { attributes }
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let mut attributes = Vec::new();
        for _ in 0..reader.read_u64().ok_or(SinError::InvalidSin)? {
            attributes.push(Attribute::from_reader(reader)?);
        }
        Ok(Self::new(attributes))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&(self.attributes.len() as u64).to_le_bytes());
        self.attributes
            .iter()
            .for_each(|e| buffer.extend_from_slice(&e.to_bytes()));
        buffer
    }
}

impl Field {
    pub fn new(hash_name: u64, attributes: Attributes) -> Self {
        Self {
            hash_name,
            attributes,
        }
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let decoded_hash = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let attributes = Attributes::from_reader(reader)?;

        Ok(Self::new(decoded_hash, attributes))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.hash_name.to_le_bytes());
        buffer.extend_from_slice(&self.attributes.to_bytes());

        buffer
    }
}

impl VProcedure {
    pub fn new(hash_name: u64, attributes: Attributes) -> Self {
        Self {
            hash_name,
            attributes,
        }
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let decoded_hash = reader.read_u64().ok_or(SinError::InvalidSin)?;

        Ok(Self::new(decoded_hash, Attributes::from_reader(reader)?))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.hash_name.to_le_bytes());
        buffer.extend_from_slice(&self.attributes.to_bytes());
        buffer
    }
}

impl Procedure {
    pub fn new(hash_name: u64, start: u64, size: u64, attributes: Attributes) -> Self {
        Self {
            hash_name,
            start,
            size,
            attributes,
        }
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let decoded_hash = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let decoded_start = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let decoded_size = reader.read_u64().ok_or(SinError::InvalidSin)?;

        return Ok(Self::new(
            decoded_hash,
            decoded_start,
            decoded_size,
            Attributes::from_reader(reader)?,
        ));
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.hash_name.to_le_bytes());
        buffer.extend_from_slice(&self.start.to_le_bytes());
        buffer.extend_from_slice(&self.size.to_le_bytes());
        buffer.extend_from_slice(&self.attributes.to_bytes());
        return buffer;
    }
}

impl Structure {
    pub fn new(
        hash_name: u64,
        fields: Vec<Field>,
        procedures: Vec<Procedure>,
        attributes: Attributes,
    ) -> Self {
        Self {
            hash_name,
            fields,
            procedures,
            attributes,
        }
    }
    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let decoded_hash = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let mut fields = Vec::new();
        for _ in 0..reader.read_u64().ok_or(SinError::InvalidSin)? {
            fields.push(Field::from_reader(reader)?);
        }
        let mut procedures = Vec::new();
        for _ in 0..reader.read_u64().ok_or(SinError::InvalidSin)? {
            procedures.push(Procedure::from_reader(reader)?);
        }
        return Ok(Self::new(
            decoded_hash,
            fields,
            procedures,
            Attributes::from_reader(reader)?,
        ));
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.hash_name.to_le_bytes());
        buffer.extend_from_slice(&(self.fields.len() as u64).to_le_bytes());
        self.fields
            .iter()
            .for_each(|e| buffer.extend_from_slice(&e.to_bytes()));
        buffer.extend_from_slice(&(self.procedures.len() as u64).to_le_bytes());
        self.procedures
            .iter()
            .for_each(|e| buffer.extend_from_slice(&e.to_bytes()));
        buffer.extend_from_slice(&self.attributes.to_bytes());
        return buffer;
    }
}

impl Interface {
    pub fn new(hash_name: u64, vprocedures: Vec<VProcedure>, attributes: Attributes) -> Self {
        Self {
            hash_name,
            vprocedures,
            attributes,
        }
    }

    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        let hash_name = reader.read_u64().ok_or(SinError::InvalidSin)?;
        let mut vprocedures = Vec::new();
        for _ in 0..reader.read_u64().ok_or(SinError::InvalidSin)? {
            vprocedures.push(VProcedure::from_reader(reader)?);
        }
        Ok(Self::new(
            hash_name,
            vprocedures,
            Attributes::from_reader(reader)?,
        ))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.hash_name.to_le_bytes());
        buffer.extend_from_slice(&(self.vprocedures.len() as u64).to_le_bytes());
        self.vprocedures
            .iter()
            .for_each(|e| buffer.extend_from_slice(&e.to_bytes()));
        buffer.extend_from_slice(&self.attributes.to_bytes());
        buffer
    }
}

impl SinSection {
    pub fn from_reader(reader: &mut BufferReader) -> Result<Self, SinError> {
        match reader.read_u8().ok_or(SinError::InvalidSin)? {
            1 => Ok(Self::Procedure(Procedure::from_reader(reader)?)),
            2 => Ok(Self::Structure(Structure::from_reader(reader)?)),
            3 => Ok(Self::Interface(Interface::from_reader(reader)?)),
            ty => Err(SinError::InvalidSectionType(ty)),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        match self {
            Self::Procedure(..) => buffer.push(1),
            Self::Structure(..) => buffer.push(2),
            Self::Interface(..) => buffer.push(3),
        }
        match self {
            Self::Procedure(proc) => buffer.extend_from_slice(&proc.to_bytes()),
            Self::Structure(structure) => buffer.extend_from_slice(&structure.to_bytes()),
            Self::Interface(procedure) => buffer.extend_from_slice(&procedure.to_bytes()),
        }
        return buffer;
    }
}
