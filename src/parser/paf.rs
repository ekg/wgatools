use crate::converter::paf2block::paf2blocks;
use crate::converter::paf2chain::paf2chains;
use crate::errors::FileFormat;
use crate::parser::common::{AlignRecord, Strand};
use csv::{DeserializeRecordsIter, ReaderBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::str;

/// Parser for PAF format files
pub struct PafReader<R: io::Read> {
    inner: csv::Reader<R>,
}

impl<R> PafReader<R>
where
    R: io::Read,
{
    /// Create a new PAF parser
    pub fn new(reader: R) -> Self {
        PafReader {
            inner: ReaderBuilder::new()
                .delimiter(b'\t')
                .has_headers(false)
                .comment(Some(b'#'))
                .from_reader(reader),
        }
    }

    /// Iterate over the records in the PAF file
    pub fn records(&mut self) -> Records<'_, R> {
        Records {
            inner: self.inner.deserialize(),
        }
    }

    /// convert method
    pub fn convert(&mut self, outputpath: &str, format: FileFormat) {
        match format {
            FileFormat::Chain => paf2chains(self, outputpath),
            FileFormat::Blocks => paf2blocks(self, outputpath),
            _ => {}
        }
    }
}

impl PafReader<File> {
    /// Create a new PAF parser from a file path
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<PafReader<File>> {
        File::open(path).map(PafReader::new)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// A PAF record refer to https://github.com/lh3/miniasm/blob/master/PAF.md
pub struct PafRecord {
    pub query_name: String,
    pub query_length: u64,
    pub query_start: u64,
    pub query_end: u64,
    pub strand: char,
    pub target_name: String,
    pub target_length: u64,
    pub target_start: u64,
    pub target_end: u64,
    pub matches: u64,
    pub block_length: u64,
    pub mapq: u64,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// An iterator struct for PAF records
pub struct Records<'a, R: io::Read> {
    inner: DeserializeRecordsIter<'a, R, PafRecord>,
}

/// impl Iterator for Records
impl<'a, R: io::Read> Iterator for Records<'a, R> {
    type Item = csv::Result<PafRecord>;
    fn next(&mut self) -> Option<csv::Result<PafRecord>> {
        self.inner.next()
    }
}

/// impl AlignRecord Trait for PafRecord
impl AlignRecord for PafRecord {
    fn query_name(&self) -> &str {
        &self.query_name
    }

    fn query_length(&self) -> u64 {
        self.query_length
    }

    fn query_start(&self) -> u64 {
        self.query_start
    }

    fn query_end(&self) -> u64 {
        self.query_end
    }

    fn query_strand(&self) -> Strand {
        match self.strand {
            '+' => Strand::Positive,
            '-' => Strand::Negative,
            _ => panic!("Invalid strand"),
        }
    }

    fn target_name(&self) -> &str {
        &self.target_name
    }

    fn target_length(&self) -> u64 {
        self.target_length
    }

    fn target_start(&self) -> u64 {
        self.target_start
    }

    fn target_end(&self) -> u64 {
        self.target_end
    }

    fn target_strand(&self) -> Strand {
        Strand::Positive
    }

    fn get_cigar_bytes(&self) -> &[u8] {
        self.tags
            .iter()
            .find(|x| x.starts_with("cg:Z:"))
            .unwrap() // TODO: handle a err
            .as_bytes()
    }
}
