# cli_data_transformer

A small Rust CLI that reads a patient CSV, parses it into structured records, and produces a per-region summary of active accounts.

## What it does

1. Reads `data/cardiology_patients.csv` (path resolved via `CARGO_MANIFEST_DIR`, so it works regardless of which directory you run the binary from).
2. Parses each row into a `Patient` struct.
3. Prints every parsed record.
4. Aggregates active patients by region (count, total amount, average amount) and prints the summary, sorted alphabetically by region.

## Data format

`data/cardiology_patients.csv` has a header row followed by 5 comma-separated fields per row:

```
id,name,region,amount,active
1,John Whitfield,EU,4200.50,true
```

## Module layout

- **`src/main.rs`** — entry point. Reads the CSV file, calls `parser::parse_all`, prints the raw records, then calls `transform::summarize_active_by_region` and prints the summary.
- **`src/parser.rs`** — defines the `Patient` struct (`id: u32, name: String, region: String, amount: f64, active: bool`) and parses CSV text into `Vec<Patient>`:
  - `parse_line(&str) -> Result<Patient, String>` splits one CSV row on commas, validates it has exactly 5 fields, and parses each field into its typed value.
  - `parse_all(&str) -> Result<Vec<Patient>, String>` skips the header row and parses every remaining line, short-circuiting on the first error.
- **`src/transform.rs`** — data transformations over parsed records.
  - `summarize_active_by_region(&[Patient]) -> HashMap<String, RegionSummary>` filters to `active == true` patients, groups them by `region`, and computes count/total/average `amount` per region.
- **`src/io.rs`** — a `read_file` helper wrapping `std::fs::read_to_string`. Currently unused by `main.rs` (which reads the file directly); kept as the intended home for file I/O helpers, including future stdin/stdout support.
- **`src/lib.rs`** — declares `parser`, `transform`, and `io` as public modules of the `cli_data_transformer` library crate, so `main.rs` can use them via `cli_data_transformer::parser`, etc.

## Running it

```bash
cargo run
```

Produces output like:

```
Patient { id: 1, name: "John Whitfield", region: "EU", amount: 4200.5, active: true }
...

APAC: 1 active, total $6400.00, avg $6400.00
EU: 6 active, total $24657.00, avg $4109.50
US: 3 active, total $10599.99, avg $3533.33
```

## Known limitations

- `parse_line` uses a naive `split(',')` — it will break on quoted fields containing commas or embedded newlines. Fine for this clean, controlled dataset; would need a proper CSV parser (e.g. the `csv` crate) for messier input.
- The CSV path is hardcoded to `data/cardiology_patients.csv` rather than taken as a CLI argument.
- `io::read_file` is defined but not yet wired into `main.rs`.
