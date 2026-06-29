use rusqlite::{Connection, Result};
use serde_json::{json, Value};

pub fn query_atom(conn: &Connection, term: &str) -> Value {
    let result: Result<Value> = conn.query_row(
        "SELECT id, emoji, label, category, keywords, color, sound, texture, temperature, definition
         FROM emoji_definitions
         WHERE LOWER(label) LIKE LOWER(?1)
         LIMIT 1",
        [format!("%{}%", term)],
        |row| {
            let keywords_str: String = row.get(4)?;
            let keywords: Value = serde_json::from_str(&keywords_str).unwrap_or(Value::Array(vec![]));
            Ok(json!({
                "id":          row.get::<_, String>(0)?,
                "emoji":       row.get::<_, String>(1)?,
                "label":       row.get::<_, String>(2)?,
                "category":    row.get::<_, String>(3)?,
                "keywords":    keywords,
                "color":       row.get::<_, Option<String>>(5)?,
                "sound":       row.get::<_, Option<String>>(6)?,
                "texture":     row.get::<_, Option<String>>(7)?,
                "temperature": row.get::<_, Option<String>>(8)?,
                "definition":  row.get::<_, Option<String>>(9)?
            }))
        },
    );
    match result {
        Ok(v) => v,
        Err(_) => json!({"error": format!("Not found: {}", term)}),
    }
}

pub fn query_emoji(conn: &Connection, emoji: &str) -> Value {
    let result: Result<Value> = conn.query_row(
        "SELECT id, emoji, label, category, keywords, color, sound, texture, temperature, definition
         FROM emoji_definitions
         WHERE emoji = ?1
         LIMIT 1",
        [emoji],
        |row| {
            let keywords_str: String = row.get(4)?;
            let keywords: Value = serde_json::from_str(&keywords_str).unwrap_or(Value::Array(vec![]));
            Ok(json!({
                "id":          row.get::<_, String>(0)?,
                "emoji":       row.get::<_, String>(1)?,
                "label":       row.get::<_, String>(2)?,
                "category":    row.get::<_, String>(3)?,
                "keywords":    keywords,
                "color":       row.get::<_, Option<String>>(5)?,
                "sound":       row.get::<_, Option<String>>(6)?,
                "texture":     row.get::<_, Option<String>>(7)?,
                "temperature": row.get::<_, Option<String>>(8)?,
                "definition":  row.get::<_, Option<String>>(9)?
            }))
        },
    );
    match result {
        Ok(v) => v,
        Err(_) => json!({"error": format!("Not found: {}", emoji)}),
    }
}

pub fn query_sense(conn: &Connection, sense_id: &str) -> Value {
    let result: Result<(String, String, String, String)> = conn.query_row(
        "SELECT id, name, emoji, description FROM senses WHERE id = ?1 LIMIT 1",
        [sense_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        },
    );

    match result {
        Err(_) => json!({"error": format!("Not found: {}", sense_id)}),
        Ok((id, name, emoji, description)) => {
            let mut stmt = conn
                .prepare(
                    "SELECT id, name, description FROM subcategories WHERE sense_id = ?1 ORDER BY id",
                )
                .unwrap();
            let subcategories: Vec<Value> = stmt
                .query_map([&id], |row| {
                    Ok(json!({
                        "id":          row.get::<_, String>(0)?,
                        "name":        row.get::<_, String>(1)?,
                        "description": row.get::<_, String>(2)?
                    }))
                })
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            json!({
                "id":            id,
                "name":          name,
                "emoji":         emoji,
                "description":   description,
                "subcategories": subcategories
            })
        }
    }
}

pub fn list_atoms(conn: &Connection) -> Value {
    let mut stmt = conn
        .prepare(
            "SELECT id, emoji, label, definition FROM emoji_definitions ORDER BY label",
        )
        .unwrap();
    let atoms: Vec<Value> = stmt
        .query_map([], |row| {
            let def: Option<String> = row.get(3)?;
            let truncated = def.map(|d| {
                if d.len() > 80 {
                    format!("{}…", &d[..80])
                } else {
                    d
                }
            });
            Ok(json!({
                "id":         row.get::<_, String>(0)?,
                "emoji":      row.get::<_, String>(1)?,
                "label":      row.get::<_, String>(2)?,
                "definition": truncated
            }))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    Value::Array(atoms)
}

pub fn list_emojis(conn: &Connection) -> Value {
    let mut stmt = conn
        .prepare(
            "SELECT id, emoji, label, category, keywords, color, sound, texture, temperature, definition
             FROM emoji_definitions ORDER BY label",
        )
        .unwrap();
    let emojis: Vec<Value> = stmt
        .query_map([], |row| {
            let keywords_str: String = row.get(4)?;
            let keywords: Value = serde_json::from_str(&keywords_str).unwrap_or(Value::Array(vec![]));
            Ok(json!({
                "id":          row.get::<_, String>(0)?,
                "emoji":       row.get::<_, String>(1)?,
                "label":       row.get::<_, String>(2)?,
                "category":    row.get::<_, String>(3)?,
                "keywords":    keywords,
                "color":       row.get::<_, Option<String>>(5)?,
                "sound":       row.get::<_, Option<String>>(6)?,
                "texture":     row.get::<_, Option<String>>(7)?,
                "temperature": row.get::<_, Option<String>>(8)?,
                "definition":  row.get::<_, Option<String>>(9)?
            }))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    Value::Array(emojis)
}

pub fn list_senses(conn: &Connection) -> Value {
    let mut stmt = conn
        .prepare("SELECT id, name, emoji, description FROM senses ORDER BY id")
        .unwrap();
    let senses: Vec<Value> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .map(|(id, name, emoji, description)| {
            let mut sub_stmt = conn
                .prepare(
                    "SELECT id, name, description FROM subcategories WHERE sense_id = ?1 ORDER BY id",
                )
                .unwrap();
            let subcategories: Vec<Value> = sub_stmt
                .query_map([&id], |row| {
                    Ok(json!({
                        "id":          row.get::<_, String>(0)?,
                        "name":        row.get::<_, String>(1)?,
                        "description": row.get::<_, String>(2)?
                    }))
                })
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();
            json!({
                "id":            id,
                "name":          name,
                "emoji":         emoji,
                "description":   description,
                "subcategories": subcategories
            })
        })
        .collect();
    Value::Array(senses)
}
