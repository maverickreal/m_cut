use crate::constants;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
struct Cut {
    file_path: PathBuf,
    delimiter: char,
    fields: Vec<usize>,
}

enum CutError {
    FileCantOpen,
}

impl Cut {
    fn new(file_path: &String, delimiter: &char, fields: &Vec<usize>) -> Cut {
        let path = PathBuf::from(file_path);

        Cut {
            file_path: path,
            delimiter: *delimiter,
            fields: fields.clone(),
        }
    }

    fn parse(&self, reader: &mut BufReader<File>, arr: &mut Vec<Vec<String>>) -> Option<CutError> {
        todo!()
    }

    fn exec_cut(&self) -> Result<Vec<Vec<String>>, CutError> {
        let mut res: Vec<Vec<String>> = Vec::new();

        let file = File::open(&self.file_path).map_err(|err| {
            println!("{}", err);

            return CutError::FileCantOpen;
        })?;

        let mut reader = BufReader::new(file);
        let parsed_resp = self.parse(&mut reader, &mut res);

        return if let Some(err) = parsed_resp {
            Err(err)
        } else {
            Ok(res)
        };
    }
}

impl std::fmt::Display for CutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CutError::FileCantOpen => constants::FILE_CANT_OPEN,
        };

        write!(f, "{}", msg)
    }
}
