use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use rust_xlsxwriter::{Workbook, Image, Format, FormatBorder, FormatAlign};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidencePair {
    pub key: String,
    pub left_image: Option<String>,
    pub right_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub pairs: Vec<EvidencePair>,
    pub errors: Vec<String>,
}

#[tauri::command]
fn scan_directory(path: String, left_token: String, right_token: String) -> ScanResult {
    let mut pairs_map: HashMap<String, EvidencePair> = HashMap::new();
    let mut errors = Vec::new();

    let left_suffix = format!("_{}", left_token);
    let right_suffix = format!("_{}", right_token);

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext = ext.to_lowercase();
                if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "bmp" {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let path_str = path.to_string_lossy().to_string();
                        let left_match = file_stem.find(&left_suffix);
                        let right_match = file_stem.find(&right_suffix);

                        if let Some(idx) = left_match {
                            let key = file_stem[..idx].to_string();
                            let pair = pairs_map.entry(key.clone()).or_insert_with(|| EvidencePair {
                                key: key.clone(),
                                left_image: None,
                                right_image: None,
                            });
                            if pair.left_image.is_some() {
                                errors.push(format!("{} の左画像が複数存在します", key));
                            } else {
                                pair.left_image = Some(path_str);
                            }
                        } else if let Some(idx) = right_match {
                            let key = file_stem[..idx].to_string();
                            let pair = pairs_map.entry(key.clone()).or_insert_with(|| EvidencePair {
                                key: key.clone(),
                                left_image: None,
                                right_image: None,
                            });
                            if pair.right_image.is_some() {
                                errors.push(format!("{} の右画像が複数存在します", key));
                            } else {
                                pair.right_image = Some(path_str);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut pairs: Vec<EvidencePair> = pairs_map.into_values().collect();
    pairs.sort_by(|a, b| a.key.cmp(&b.key));

    for pair in &pairs {
        if pair.left_image.is_some() && pair.right_image.is_none() {
            errors.push(format!("{} の右画像が見つかりません", pair.key));
        } else if pair.right_image.is_some() && pair.left_image.is_none() {
            errors.push(format!("{} の左画像が見つかりません", pair.key));
        }
    }

    ScanResult { pairs, errors }
}

#[tauri::command]
fn generate_excel(save_path: String, pairs: Vec<EvidencePair>, left_token: String, right_token: String) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Evidence").map_err(|e| e.to_string())?;

    // Page setup
    worksheet.set_landscape();
    worksheet.set_print_fit_to_pages(1, 0); // 1 page wide

    // Headers
    let header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    worksheet.write_string_with_format(0, 0, "No", &header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(0, 1, "項目", &header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(0, 2, "左画像", &header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(0, 3, "右画像", &header_format).map_err(|e| e.to_string())?;

    worksheet.set_column_width(0, 5.0).map_err(|e| e.to_string())?;
    worksheet.set_column_width(1, 15.0).map_err(|e| e.to_string())?;
    worksheet.set_column_width(2, 45.0).map_err(|e| e.to_string())?;
    worksheet.set_column_width(3, 45.0).map_err(|e| e.to_string())?;

    let cell_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let mut row = 1;
    for (i, pair) in pairs.iter().enumerate() {
        let no = (i + 1) as u32;
        worksheet.write_number_with_format(row, 0, no as f64, &cell_format).map_err(|e| e.to_string())?;
        worksheet.write_string_with_format(row, 1, &pair.key, &cell_format).map_err(|e| e.to_string())?;

        worksheet.set_row_height(row, 200.0).map_err(|e| e.to_string())?;

        // Write borders for cells
        worksheet.write_string_with_format(row, 2, "", &cell_format).map_err(|e| e.to_string())?;
        worksheet.write_string_with_format(row, 3, "", &cell_format).map_err(|e| e.to_string())?;

        let max_w = 315.0;
        let max_h = 260.0;

        if let Some(ref left_path) = pair.left_image {
            let mut image = Image::new(left_path).map_err(|e| e.to_string())?;
            let w = image.width() as f64;
            let h = image.height() as f64;
            if w > 0.0 && h > 0.0 {
                let scale_w = max_w / w;
                let scale_h = max_h / h;
                let scale = f64::min(scale_w, f64::min(scale_h, 1.0));
                image = image.set_scale_width(scale).set_scale_height(scale);
            }
            worksheet.insert_image_with_offset(row, 2, &image, 5, 5).map_err(|e| e.to_string())?;
        }

        if let Some(ref right_path) = pair.right_image {
            let mut image = Image::new(right_path).map_err(|e| e.to_string())?;
            let w = image.width() as f64;
            let h = image.height() as f64;
            if w > 0.0 && h > 0.0 {
                let scale_w = max_w / w;
                let scale_h = max_h / h;
                let scale = f64::min(scale_w, f64::min(scale_h, 1.0));
                image = image.set_scale_width(scale).set_scale_height(scale);
            }
            worksheet.insert_image_with_offset(row, 3, &image, 5, 5).map_err(|e| e.to_string())?;
        }

        row += 1;
    }

    workbook.save(save_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_directory, generate_excel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
