mod db;
mod query;

fn usage() {
    println!(
        "Resonance Knowledge — The Resonance Grammar

USAGE:
  resonance-knowledge <command> [args]

COMMANDS:
  atom <term>       Query an atom by term
  emoji <char>      Query an emoji by character
  sense <id>        Query a sense with subcategories
  list-atoms        List all atoms
  list-emojis       List all emoji definitions
  list-senses       List all senses with subcategories"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let conn = match db::init_db() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{{\"error\": \"Database init failed: {}\"}}", e);
            std::process::exit(1);
        }
    };

    if args.len() < 2 {
        usage();
        return;
    }

    let result = match args[1].as_str() {
        "atom" => {
            if args.len() < 3 {
                serde_json::json!({"error": "Usage: atom <term>"})
            } else {
                query::query_atom(&conn, &args[2])
            }
        }
        "emoji" => {
            if args.len() < 3 {
                serde_json::json!({"error": "Usage: emoji <char>"})
            } else {
                query::query_emoji(&conn, &args[2])
            }
        }
        "sense" => {
            if args.len() < 3 {
                serde_json::json!({"error": "Usage: sense <id>"})
            } else {
                query::query_sense(&conn, &args[2])
            }
        }
        "list-atoms" => query::list_atoms(&conn),
        "list-emojis" => query::list_emojis(&conn),
        "list-senses" => query::list_senses(&conn),
        unknown => {
            serde_json::json!({"error": format!("Unknown command: {}", unknown)})
        }
    };

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
