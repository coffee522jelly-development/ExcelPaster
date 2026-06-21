use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use rust_xlsxwriter::{Workbook, Image, Format, FormatBorder, FormatAlign, Color};
use base64::{engine::general_purpose, Engine as _};
use std::fs;

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
fn read_file_base64(path: String) -> Result<String, String> {
    match fs::read(&path) {
        Ok(bytes) => {
            let encoded = general_purpose::STANDARD.encode(&bytes);
            let ext = std::path::Path::new(&path)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("png")
                .to_lowercase();
            let mime_type = match ext.as_str() {
                "jpg" | "jpeg" => "image/jpeg",
                "bmp" => "image/bmp",
                _ => "image/png",
            };
            Ok(format!("data:{};base64,{}", mime_type, encoded))
        }
        Err(e) => Err(format!("ファイルの読み込みに失敗しました: {}", e)),
    }
}

#[tauri::command]
fn generate_excel(save_path: String, pairs: Vec<EvidencePair>, left_header: String, right_header: String) -> Result<(), String> {
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

    // Configure whole sheet as a grid (width 2.0)
    for col in 0..100 {
        worksheet.set_column_width(col, 2.0).map_err(|e| e.to_string())?;
    }

    let left_header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(Color::Orange);

    let right_header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(Color::Green);

    // Headers (Merged cells to be visible on the grid)
    worksheet.merge_range(0, 0, 0, 2, "No", &header_format).map_err(|e| e.to_string())?;
    worksheet.merge_range(0, 3, 0, 9, "項目", &header_format).map_err(|e| e.to_string())?;
    worksheet.merge_range(0, 10, 0, 30, &left_header, &left_header_format).map_err(|e| e.to_string())?;
    worksheet.merge_range(0, 32, 0, 52, &right_header, &right_header_format).map_err(|e| e.to_string())?;

    let cell_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let left_cell_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(Color::Orange);

    let right_cell_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(Color::Green);

    let mut row = 2; // Start from row 2 due to headers
    for (i, pair) in pairs.iter().enumerate() {
        let no = (i + 1) as u32;

        // Data cells
        worksheet.merge_range(row, 0, row + 20, 2, &no.to_string(), &cell_format).map_err(|e| e.to_string())?;
        worksheet.merge_range(row, 3, row + 20, 9, &pair.key, &cell_format).map_err(|e| e.to_string())?;
        worksheet.merge_range(row, 10, row + 20, 30, "", &left_cell_format).map_err(|e| e.to_string())?;
        worksheet.merge_range(row, 32, row + 20, 52, "", &right_cell_format).map_err(|e| e.to_string())?;

        // Ensure the grid rows are square height
        for r in row..(row + 21) {
            worksheet.set_row_height(r, 12.0).map_err(|e| e.to_string())?; // 12.0 height ~ square for 2.0 width
        }

        // Overlay images on top of the grid
        let max_w = 315.0; // Standard viewable max width
        let max_h = 260.0; // Standard viewable max height

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
            worksheet.insert_image(row, 10, &image).map_err(|e| e.to_string())?;
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
            worksheet.insert_image(row, 32, &image).map_err(|e| e.to_string())?;
        }

        row += 22;
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
        .invoke_handler(tauri::generate_handler![scan_directory, generate_excel, read_file_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
