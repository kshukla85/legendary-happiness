// Transform parsed records (filter, aggregate, reshape, etc).
use std::collections::HashMap;
use crate::parser::Patient;

#[derive(Debug)]
pub struct RegionSummary {
    pub count: usize,
    pub total_amount: f64,
    pub average_amount: f64,
}

pub fn summarize_active_by_region(patients: &[Patient]) -> HashMap<String, RegionSummary> {
    let mut totals: HashMap<String, (usize, f64)> = HashMap::new();

    for p in patients.iter().filter(|p| p.active) {
        let entry = totals.entry(p.region.clone()).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += p.amount;
    }

    totals
        .into_iter()
        .map(|(region, (count, total_amount))| {
            let average_amount = total_amount / count as f64;
            (region, RegionSummary { count, total_amount, average_amount })
        })
        .collect()
}
