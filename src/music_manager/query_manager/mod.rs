use std::{fs::File, io::{Read, Error, ErrorKind}, collections::HashSet};

pub fn select(columns: &[TableColumn], conditionals: &[Conditional]) -> String {
    let mut query = String::from("SELECT ");

    let mut columns_iter = columns.iter().peekable();
    while let Some(column) = columns_iter.next() {
        query += &column.to_string();
        if columns_iter.peek().is_some() {
            query += ", ";
        }
    }
    query += " FROM ";
    query += &get_tables_from_columns(columns);

    if conditionals.len() > 0 {
        query += " WHERE ";

        let mut conditionals_iter = conditionals.iter().peekable();
        while let Some(conditional) = conditionals_iter.next() {
            query += &conditional.to_string();
            if conditionals_iter.peek().is_some() {
                query += " AND ";
            }
        }
    }
    query
}

fn get_tables_from_columns(columns: &[TableColumn]) -> String {
    let mut tables = String::new();
    let mut used_tables = HashSet::new();
    let mut columns_iter = columns.iter().peekable();
    while let Some(column) = columns_iter.next() {
        if used_tables.contains(&column.as_table()) {
            continue;
        }
        tables += &column.as_table();
        if columns_iter.peek().is_some() {
            tables += ", ";
        }
        used_tables.insert(column.as_table());
    }
    tables
}

pub fn create_database() -> Result<String, Error> {
    let mut tables_file = File::open("./tables.sql")?;
    let mut buffer = String::new();
    tables_file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub enum Conditional<'a> {
    Eq(TableColumn<'a>, TableColumn<'a>),
    EqVal(TableColumn<'a>, &'a str),
    Like(TableColumn<'a>, &'a str),
}

impl <'a> ToString for Conditional<'a> {
    fn to_string(&self) -> String {
        match self {
            Conditional::Eq(table1, table2) => format!("{} = {}", table1.to_string(), table2.to_string()),
            Conditional::EqVal(table, value) => format!("{} = '{}'", table.to_string(), value),
            Conditional::Like(table, value) => format!("{} LIKE '%{}%'", table.to_string(), value),
        }
    }
}

pub enum TableColumn<'a> {
    Types(&'a str),
    Performers(&'a str),
    Persons(&'a str),
    Groups(&'a str),
    Albums(&'a str),
    Rolas(&'a str),
    InGroup(&'a str),
}

impl <'a> TableColumn<'a> {
    fn as_table(&self) -> &str {
        match self {
            TableColumn::Types(_) => "types",
            TableColumn::Performers(_) => "performers",
            TableColumn::Persons(_) => "persons",
            TableColumn::Groups(_) => "groups",
            TableColumn::Albums(_) => "albums",
            TableColumn::Rolas(_) => "rolas",
            TableColumn::InGroup(_) => "in_group",
        }
    }

    pub fn from_str(table: &'a str, column: &'a str) -> Result<TableColumn<'a>, Error> {
        match table {
            "types" | "type" => Ok(TableColumn::Types(column)),
            "performers" | "performer" => Ok(TableColumn::Performers(column)),
            "persons" | "person" => Ok(TableColumn::Persons(column)),
            "groups" | "group" => Ok(TableColumn::Groups(column)),
            "albums" | "album" => Ok(TableColumn::Albums(column)),
            "rolas" | "rola" => Ok(TableColumn::Rolas(column)),
            "in_group" => Ok(TableColumn::InGroup(column)),
            _ => Err(Error::new(ErrorKind::Other, "Error parsing table"))
        }
    }
}


impl <'a> ToString for TableColumn<'a> {
    fn to_string(&self) -> String {
        match self {
            TableColumn::Types(column) => format!("types.{}", column),
            TableColumn::Performers(column) => format!("performers.{}", column),
            TableColumn::Persons(column) => format!("persons.{}", column),
            TableColumn::Groups(column) => format!("groups.{}", column),
            TableColumn::Albums(column) => format!("albums.{}", column),
            TableColumn::Rolas(column) => format!("rolas.{}", column),
            TableColumn::InGroup(column) => format!("in_group.{}", column),
        }
    }
}
