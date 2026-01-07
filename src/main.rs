mod constants;

use clap::Parser;

/// The command-line framework.
#[derive(Parser, Debug)]
#[clap(name = "m_cut", about = "Obtain the specified fields from file(s). ")]
struct Cli {
    #[arg(short, long, required = true)]
    fields: String,

    #[arg(short, long, required = false, default_value = "\t")]
    delimiter: String,

    #[arg(short = 'p', long, required = true)]
    file_path: String,
}

#[derive(Debug)]
enum CliError {
    InvalidFields,
    EmptyDelimiter,
    FileNotFound,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CliError::InvalidFields => constants::INVALID_FIELD_TYPE,
            CliError::EmptyDelimiter => constants::EMPTY_DELIMITER,
            CliError::FileNotFound => constants::FILE_NOT_FOUND,
        };
        write!(f, "{}", msg)
    }
}

impl Cli {
    /// Enforces that fields must be a non-empty space
    /// separated string of number(s).
    fn validate_fields(&self) -> Option<CliError> {
        let mut has_num: bool = false;

        for ch in self.fields.chars() {
            if ch.is_numeric() {
                has_num = true;
            } else if !ch.is_ascii_whitespace() {
                return Some(CliError::InvalidFields);
            }
        }

        return if has_num {
            None
        } else {
            Some(CliError::InvalidFields)
        };
    }

    /// Enforces that the delimiter
    /// must be a non-empty string.
    fn validate_delimiter(&self) -> Option<CliError> {
        return if self.delimiter.is_empty() {
            Some(CliError::EmptyDelimiter)
        } else {
            None
        };
    }

    /// Enforces that the file path is valid,
    /// i.e., a file of the given name exists
    /// at the given location.
    fn validate_file_path(&self) -> Option<CliError> {
        return if std::path::Path::new(&self.file_path).exists() {
            None
        } else {
            Some(CliError::FileNotFound)
        };
    }

    fn validate_attrs(&self) -> Option<CliError> {
        self.validate_fields()
            .or_else(|| self.validate_delimiter())
            .or_else(|| self.validate_file_path())
    }

    /// Getter method for the fields attribute.
    fn get_fields(&self) -> Result<Vec<usize>, CliError> {
        let fields = self.fields.split_ascii_whitespace();
        let mut field_arr: Vec<usize> = Vec::new();

        for field in fields {
            if let Ok(f) = field.parse::<usize>() {
                field_arr.push(f);
            } else {
                return Err(CliError::InvalidFields);
            }
        }

        Ok(field_arr)
    }
}

fn main() {
    let cli_parsed = Cli::parse();

    if let Some(err) = cli_parsed.validate_attrs() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!(
        "{}\n{}\n{}",
        cli_parsed.fields, cli_parsed.delimiter, cli_parsed.file_path
    );
}
