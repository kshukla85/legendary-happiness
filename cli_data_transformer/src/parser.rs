// Parse raw CSV rows into structured data (e.g. a Patient struct).
#[derive(Debug, Clone, PartialEq)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub region: String,
    pub amount: f64,
    pub active: bool,
}

pub fn parse_line(line: &str) -> Result<Patient, String>{
let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != 5 {
        return Err("Invalid number of fields in line".to_string());
    }
    let id: u32 = fields[0].parse().map_err(|_| "Invalid id format".to_string())?;
    let name = fields[1].to_string();
    let region = fields[2].to_string();
    let amount: f64 = fields[3].parse().map_err(|_| "Invalid amount format".to_string())?;
    let active: bool = fields[4].parse().map_err(|_| "Invalid active format".to_string())?;
    Ok(Patient { id, name, region, amount, active })
}


pub fn parse_all(contents: &str) -> Result<Vec<Patient>, String> {
    contents
        .lines()
        .skip(1) //skip header row
        .map(parse_line)
        .collect()
    
}