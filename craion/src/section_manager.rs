use crate::memory::{address::Address, Memory};
use common::{
    constants::{
        BOOL_HASH, I16_HASH, I32_HASH, I64_HASH, I8_HASH, U16_HASH, U32_HASH, U64_HASH, U8_HASH,
        VOID_HASH,
    },
    no_hash_hashmap::NoHashHashMap,
    sin::sections::{Attribute, Field, Procedure, SinSection, VProcedure},
};

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone)]
pub enum LoadedType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Void,
    Bool,
    Custom { hash: u64 },
}

#[derive(Debug, Clone)]
pub struct LoadedVProcs {
    return_type: LoadedType,
    accept: Vec<LoadedType>,
    is_static: bool,
}

#[derive(Debug, Clone)]
pub struct LoadedProcedure {
    mem_start: Address,
    mem_size: u64,
    is_static: bool,
    return_type: LoadedType,
    accept: Vec<LoadedType>,
    visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct LoadedInterface {
    vprocs: NoHashHashMap<u64, LoadedVProcs>,
    visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct ImplementedInterface {
    procs: NoHashHashMap<u64, LoadedProcedure>,
}

#[derive(Debug, Clone)]
pub struct RuntimeField {
    contain_type: LoadedType,
    visibility: Visibility,
}

#[derive(Debug, Clone)]
pub struct LoadedStructure {
    fields: NoHashHashMap<u64, RuntimeField>,
    procs: NoHashHashMap<u64, LoadedProcedure>,
    implements: NoHashHashMap<u64, ImplementedInterface>,
    visibility: Visibility,
}

#[derive(Debug)]
pub struct SectionManager {
    procedures: NoHashHashMap<u64, LoadedProcedure>,
    interfaces: NoHashHashMap<u64, LoadedInterface>,
    structures: NoHashHashMap<u64, LoadedStructure>,
    write_pos: Address,
}

impl SectionManager {
    pub fn new() -> Self {
        Self {
            procedures: NoHashHashMap::default(),
            interfaces: NoHashHashMap::default(),
            structures: NoHashHashMap::default(),
            write_pos: Address::new(0),
        }
    }

    fn load_procedure(
        &mut self,
        proc: &Procedure,
        data: &[u8],
        memory: &mut Memory,
    ) -> (LoadedProcedure, Option<u64>) {
        let mut visibility = Visibility::Private;
        let mut is_static = false;
        let mut return_type = LoadedType::Void;
        let mut accept = Vec::new();
        let mut overwrited = None;
        for attribute in &proc.attributes.attributes {
            match attribute {
                Attribute::Public => visibility = Visibility::Public,
                Attribute::Private => visibility = Visibility::Private,
                Attribute::Static => is_static = true,
                Attribute::Accept(accep) => {
                    accept = accep.iter().map(|e| LoadedType::from_hash(*e)).collect();
                }
                Attribute::Overwrite(overwrite) => overwrited = Some(overwrite),
                Attribute::Return(retur) => return_type = LoadedType::from_hash(*retur),
                attri => panic!("Functions cant have this attribute {attri:?}"),
            }
        }
        let data = &data[proc.start as usize..proc.start as usize + proc.size as usize];
        memory
            .mem_sets(self.write_pos, data)
            .expect("Not enough memory to load the section");
        self.write_pos += data.len();

        (
            LoadedProcedure {
                mem_start: self.write_pos,
                mem_size: data.len() as u64,
                return_type,
                accept,
                visibility,
                is_static,
            },
            overwrited.copied(),
        )
    }

    fn load_field(field: &Field) -> RuntimeField {
        let mut visibility = Visibility::Private;
        let mut contain_type = LoadedType::Void;
        for attri in &field.attributes.attributes {
            match attri {
                Attribute::Public => visibility = Visibility::Public,
                Attribute::Private => visibility = Visibility::Private,
                Attribute::Contain(contain) => contain_type = LoadedType::from_hash(*contain),
                attri => panic!("Functions cant have this attribute {attri:?}"),
            }
        }
        RuntimeField {
            contain_type,
            visibility,
        }
    }

    fn load_vproc(vproc: &VProcedure) -> LoadedVProcs {
        let mut is_static = false;
        let mut return_type = LoadedType::Void;
        let mut accept = Vec::new();
        for attribute in &vproc.attributes.attributes {
            match attribute {
                Attribute::Static => is_static = true,
                Attribute::Accept(accep) => {
                    accept = accep.iter().map(|e| LoadedType::from_hash(*e)).collect();
                }
                Attribute::Return(retur) => return_type = LoadedType::from_hash(*retur),
                attri => panic!("Functions cant have this attribute {attri:?}"),
            }
        }

        LoadedVProcs {
            return_type,
            accept,
            is_static,
        }
    }

    pub fn load_section(&mut self, section: &SinSection, data: &[u8], memory: &mut Memory) {
        match section {
            SinSection::Procedure(proc) => {
                let proce = self.load_procedure(&proc, data, memory);
                match proce {
                    (proce, None) => {
                        self.procedures.insert(proc.hash_name, proce);
                    }
                    (_, Some(_)) => panic!("Stand alone attributes must not have overwrite"),
                };
            }
            SinSection::Structure(structure) => {
                let mut visibility = Visibility::Private;
                let mut must_implements = Vec::new();
                for attri in &structure.attributes.attributes {
                    match attri {
                        Attribute::Public => visibility = Visibility::Public,
                        Attribute::Private => visibility = Visibility::Private,
                        Attribute::Implemented(implement) => must_implements.push(implement),
                        attri => panic!("Functions cant have this attribute {attri:?}"),
                    }
                }

                let mut implements = NoHashHashMap::default();
                let mut procedures = NoHashHashMap::default();
                let mut overwrites = Vec::new();

                for proc in &structure.procedures {
                    let procd = self.load_procedure(&proc, data, memory);
                    match procd {
                        (procd, None) => {
                            procedures.insert(proc.hash_name, procd);
                        }
                        (procd, Some(overwrited)) => {
                            overwrites.push((proc.hash_name, procd, overwrited));
                        }
                    }
                }

                for (hash_name, overwrite, implemented_hash) in overwrites {
                    implements
                        .entry(implemented_hash)
                        .or_insert(ImplementedInterface {
                            procs: NoHashHashMap::default(),
                        })
                        .procs
                        .insert(hash_name, overwrite);
                }

                let implement_all = must_implements.iter().all(|e| implements.contains_key(e));
                if !implement_all {
                    panic!("The implement method does not match the specified implements");
                }

                let mut fields = NoHashHashMap::default();

                for field in &structure.fields {
                    fields.insert(field.hash_name, Self::load_field(&field));
                }

                self.structures.insert(
                    structure.hash_name,
                    LoadedStructure {
                        fields,
                        procs: procedures,
                        implements,
                        visibility,
                    },
                );
            }
            SinSection::Interface(interface) => {
                let mut visibility = Visibility::Private;
                for attri in &interface.attributes.attributes {
                    match attri {
                        Attribute::Public => visibility = Visibility::Public,
                        Attribute::Private => visibility = Visibility::Private,
                        attri => panic!("Functions cant have this attribute {attri:?}"),
                    }
                }
                let mut vprocs = NoHashHashMap::default();
                for vproc in &interface.vprocedures {
                    vprocs.insert(vproc.hash_name, Self::load_vproc(&vproc));
                }
                self.interfaces
                    .insert(interface.hash_name, LoadedInterface { vprocs, visibility });
            }
        }
        //let data = &data[section.start() as usize..section.end() as usize];
        //memory
        //    .mem_sets(self.write_pos, data)
        //    .expect("Not enough memory to load the section");
        //self.set_section_hash(
        //    section.hash(),
        //    LoadedSection {
        //        ty: section.section_type(),
        //        mem_start: self.write_pos,
        //        mem_end: self.write_pos + data.len() - 1,
        //    },
        //);

        //self.write_pos += data.len();
        //return self.get_section_hash(section.hash()).unwrap();
    }
}

impl LoadedType {
    pub fn from_hash(hash: u64) -> Self {
        match hash {
            U8_HASH => Self::U8,
            U16_HASH => Self::U16,
            U32_HASH => Self::U32,
            U64_HASH => Self::U64,
            I8_HASH => Self::I8,
            I16_HASH => Self::I16,
            I32_HASH => Self::I32,
            I64_HASH => Self::I64,
            BOOL_HASH => Self::Bool,
            VOID_HASH => Self::Void,
            hash => Self::Custom { hash },
        }
    }
}
