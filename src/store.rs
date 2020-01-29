use rusqlite::{params, Connection};
use std::error::Error;
use time::Timespec;

#[derive(Debug)]
struct Binding {
    binding_id: String,
    instance_id: String,
    updated_at: Timespec,
    data: String,
}

// TODO: configurable database location?
const DB: &str = "~/.local/rocs";

pub fn binding_save(
    instance_id: String,
    binding_id: String,
    data: String,
) -> Result<String, Box<dyn Error>> {
    let conn = Connection::open(DB)?;
    binding_table_check(&conn)?;

    conn.execute(
        "INSERT INTO service_bindings (binding_id, instance_id, updated_at, data) VALUES (?1, ?2, NOW(), ?3)",
        params![binding_id, instance_id, data],
    )?;

    Ok(binding_id)
}

pub fn binding_instance_id(binding_id: &String) -> Result<(String, String), Box<dyn Error>> {
    let conn = Connection::open(DB)?;
    binding_table_check(&conn)?;

    let mut stmt = conn.prepare(
        "SELECT instance_id, updated_at FROM service_bindings WHERE binding_id = ?1 LIMIT 1",
    )?;

    let mut binding = Binding {
        binding_id: "".into(),
        instance_id: "".into(),
        updated_at: Timespec::new(0, 0),
        data: "".into(),
    };

    let mut rows = stmt.query(params![binding_id])?;

    if let Some(row) = rows.next()? {
        binding.binding_id = binding_id.clone();
        binding.instance_id = row.get(0)?;
        binding.updated_at = row.get(1)?;
    }

    Ok((binding.instance_id, binding.binding_id))
}

pub fn binding_list(instance_id: String) -> Result<Vec<String>, Box<dyn Error>> {
    let conn = Connection::open(DB)?;
    binding_table_check(&conn)?;

    let mut stmt =
        conn.prepare("SELECT binding_id FROM service_bindings WHERE instance_id = ?1")?;
    let mut rows = stmt.query(params![instance_id])?;
    let mut list: Vec<String> = Vec::new();

    while let Some(row) = rows.next()? {
        list.push(row.get(0)?)
    }

    Ok(list)
}

pub fn binding_data(binding_id: String) -> Result<String, Box<dyn Error>> {
    let conn = Connection::open(DB)?;
    binding_table_check(&conn)?;

    let mut stmt = conn.prepare("SELECT data FROM service_bindings WHERE binding_id = ?1")?;
    let mut rows = stmt.query(params![binding_id])?;
    let mut data: String = String::from("");

    if let Some(row) = rows.next()? {
        data = row.get(0)?
    }

    Ok(data)
}

fn binding_table_check(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS service_bindings (
            binding_id      TEXT PRIMARY KEY,
            instance_id     TEXT NOT NULL,
            updated_at      DATE NOT NULL,
            data            TEXT
        )",
        params![],
    )?;

    Ok(())
}
