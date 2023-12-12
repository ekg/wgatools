use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Seek, Write},
};

use itertools::enumerate;
use serde::{Deserialize, Serialize};

use crate::parser::{common::Strand, maf::MAFReader};

pub fn build_index(
    mafreader: &mut MAFReader<File>,
    idx_wtr: Box<dyn Write>,
) -> Result<(), Box<dyn Error>> {
    // init a MAfIndex2 struct
    let mut idx: MafIndex = HashMap::new();

    loop {
        let offset = mafreader.inner.stream_position()?;
        let record = mafreader.records().next();
        let record = match record {
            Some(r) => match r {
                Ok(r) => r,
                Err(e) => {
                    return Err(Box::new(e));
                }
            },
            None => break,
        };
        // TODO: check if name is unique
        for (ord, sline) in enumerate(record.slines) {
            let name = sline.name;
            let start = sline.start;
            let end = sline.start + sline.align_size;
            let size = sline.size;
            let strand = sline.strand;

            idx.entry(name.clone()).or_insert(MafIndexItem {
                ivls: Vec::new(),
                size,
                ord,
            });
            idx.get_mut(&name).unwrap().ivls.push(IvP {
                start,
                end,
                strand,
                offset,
            });
        }
    }
    serde_json::to_writer(idx_wtr, &idx)?;
    Ok(())
}

pub type MafIndex = HashMap<String, MafIndexItem>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MafIndexItem {
    pub ivls: Vec<IvP>,
    pub size: u64,
    pub ord: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IvP {
    pub start: u64,
    pub end: u64,
    pub strand: Strand,
    pub offset: u64,
}
