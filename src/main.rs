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
    let mut rook_edge     = HashMap::<u64, String>::new();
    let mut center_map    = HashMap::<String, HashMap<u64, String>>::new();

    for (uuid, entry) in data {
        let fen = symbols_to_fen(&entry.symbols);
        by_uuid.insert(uuid.clone(), OutputEntry { sequence: entry.sequence, notation: fen.clone() });
        by_sequence.insert(entry.sequence, fen.clone());

        match entry.fill.as_str() {
            "darkGray"  => { dark_gray.insert(entry.sequence, fen.clone()); }
            "lightGray" => { light_gray.insert(entry.sequence, fen.clone()); }
            "darkRed"   => { dark_red.insert(entry.sequence, fen.clone()); }
            "lightRed"  => { light_red.insert(entry.sequence, fen.clone()); }
            _ => {}
        }

        let top_edge    = entry.symbols[0].iter().all(|sq| sq.as_deref() == Some("Rw"));
        let bottom_edge = entry.symbols[7].iter().all(|sq| sq.as_deref() == Some("Rw"));
        let left_edge   = (0..8).all(|r| entry.symbols[r][0].as_deref() == Some("Rw"));
        let right_edge  = (0..8).all(|r| entry.symbols[r][7].as_deref() == Some("Rw"));

        if top_edge || bottom_edge || left_edge || right_edge {
            rook_edge.insert(entry.sequence, fen.clone());
        }

        let c33 = entry.symbols[3][3].as_ref();
        let c34 = entry.symbols[3][4].as_ref();
        let c43 = entry.symbols[4][3].as_ref();
        let c44 = entry.symbols[4][4].as_ref();
        if let (Some(p0), Some(p1), Some(p2), Some(p3))
            = (c33, c34, c43, c44)
        {
            if p0 == p1 && p1 == p2 && p2 == p3 {
                center_map
                    .entry(p0.clone())
                    .or_default()
                    .insert(entry.sequence, fen.clone());
            }
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
    fs::write("output/rook_edge.json",     serde_json::to_string_pretty(&rook_edge)?).await?;

    for (piece, map) in center_map {
        let filename = format!("output/center_{}.json", piece);
        fs::write(&filename, serde_json::to_string_pretty(&map)?).await?;
    }

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
