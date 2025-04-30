use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, Deserialize)]
struct Entry {
    sequence: u64,
    symbols: Vec<Vec<Option<String>>>,
    fill: String,
}

#[derive(Debug, Serialize)]
struct OutputEntry {
    sequence: u64,
    notation: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    fs::create_dir_all("output").await?;

    let url = "https://tjl.co/queens-gambit-arg/data-verified.json";
    let data: HashMap<String, Entry> = reqwest::get(url)
        .await?
        .json()
        .await?;

    let mut by_uuid      = HashMap::<String, OutputEntry>::new();
    let mut by_sequence  = HashMap::<u64, String>::new();
    let mut dark_gray    = HashMap::<u64, String>::new();
    let mut light_gray   = HashMap::<u64, String>::new();
    let mut dark_red     = HashMap::<u64, String>::new();
    let mut light_red    = HashMap::<u64, String>::new();

    for (uuid, entry) in data {
        let fen = symbols_to_fen(&entry.symbols);
        by_uuid.insert(uuid.clone(), OutputEntry { sequence: entry.sequence, notation: fen.clone() });
        by_sequence.insert(entry.sequence, fen.clone());

        match entry.fill.as_str() {
            "darkGray"  => { dark_gray.insert(entry.sequence, fen); }
            "lightGray" => { light_gray.insert(entry.sequence, fen); }
            "darkRed"   => { dark_red.insert(entry.sequence, fen); }
            "lightRed"  => { light_red.insert(entry.sequence, fen); }
            _ => {}
        }
    }

    let mut reds  = dark_red.clone();
    reds.extend(light_red.clone());
    let mut grays = dark_gray.clone();
    grays.extend(light_gray.clone());

    fs::write("output/output.json",        serde_json::to_string_pretty(&by_uuid)?).await?;
    fs::write("output/sequence_map.json",  serde_json::to_string_pretty(&by_sequence)?).await?;
    fs::write("output/darkGray.json",      serde_json::to_string_pretty(&dark_gray)?).await?;
    fs::write("output/lightGray.json",     serde_json::to_string_pretty(&light_gray)?).await?;
    fs::write("output/darkRed.json",       serde_json::to_string_pretty(&dark_red)?).await?;
    fs::write("output/lightRed.json",      serde_json::to_string_pretty(&light_red)?).await?;
    fs::write("output/reds.json",          serde_json::to_string_pretty(&reds)?).await?;
    fs::write("output/grays.json",         serde_json::to_string_pretty(&grays)?).await?;

    println!("All files written into ./output/");

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
