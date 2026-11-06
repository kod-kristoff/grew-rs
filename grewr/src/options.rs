use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// application of a rewriting system to a set of graphs
    Transform(Transform),
    /// search for a request in a corpus
    Grep(Grep),
    /// compute stats of a set of requests in a set of corpora
    Count(Count),
    /// compile a set of corpora
    Compile(Compile),
    /// clean a set of corpora
    Clean(Clean),
}

#[derive(Debug, clap::Args)]
pub struct Transform {
    #[clap(long = "grs")]
    /// the main file which describes the Graph Rewriting System. If no GRS is given, the empty GRS is loaded: `strat main {Seq ()}`
    pub grs_file: Option<PathBuf>,
    #[clap(short = 'i', long = "input")]
    /// select the input data. If no input file is given, Grewr reads from `stdin`
    pub file_or_directory: Option<PathBuf>,
    #[clap(short, long = "output")]
    /// the name of the output file (CoNLL file). If no output file is given, Grewr reads from `stdout`
    pub output_file: Option<PathBuf>,
    #[clap(long, group = "output_format")]
    /// use CoNLL-U as output format, this is the default
    conll_u: bool,
    #[clap(long, group = "output_format")]
    /// use JSON as output format
    json: bool,
    #[clap(long, group = "output_format")]
    /// use Multi-JSON as output format
    multi_json: bool,
    #[clap(long, group = "output_format")]
    /// use CUPT as output format
    cupt: bool,
    #[clap(long, group = "output_format")]
    /// use SemCor as output format
    semcor: bool,
    #[clap(long, group = "output_format")]
    /// use DOT as output format
    dot: bool,
    #[clap(long, group = "output_format")]
    /// use CoNLL-X as output format, specify columns as comma-separated string
    columns: Option<String>,
    #[clap(short, long, default_value = "main")]
    /// the strategy used in transformation
    pub strat: String,
    #[clap(long)]
    /// If given, it makes rewriting process fail in case of ineffective command
    pub safe_commands: bool,
    #[clap(short, long, default_value = "ud")]
    /// This changes how CoNLL-U and GRS files are interpreted
    config: FormatConfig,
}

impl Transform {
    pub fn output_format(&self) -> TransformOutputFormat {
        if self.json {
            TransformOutputFormat::Json
        } else if self.multi_json {
            TransformOutputFormat::MultiJson
        } else if self.cupt {
            TransformOutputFormat::Cupt
        } else if self.semcor {
            TransformOutputFormat::Semcor
        } else if self.dot {
            TransformOutputFormat::Dot
        } else if let Some(columns) = &self.columns {
            TransformOutputFormat::ConllX(columns.split(',').map(ToString::to_string).collect())
        } else {
            TransformOutputFormat::ConllU
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct Grep {
    #[clap(long = "request")]
    /// file or string which describes a request
    pub file_or_code: String,
    #[clap(short = 'i', long = "input")]
    /// select the input data.
    /// Can be given multiple times.
    /// If no input file is given, Grewr reads from `stdin`
    pub file_or_directory: Vec<PathBuf>,
    #[clap(long = "key")]
    /// make a clustering of search output (the option can appear multiple times)
    pub clustering_item: Vec<String>,
    #[clap(short, long, default_value = "ud")]
    /// This changes how CoNLL-U and GRS files are interpreted
    config: FormatConfig,
}

#[derive(Debug, clap::Args)]
pub struct Count {
    #[clap(long = "request")]
    /// file or string which describes a request
    pub file_or_code: String,
    #[clap(short = 'i', long = "input")]
    /// select the input data.
    /// Can be given multiple times.
    /// If no input file is given, Grewr reads from `stdin`
    pub file_or_directory: Vec<PathBuf>,
    #[clap(long = "key")]
    /// make a clustering of search output (the option can appear multiple times)
    pub clustering_item: Vec<String>,
    #[clap(long, group = "output_format")]
    /// use JSON as output format, this is the default
    pub json: bool,
    #[clap(long, group = "output_format")]
    /// use TSV as output format, only if the output dimension is 2
    pub tsv: bool,
    #[clap(short, long, default_value = "ud")]
    /// This changes how CoNLL-U and GRS files are interpreted
    config: FormatConfig,
}

impl Count {
    pub fn output_format(&self) -> CountOutputFormat {
        if self.tsv {
            CountOutputFormat::Tsv
        } else {
            CountOutputFormat::Json
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct Compile {
    #[clap(long, name = "FOLDER", env = "CORPUSBANK")]
    /// folder to search for corpora in
    pub corpusbank: PathBuf,
    /// compiles all corpora for which id contains PATTERN
    pub pattern: Option<String>,
}

#[derive(Debug, clap::Args)]
pub struct Clean {
    #[clap(long, name = "FOLDER", env = "CORPUSBANK")]
    /// folder to search for corpora in
    pub corpusbank: PathBuf,
    /// compiles all corpora for which id contains PATTERN
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
/// This changes how CoNLL-U and GRS files are interpreted
pub enum FormatConfig {
    Ud,
    Sud,
    Sequoia,
    Basic,
}

#[derive(Debug, Clone)]
/// This changes how CoNLL-U and GRS files are interpreted
pub enum TransformOutputFormat {
    ConllU,
    Json,
    Cupt,
    Semcor,
    ConllX(Vec<String>),
    Dot,
    MultiJson,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
/// This changes how CoNLL-U and GRS files are interpreted
pub enum CountOutputFormat {
    Json,
    Tsv,
}
