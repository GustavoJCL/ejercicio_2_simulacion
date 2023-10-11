// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::BTreeMap, f64::INFINITY};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    estadistico: BTreeMap<usize, f64>,
    frecuencia_esperada: BTreeMap<usize, f64>,
    frecuencia_observada: BTreeMap<usize, f64>,
    intervalo: BTreeMap<usize, f64>,
    poison: BTreeMap<usize, f64>,
    error: BTreeMap<usize, f64>,
    total_estadistico: f64,
    total_estadistico_total: f64,
    chi_square_values: BTreeMap<usize, (f64, f64)>,
}

#[tauri::command(rename_all = "snake_case")]
fn chi_square(data_string: String, nivel_confianza: f64) -> ResponseData {
    let data: BTreeMap<usize, f64> = csv_to_data(data_string);
    let m: i64 = (data.len() as f64).powf(0.5) as i64;

    let (min_value, max_value) =
        data.values()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &value| {
                (
                    if value < min { value } else { min },
                    if value > max { value } else { max },
                )
            });
    let inte_length: f64 = (max_value - min_value) / m as f64;

    let intervalo: BTreeMap<usize, (f64, f64)> = {
        let mut inter: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
        for i in 0..=m + 1 {
            let start = if i != 0 {
                min_value + inte_length * (i - 1) as f64
            } else {
                0.0
            };
            let end = if i != m {
                min_value + inte_length * i as f64
            } else {
                INFINITY
            };
            inter.insert(i as usize, (start, end));
        }
        inter
    };

    let frecuencia_observada: BTreeMap<usize, f64> = {
        let mut frec: BTreeMap<usize, f64> = BTreeMap::new();
        for (key, value) in &data {
            for (interval_index, (start, end)) in &intervalo {
                if value >= start && value <= end {
                    let entry = frec.entry(*interval_index).or_insert(0.0);
                    *entry += 1.0;
                    break;
                }
            }
        }
    };

    ResponseData {}
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn csv_to_data(csv_data: String) -> BTreeMap<usize, f64> {
    let mut map: BTreeMap<usize, f64> = BTreeMap::new();

    for line in csv_data.lines() {
        let line = line.trim_end_matches(','); // Remove trailing comma
        let parts: Vec<&str> = line.split(',').collect();
        if let (Some(index), Some(value)) = (parts.get(0), parts.last()) {
            if let (Ok(index), Ok(value)) = (index.parse::<usize>(), value.parse::<f64>()) {
                map.insert(index - 1, value);
            }
        }
    }

    // for (key, value) in &map {
    //     println!("{}: {}", key, value);
    // }
    // println!("{:#?}", map);
    //     println!("{} ", map.len());
    map
}
