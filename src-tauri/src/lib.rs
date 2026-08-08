use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use walkdir::WalkDir;
use rust_xlsxwriter::{Workbook, Image, Format, FormatBorder, FormatAlign};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use chrono::Local;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidencePair {
    pub key: String,
    pub left_image: Option<String>,
    pub right_image: Option<String>,
    pub extra_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub pairs: Vec<EvidencePair>,
    pub errors: Vec<String>,
}

#[tauri::command]
fn scan_directory(path: String, left_token: String, right_token: String, extra_token: String) -> ScanResult {
    let mut pairs_map: HashMap<String, EvidencePair> = HashMap::new();
    let mut errors = Vec::new();

    let left_suffix = left_token;
    let right_suffix = right_token;
    let extra_suffix = extra_token;

    // To prevent prefix collision (e.g. "_a" matching before "_aa"), we sort suffixes by length descending
    let mut suffixes = vec![
        (&left_suffix, "left"),
        (&right_suffix, "right"),
        (&extra_suffix, "extra"),
    ];
    suffixes.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext = ext.to_lowercase();
                if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "bmp" {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let path_str = path.to_string_lossy().to_string();

                        for (suffix, token_type) in &suffixes {
                            if let Some(idx) = file_stem.rfind(*suffix) {
                                // Ensure it exactly matches the end of the stem
                                if idx + suffix.len() == file_stem.len() {
                                    let key = file_stem[..idx].to_string();
                                    let pair = pairs_map.entry(key.clone()).or_insert_with(|| EvidencePair {
                                        key: key.clone(),
                                        left_image: None,
                                        right_image: None,
                                        extra_image: None,
                                    });

                                    match *token_type {
                                        "left" => {
                                            if pair.left_image.is_some() {
                                                errors.push(format!("{} の左画像が複数存在します", key));
                                            } else {
                                                pair.left_image = Some(path_str);
                                            }
                                        }
                                        "right" => {
                                            if pair.right_image.is_some() {
                                                errors.push(format!("{} の右画像が複数存在します", key));
                                            } else {
                                                pair.right_image = Some(path_str);
                                            }
                                        }
                                        "extra" => {
                                            if pair.extra_image.is_some() {
                                                errors.push(format!("{} の補足画像が複数存在します", key));
                                            } else {
                                                pair.extra_image = Some(path_str);
                                            }
                                        }
                                        _ => {}
                                    }
                                    break; // Only match the longest suffix once per file
                                }
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
fn rename_files(file_paths: Vec<String>, suffix: String) -> Result<(), String> {
    for path_str in file_paths {
        let path = std::path::Path::new(&path_str);
        if !path.is_file() {
            continue;
        }

        let parent = path.parent().unwrap_or(std::path::Path::new(""));

        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            let new_stem = format!("{}{}", stem, suffix);
            let mut new_filename = new_stem;
            if !ext.is_empty() {
                new_filename = format!("{}.{}", new_filename, ext);
            }

            let new_path = parent.join(new_filename);

            if let Err(e) = fs::rename(&path, &new_path) {
                return Err(format!("ファイル '{}' のリネームに失敗しました: {}", path_str, e));
            }
        }
    }
    Ok(())
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
fn generate_excel(
    save_path: String,
    pairs: Vec<EvidencePair>,
    left_header: String,
    right_header: String,
    extra_header: String,
    left_color: String,
    right_color: String,
    extra_color: String,
    subject_color: String,
    sheet_name: String,
    subject: String,
    skip_images: bool
) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let safe_sheet_name = if sheet_name.trim().is_empty() { "Evidence" } else { &sheet_name };
    worksheet.set_name(safe_sheet_name).map_err(|e| e.to_string())?;

    // Page setup
    worksheet.set_landscape();
    worksheet.set_print_fit_to_pages(1, 0); // 1 page wide

    // Headers
    let header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    // Configure columns explicitly instead of a grid, avoiding merge_range
    worksheet.set_column_width(0, 6.0).map_err(|e| e.to_string())?; // No
    worksheet.set_column_width(1, 14.0).map_err(|e| e.to_string())?; // 項目
    worksheet.set_column_width(2, 42.0).map_err(|e| e.to_string())?; // Left Image
    worksheet.set_column_width(3, 42.0).map_err(|e| e.to_string())?; // Right Image
    worksheet.set_column_width(4, 42.0).map_err(|e| e.to_string())?; // Extra Image

    let left_header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(left_color.as_str());

    let right_header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(right_color.as_str());

    let extra_header_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(extra_color.as_str());

    // Subject (Top left)
    let subject_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Left)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(subject_color.as_str())
        .set_font_color("#000000")
        .set_font_size(14);
    worksheet.write_string_with_format(0, 0, &subject, &subject_format).map_err(|e| e.to_string())?;

    let subject_blank_format = Format::new()
        .set_background_color(subject_color.as_str());
    worksheet.write_string_with_format(0, 1, "", &subject_blank_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(0, 2, "", &subject_blank_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(0, 3, "", &subject_blank_format).map_err(|e| e.to_string())?;

    // Add current date to the top right
    let current_date = Local::now().format("%Y/%m/%d").to_string();
    let date_format = Format::new()
        .set_align(FormatAlign::Right)
        .set_align(FormatAlign::VerticalCenter)
        .set_background_color(subject_color.as_str())
        .set_font_color("#000000");
    worksheet.write_string_with_format(0, 4, &current_date, &date_format).map_err(|e| e.to_string())?;

    // Headers
    worksheet.write_string_with_format(1, 0, "No", &header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(1, 1, "項目", &header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(1, 2, &left_header, &left_header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(1, 3, &right_header, &right_header_format).map_err(|e| e.to_string())?;
    worksheet.write_string_with_format(1, 4, &extra_header, &extra_header_format).map_err(|e| e.to_string())?;

    let cell_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let empty_cell_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let mut row = 3;
    for (i, pair) in pairs.iter().enumerate() {
        let no = (i + 1) as u32;

        // Ensure row height is large enough for the images
        worksheet.set_row_height(row, 252.0).map_err(|e| e.to_string())?;

        // Data cells
        worksheet.write_string_with_format(row, 0, &no.to_string(), &cell_format).map_err(|e| e.to_string())?;
        worksheet.write_string_with_format(row, 1, &pair.key, &cell_format).map_err(|e| e.to_string())?;
        worksheet.write_string_with_format(row, 2, "", &empty_cell_format).map_err(|e| e.to_string())?;
        worksheet.write_string_with_format(row, 3, "", &empty_cell_format).map_err(|e| e.to_string())?;
        worksheet.write_string_with_format(row, 4, "", &empty_cell_format).map_err(|e| e.to_string())?;

        // Images layout
        let max_w = 315.0; // Standard viewable max width
        let max_h = 260.0; // Standard viewable max height

        if !skip_images {
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
                worksheet.insert_image(row, 2, &image).map_err(|e| e.to_string())?;
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
                worksheet.insert_image(row, 3, &image).map_err(|e| e.to_string())?;
            }

            if let Some(ref extra_path) = pair.extra_image {
                let mut image = Image::new(extra_path).map_err(|e| e.to_string())?;
                let w = image.width() as f64;
                let h = image.height() as f64;
                if w > 0.0 && h > 0.0 {
                    let scale_w = max_w / w;
                    let scale_h = max_h / h;
                    let scale = f64::min(scale_w, f64::min(scale_h, 1.0));
                    image = image.set_scale_width(scale).set_scale_height(scale);
                }
                worksheet.insert_image(row, 4, &image).map_err(|e| e.to_string())?;
            }
        }

        row += 2; // Leave a blank row as a gap between entries
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
        .invoke_handler(tauri::generate_handler![scan_directory, generate_excel, read_file_base64, rename_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
