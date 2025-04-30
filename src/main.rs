use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, Deserialize)]
struct Entry {
    sequence: u64,
    symbols: Vec<Vec<Option<String>>>,
}

#[derive(Debug, Serialize)]
struct OutputEntry {
    sequence: u64,
    notation: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://tjl.co/queens-gambit-arg/data-verified.json";
    let data: HashMap<String, Entry> = reqwest::get(url)
        .await?
        .json()
        .await?;

    let mut out = HashMap::new();
    for (id, e) in data {
        let fen = symbols_to_fen(&e.symbols);
        out.insert(id, OutputEntry { sequence: e.sequence, notation: fen });
    }

    let json = serde_json::to_string_pretty(&out)?;
    fs::write("output.json", json).await?;

    println!("Wrote {} entries to output.json", out.len());
    Ok(())
}

fn symbols_to_fen(symbols: &[Vec<Option<String>>]) -> String {
    symbols.iter().map(|row| {
        let mut s = String::new();
        let mut empty = 0;
        for sq in row {
            if let Some(piece) = sq {
                if empty > 0 {
                    s.push_str(&empty.to_string());
                    empty = 0;
                }
                s.push(piece_code(piece));
            } else {
                empty += 1;
            }
        }
        if empty > 0 {
            s.push_str(&empty.to_string());
        }
        s
    })
    .collect::<Vec<_>>()
    .join("/")
}

fn piece_code(p: &str) -> char {
    let (pt, col) = p.split_at(p.len() - 1);
    let base = match pt {
        "K" => 'K',
        "Q" => 'Q',
        "R" => 'R',
        "B" => 'B',
        "N" => 'N',
        "P" => 'P',
        _   => '?',
    };
    if col == "b" { base.to_ascii_lowercase() } else { base }
}
