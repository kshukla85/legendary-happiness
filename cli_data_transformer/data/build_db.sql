-- Builds cardiology_patients.db from cardiology_patients.csv.
-- Run from the cli_data_transformer/data directory:
--   rm -f cardiology_patients.db && sqlite3 cardiology_patients.db < build_db.sql
--
-- Schema mirrors the Rust `Patient` struct in src/parser.rs:
--   id: u32, name: String, region: String, amount: f64, active: bool

PRAGMA foreign_keys = ON;

CREATE TABLE patients (
    id      INTEGER PRIMARY KEY CHECK (id >= 0),
    name    TEXT    NOT NULL,
    region  TEXT    NOT NULL,
    amount  REAL    NOT NULL CHECK (amount >= 0),
    active  INTEGER NOT NULL CHECK (active IN (0, 1))  -- SQLite has no BOOLEAN; 1 = true
);

CREATE INDEX idx_patients_region_active ON patients (region, active);

-- Load the CSV into a staging table (all TEXT), then cast into the typed table.
.mode csv
.import cardiology_patients.csv patients_staging

INSERT INTO patients (id, name, region, amount, active)
SELECT
    CAST(id AS INTEGER),
    name,
    region,
    CAST(amount AS REAL),
    CASE lower(active) WHEN 'true' THEN 1 WHEN 'false' THEN 0 END
FROM patients_staging;

DROP TABLE patients_staging;

-- Same aggregation as transform::summarize_active_by_region.
CREATE VIEW active_region_summary AS
SELECT
    region,
    COUNT(*)              AS active_count,
    ROUND(SUM(amount), 2) AS total_amount,
    ROUND(AVG(amount), 2) AS avg_amount
FROM patients
WHERE active = 1
GROUP BY region
ORDER BY region;
