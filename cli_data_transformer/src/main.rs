use cli_data_transformer::parser;
use cli_data_transformer::transform::summarize_active_by_region;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/cardiology_patients.csv");
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    let patients = parser::parse_all(&contents).expect("Failed to parse CSV");

    for p in &patients {
        println!("{:?}", p);
    }

    let summary = summarize_active_by_region(&patients);
    let mut regions: Vec<_> = summary.into_iter().collect();
    regions.sort_by(|a, b| a.0.cmp(&b.0));

    println!();
    for (region, stats) in &regions {
        println!(
            "{}: {} active, total ${:.2}, avg ${:.2}",
            region, stats.count, stats.total_amount, stats.average_amount
        );
    }
}