# Queens Gambit FEN Converter

A simple Rust CLI tool that:

1. Fetches verified chess positions from `https://tjl.co/queens-gambit-arg/data-verified.json`.
2. Converts each 8×8 symbol array into Forsyth–Edwards Notation (FEN).
3. Saves the resulting JSON mapping IDs to `{ sequence, notation }` in `output.json`.

---

## Prerequisites

- **Rust & Cargo**: Ensure you have Rust (1.60 or later) and Cargo installed. Install via [rustup.rs](https://rustup.rs).
- **Internet Access**: Required to download the JSON data at runtime.

---

## Getting Started

1. **Clone the repository**

   ```bash
   git clone https://github.com/SoZ0/destiny2-2025-community-puzzle-tjil-to-fen.git
   cd destiny2-2025-community-puzzle-tjil-to-fen
   ```

2. **Review dependencies**

   The project uses the following crates (see `Cargo.toml`):

   - `reqwest` for HTTP requests
   - `tokio` for async runtime
   - `serde` / `serde_json` for JSON parsing and serialization
   - `tokio::fs` for async file I/O

---

## Building the Project

To compile the project in release mode:

```bash
cargo build --release
```

Binaries will be located in `target/release/`.

---

## Running the Converter

Execute the compiled binary (or use `cargo run` directly):

```bash
# Using cargo
cargo run --release

# Or direct binary
./target/release/queens-gambit-fen-converter
```

This will:

- Fetch the source JSON.
- Convert each entry to FEN.
- Write the output to `output.json` in the project root.

Upon success, you should see:

```
Wrote <N> entries to output.json
```

---

## Output Format

The `output.json` file maps each position ID to an object with:

- `sequence`: the original sequence number
- `notation`: the FEN string for piece placement only (ranks separated by `/`)

Example:

```json
{
  "0053ec4563fb7324591ab128c3c89949": {
    "sequence": 4058,
    "notation": "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr"
  }
}
```

---

## Customization

- To include full FEN fields (side to move, castling rights, etc.), modify the `symbols_to_fen` function in `src/main.rs`.
- Change the output filename by editing the `fs::write` path.

---

## License

This project is licensed under the MIT License. See `LICENSE` for details.

