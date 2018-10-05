use std::{fs::File, io::{Read, Error}, collections::HashSet};

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

    if conditionals.len() > 1 {
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
}

impl <'a> ToString for Conditional<'a> {
    fn to_string(&self) -> String {
        match self {
            Conditional::Eq(table1, table2) => format!("{} = {}", table1.to_string(), table2.to_string()),
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
