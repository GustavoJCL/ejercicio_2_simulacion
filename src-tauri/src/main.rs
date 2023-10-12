// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use statrs::distribution::{ChiSquared, ContinuousCDF, Discrete, Poisson};

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    estadistico: BTreeMap<usize, f64>,
    frecuencia_esperada: BTreeMap<usize, f64>,
    frecuencia_observada: BTreeMap<usize, f64>,
    intervalo: BTreeMap<usize, (f64, f64)>,
    poisson: BTreeMap<usize, f64>,
    total_estadistico: f64,
    total_estadistico_tabla: f64,
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
                    if value < min { value.round() } else { min },
                    if value > max { value.round() } else { max },
                )
            });
    let inte_length: f64 = ((max_value - min_value) / m as f64).round();

    let intervalo: BTreeMap<usize, (f64, f64)> = {
        let mut inter: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
        for i in 0..=m + 1 {
            let start = if i != 0 {
                min_value + inte_length * (i - 1) as f64
            } else {
                0.0
            };
            let end = min_value + inte_length * i as f64;
            inter.insert(i as usize, (start, end));
        }
        inter
    };

    let mut frecuencia_observada: BTreeMap<usize, f64> = BTreeMap::new();
    let n = data.len();

    for i in 0..=m {
        frecuencia_observada.insert(i as usize, 0.0);
    }

    for (key, value) in &data {
        for (interval_index, (start, end)) in &intervalo {
            if value > start && value <= end {
                let entry = frecuencia_observada.entry(*interval_index).or_insert(0.0);
                *entry += 1.0;
                break;
            }
        }
    }

    let mut total_events = 0.0;

    for (interval_index, (start, end)) in &intervalo {
        if let Some(frequency) = frecuencia_observada.get(interval_index) {
            let midpoint = (start + end) / 2.0;
            total_events += frequency * midpoint;
        }
    }

    let lambda = total_events / m as f64;
    let poisson_function = Poisson::new(lambda).unwrap();
    let mut poisson: BTreeMap<usize, f64> = BTreeMap::new();
    for (&interval_index, (start, end)) in &intervalo {
        let v = poisson_function.pmf(*start as u64) + poisson_function.pmf(*end as u64);
        // let v = poisson_function.pmf(interval_index as u64);
        poisson.insert(interval_index, v);
    }

    let frecuencia_esperada: BTreeMap<usize, f64> = poisson
        .iter()
        .map(|(key, value)| (key.clone(), *value * n as f64))
        .collect();
    let mut estadistico: BTreeMap<usize, f64> = BTreeMap::new();
    let mut total_estadistico = 0.0;
    for i in 0..poisson.len() {
        let est = (frecuencia_esperada[&i] - frecuencia_observada[&i]).powf(2.0)
            / frecuencia_esperada[&i];
        total_estadistico += est;
        estadistico.insert(i.clone(), est);
    }

    let grados_libertad = (frecuencia_observada.len() - 1) as f64;
    let mut total_estadistico_tabla: f64 = 0.0;
    let mut chi_square_values: BTreeMap<usize, (f64, f64)> = BTreeMap::new();

    if grados_libertad > 0.0 {
        let chi_square_function = ChiSquared::new(grados_libertad).unwrap();
        total_estadistico_tabla = chi_square_function.inverse_cdf(1.0 - nivel_confianza);

        chi_square_values = {
            let mut i = 0.0001;
            let mut count: usize = 0;
            let mut chi_inv_value: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
            while i < nivel_confianza * 1.25 {
                let inv_cdf = chi_square_function.inverse_cdf(i);
                chi_inv_value.insert(count, (1.0 - i, inv_cdf));
                count += 1;
                i += 0.01;
            }
            chi_inv_value
        };
    }

    ResponseData {
        estadistico,
        frecuencia_esperada,
        chi_square_values,
        frecuencia_observada,
        intervalo,
        poisson,
        total_estadistico,
        total_estadistico_tabla,
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, chi_square])
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
