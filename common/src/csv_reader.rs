use crate::error::{AppError, AppResult};
use csv::ReaderBuilder;
use maidenhead::longlat_to_grid;
use serde::de::DeserializeOwned;

pub fn csv_reader<T: DeserializeOwned + std::fmt::Debug>(
    csv: String,
    skip: usize,
) -> AppResult<Vec<T>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(csv.as_bytes());

    let mut reflist: Vec<T> = Vec::new();
    for result in rdr.records().skip(skip) {
        let req: T = result
            .map_err(AppError::CSVReadError)?
            .deserialize(None)
            .map_err(AppError::CSVReadError)?;
        reflist.push(req);
    }
    Ok(reflist)
}

pub fn maidenhead(lon: f64, lat: f64) -> String {
    longlat_to_grid(lon, lat, 8).unwrap_or("--------".to_string())
}
