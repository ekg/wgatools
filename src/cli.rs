use clap::ArgAction;
use clap::{command, Parser, Subcommand};

use crate::parser::common::FileFormat;

#[derive(Parser)]
#[command(name = "wgatools")]
#[command(
    about = "a cross-platform and ultrafast toolkit for Whole Genome Alignment Files manipulation"
)]
#[command(long_about = "long_about todo!!!")]
#[command(author, version)]
#[command(
help_template =
"{name} -- {about}\n\nVersion: {version}\n\nAuthors: {author}\
    \n\n{usage-heading} {usage}\n\n{all-args}"
) // change template more!
]
pub struct Cli {
    /// Output file ("-" for stdout)
    #[arg(long, short, global = true, default_value = "-", help_heading = Some("GLOBAL"))]
    pub outfile: String,
    /// Bool, if rewrite output file [default: false]
    #[arg(long, short, global = true, default_value = "false", help_heading = Some("GLOBAL"))]
    pub rewrite: bool,
    /// Threads, default 1
    #[arg(long, short, global = true, default_value = "1", help_heading = Some("GLOBAL"))]
    pub threads: usize,
    /// Logging level [-v: Info, -vv: Debug, -vvv: Trace, defalut: Warn].
    #[arg(short, long, global = true, action = ArgAction::Count, help_heading = Some("GLOBAL"))]
    pub verbose: u8,
    /// Subcommands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert MAF format to PAF format
    #[command(visible_alias = "m2p", name = "maf2paf")]
    Maf2Paf {
        /// Input MAF File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
    },
    /// Convert MAF format to Chain format
    #[command(visible_alias = "m2c", name = "maf2chain")]
    Maf2Chain {
        /// Input MAF File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
    },
    /// Convert PAF format to MAF format
    #[command(visible_alias = "p2m", name = "paf2maf")]
    Paf2Maf {
        /// Input PAF File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
        /// Input target FASTA File, required
        #[arg(required = true, long, short)]
        target: String,
        /// Input query FASTA File, required
        #[arg(required = true, long, short)]
        query: String,
    },
    /// Convert PAF format to Chain format
    #[command(visible_alias = "p2c", name = "paf2chain")]
    Paf2Chain {
        /// Input PAF File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
    },
    /// Convert Chain format to MAF format
    #[command(visible_alias = "c2m", name = "chain2maf")]
    Chain2Maf {
        /// Input Chain File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
        /// Input target FASTA File, required
        #[arg(required = true, long, short)]
        target: String,
        /// Input query FASTA File, required
        #[arg(required = true, long, short)]
        query: String,
    },
    /// Convert Chain format to PAF format
    #[command(visible_alias = "c2p", name = "chain2paf")]
    Chain2Paf {
        /// Input Chain File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
    },
    /// Build index for MAF file
    #[command(visible_alias = "mi", name = "maf-index")]
    MafIndex {
        /// Input MAF File
        #[arg(required = true)]
        input: String,
    },
    /// Extract specific region from MAF file with index
    #[command(visible_alias = "me", name = "maf-ext")]
    MafExtract {
        /// Input MAF File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
        /// Input regions
        #[arg(required = false, long, short, value_delimiter = ',')]
        regions: Option<Vec<String>>,
        /// Input regions file
        #[arg(required = false, long, short)]
        file: Option<String>,
    },
    /// Call Variants from MAF file
    #[command(visible_alias = "c", name = "call")]
    Call {
        /// Input MAF File
        #[arg(required = false)]
        input: Option<String>,
        /// Sample name
        #[arg(
            required = false,
            long = "sample",
            short = 'n',
            default_value = "sample"
        )]
        sample: Option<String>,
        /// If call SNP
        #[arg(required = false, long = "snp", short = 's', default_value = "false")]
        snp: bool,
        /// SV length cutoff
        #[arg(required = false, long = "svlen", short = 'l', default_value = "50")]
        svlen: u64,
    },
    /// View MAF file in terminal
    #[command(visible_alias = "tv", name = "tview")]
    Tview {
        /// Input MAF File, with index '.index'
        #[arg(required = false)]
        input: String,
        /// Move step size
        #[arg(required = false, long, short, default_value = "10")]
        step: usize,
    },
    /// Statistics for Alignment file
    #[command(visible_alias = "st", name = "stat")]
    Stat {
        /// Input Alignment File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
        /// Input File format,
        #[arg(required = false, long, short, default_value = "maf")]
        format: FileFormat,
        /// Show each block's statistics, default: false
        #[arg(required = false, long, short, default_value = "false")]
        each: bool,
    },
    /// TEST: Plot dotplot for Alignment file
    #[command(visible_alias = "dp", name = "dotplot")]
    Dotplot {},
    /// Filter records for Alignment file
    #[command(visible_alias = "fl", name = "filter")]
    Filter {
        /// Input Alignment File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
        /// Input File format,
        #[arg(required = false, long, short, default_value = "maf")]
        format: FileFormat,
        /// Min block size
        #[arg(required = false, long, short = 'b', default_value = "0")]
        min_block_size: u64,
        /// Min query size, usually for contigs
        #[arg(required = false, long, short = 'q', default_value = "0")]
        min_query_size: u64,
    },
    /// TEST: maf2sam
    #[command(visible_alias = "m2s", name = "maf2sam")]
    Maf2Sam {
        /// Input MAF File, None for STDIN
        #[arg(required = false)]
        input: Option<String>,
    },
}

pub fn make_cli_parse() -> Cli {
    Cli::parse()
}
