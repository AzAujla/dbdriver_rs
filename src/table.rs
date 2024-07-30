#![allow(dead_code)]
use std::char;

#[derive(Debug)]
enum Key {
    Primary,
    Foreign,
}

#[derive(Debug)]
enum Contraints {
    ForeignRelation,
    Unique,
    NotNull,
    AutoIncrement,
}

#[derive(Debug)]
enum DataType {
    Int,
    String,
}

#[derive(Debug)]
struct Column {
    index: u16,
    name: String,
    key: Option<Key>,
    constraints: Vec<Contraints>,
    default: String,
    datatype: DataType,
    limit: usize,
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn col_len(&self) -> usize {
        self.columns.len()
    }

    pub fn new(cmd: String) -> Result<Self, String> {
        let cmd = cmd.clone();

        // Check if Syntax is correct and both defs and name are present
        match cmd.split_once(" ") {
            Some((name, defs)) => {
                let columns = Self::col_defs_from_string(defs);

                match columns {
                    Ok(columns) => {
                        return Ok(Table {
                            name: name.to_string(),
                            columns,
                        });
                    }
                    Err(e) => Err(e),
                }
            }
            //if one or both is absent
            None => Err(
                "Incorrect Syntax. Correct Syntax: CREATE TABLE <TABLE_NAME> <COLUMN_DEFINITIONS>"
                    .to_string(),
            ),
        }
    }

    fn col_defs_from_string(defs: &str) -> Result<Vec<Column>, String> {
        let defs_chars = defs.chars();

        if defs_chars.clone().nth(0).unwrap().ne(&'(')
            || defs_chars.clone().last().unwrap().ne(&')')
        {
            return Err(String::from(
                "Incorrect Syntax. Column Definitions must be wrapped in round brackets.",
            ));
        }

        let vec: Vec<char> = defs.chars().collect();
        let s: String = vec[1..(vec.len() - 1)].iter().collect();
        let cols: Vec<Column> = s
            .split(", ")
            .into_iter()
            .map(|v| {
                let name = v.split(" ").nth(0).unwrap();
                Column {
                    index: 0,
                    name: name.to_string(),
                    key: None,
                    constraints: Vec::new(),
                    default: String::from(""),
                    datatype: DataType::String,
                    limit: 255,
                }
            })
            .collect();

        Ok(cols)
    }
}
