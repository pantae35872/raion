use common::{
    no_hash_hashmap::NoHashHashMap,
    sin::sections::{SectionType, SinSection},
};
use xxhash_rust::xxh3::xxh3_64;

use crate::memory::{address::Address, Memory};

#[derive(Debug, Clone)]
pub struct LoadedSection {
    ty: SectionType,
    mem_start: Address,
    mem_end: Address,
}

#[derive(Debug)]
pub struct SectionManager {
    sections: NoHashHashMap<u64, LoadedSection>,
    write_pos: Address,
}

impl LoadedSection {
    pub fn section_type(&self) -> SectionType {
        return self.ty;
    }

    pub fn mem_start(&self) -> Address {
        return self.mem_start;
    }

    pub fn mem_end(&self) -> Address {
        return self.mem_end;
    }
}

impl SectionManager {
    pub fn new() -> Self {
        Self {
            sections: NoHashHashMap::default(),
            write_pos: Address::new(0),
        }
    }

    pub fn get_section_hash(&self, hash: u64) -> Option<&LoadedSection> {
        return self.sections.get(&hash);
    }

    pub fn get_section<T: AsRef<str>>(&self, name: T) -> Option<&LoadedSection> {
        return self.get_section_hash(xxh3_64(name.as_ref().as_bytes()));
    }

    pub fn set_section_hash(&mut self, hash: u64, section: LoadedSection) {
        self.sections.insert(hash, section);
    }

    pub fn load_section(
        &mut self,
        section: &SinSection,
        data: &[u8],
        memory: &mut Memory,
    ) -> &LoadedSection {
        let data = &data[section.start() as usize..section.end() as usize];
        memory
            .mem_sets(self.write_pos, data)
            .expect("Not enough memory to load the section");
        self.set_section_hash(
            section.hash(),
            LoadedSection {
                ty: section.section_type(),
                mem_start: self.write_pos,
                mem_end: self.write_pos + data.len() - 1,
            },
        );

        self.write_pos += data.len();
        return self.get_section_hash(section.hash()).unwrap();
    }
}
