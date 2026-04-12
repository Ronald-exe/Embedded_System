/// ╔══════════════════════════════════════════════════════════════════╗
/// ║       CONTADOR DE OBJETOS POR COLOR EN VIDEO                    ║
/// ║       Rust + OpenCV (filtro HSV puro)                           ║
/// ╚══════════════════════════════════════════════════════════════════╝
///
/// Uso:
///   cargo run --release -- --video ruta/al/video.mp4
///   cargo run --release -- --youtube "https://www.youtube.com/watch?v=ID"
///   cargo run --release -- --video ruta/al/video.avi --headless
///
/// Controles:
///   q  → Salir y mostrar reporte final

use opencv::{
    core::{self, Mat, Point, Scalar, Size, Vector},
    highgui,
    imgproc,
    videoio::{self, VideoCapture, CAP_PROP_FPS, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH},
    prelude::*,
};
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::time::Instant;

// ══════════════════════════════════════════════
//  CONFIGURACIÓN DE COLORES EN HSV
// ══════════════════════════════════════════════

struct ColorRange {
    name:    &'static str,
    ranges:  Vec<(f64, f64, f64, f64, f64, f64)>,
    display: (f64, f64, f64),
}

fn get_color_ranges() -> Vec<ColorRange> {
    vec![
        ColorRange {
            name: "Rojo",
            ranges: vec![
                (0.0, 10.0, 80.0, 255.0, 50.0, 255.0),
                (160.0, 180.0, 80.0, 255.0, 50.0, 255.0),
            ],
            display: (0.0, 0.0, 220.0),
        },
        ColorRange {
            name: "Naranja",
            ranges: vec![(10.0, 25.0, 80.0, 255.0, 50.0, 255.0)],
            display: (0.0, 140.0, 255.0),
        },
        ColorRange {
            name: "Amarillo",
            ranges: vec![(25.0, 35.0, 80.0, 255.0, 50.0, 255.0)],
            display: (0.0, 220.0, 220.0),
        },
        ColorRange {
            name: "Verde",
            ranges: vec![(35.0, 85.0, 50.0, 255.0, 50.0, 255.0)],
            display: (0.0, 180.0, 50.0),
        },
        ColorRange {
            name: "Azul",
            ranges: vec![(100.0, 130.0, 50.0, 255.0, 50.0, 255.0)],
            display: (220.0, 80.0, 0.0),
        },
        ColorRange {
            name: "Violeta",
            ranges: vec![(130.0, 160.0, 50.0, 255.0, 50.0, 255.0)],
            display: (200.0, 0.0, 180.0),
        },
        ColorRange {
            name: "Blanco",
            ranges: vec![(0.0, 180.0, 0.0, 30.0, 200.0, 255.0)],
            display: (240.0, 240.0, 240.0),
        },
        ColorRange {
            name: "Negro",
            ranges: vec![(0.0, 180.0, 0.0, 255.0, 0.0, 40.0)],
            display: (60.0, 60.0, 60.0),
        },
    ]
}

// ══════════════════════════════════════════════
//  ARGUMENTOS
// ══════════════════════════════════════════════

struct Args {
    video:    Option<String>,
    youtube:  Option<String>,
    skip:     usize,
    min_area: f64,
    headless: bool,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut video    = None;
    let mut youtube  = None;
    let mut skip     = 3usize;
    let mut min_area = 500.0f64;
    let mut headless = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--video"    => { i += 1; video   = Some(args[i].clone()); }
            "--youtube"  => { i += 1; youtube = Some(args[i].clone()); }
            "--skip"     => { i += 1; skip    = args[i].parse().unwrap_or(3); }
            "--min-area" => { i += 1; min_area = args[i].parse().unwrap_or(500.0); }
            "--headless" => { headless = true; }
            _ => {}
        }
        i += 1;
    }

    if video.is_none() && youtube.is_none() {
        eprintln!("[ERROR] Especificá --video <ruta> o --youtube <url>");
        eprintln!("  Ejemplo: cargo run --release -- --video mi_video.mp4");
        std::process::exit(1);
    }

    Args { video, youtube, skip, min_area, headless }
}

// ══════════════════════════════════════════════
//  YOUTUBE
// ══════════════════════════════════════════════

fn get_youtube_stream(url: &str) -> String {
    println!("[INFO] Obteniendo stream de YouTube...");
    let output = Command::new("yt-dlp")
        .args(["-f", "best[ext=mp4]/best", "--get-url", url])
        .output()
        .expect("[ERROR] yt-dlp no encontrado.");

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        eprintln!("[ERROR] yt-dlp falló: {}", err);
        std::process::exit(1);
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

// ══════════════════════════════════════════════
//  DETECCIÓN
// ══════════════════════════════════════════════

fn detect_color_objects(
    frame: &Mat,
    hsv: &Mat,
    color: &ColorRange,
    min_area: f64,
) -> opencv::Result<(Mat, usize)> {
    let mut combined_mask = Mat::zeros(hsv.rows(), hsv.cols(), core::CV_8UC1)?.to_mat()?;

    for &(h_min, h_max, s_min, s_max, v_min, v_max) in &color.ranges {
        let lower = Scalar::new(h_min, s_min, v_min, 0.0);
        let upper = Scalar::new(h_max, s_max, v_max, 0.0);
        let mut mask = Mat::default();
        core::in_range(hsv, &lower, &upper, &mut mask)?;
        core::bitwise_or(&combined_mask.clone(), &mask, &mut combined_mask, &core::no_array())?;
    }

    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_ELLIPSE, Size::new(5, 5), Point::new(-1, -1),
    )?;
    let mut clean_mask = Mat::default();
    imgproc::morphology_ex(&combined_mask, &mut clean_mask, imgproc::MORPH_OPEN, &kernel, Point::new(-1, -1), 2, core::BORDER_CONSTANT, imgproc::morphology_default_border_value()?)?;
    imgproc::morphology_ex(&clean_mask.clone(), &mut clean_mask, imgproc::MORPH_CLOSE, &kernel, Point::new(-1, -1), 2, core::BORDER_CONSTANT, imgproc::morphology_default_border_value()?)?;

    let mut contours: Vector<Vector<Point>> = Vector::new();
    imgproc::find_contours(&clean_mask, &mut contours, imgproc::RETR_EXTERNAL, imgproc::CHAIN_APPROX_SIMPLE, Point::new(0, 0))?;

    let mut result = frame.clone();
    let mut count  = 0usize;
    let draw_color = Scalar::new(color.display.0, color.display.1, color.display.2, 0.0);

    for i in 0..contours.len() {
        let contour = contours.get(i)?;
        let area    = imgproc::contour_area(&contour, false)?;
        if area < min_area { continue; }
        count += 1;
        let rect = imgproc::bounding_rect(&contour)?;
        imgproc::rectangle(&mut result, rect, draw_color, 2, imgproc::LINE_AA, 0)?;
        let label = format!("{} #{}", color.name, count);
        imgproc::put_text(&mut result, &label, Point::new(rect.x, rect.y - 8), imgproc::FONT_HERSHEY_SIMPLEX, 0.55, Scalar::new(0.0, 0.0, 0.0, 0.0), 2, imgproc::LINE_AA, false)?;
        imgproc::put_text(&mut result, &label, Point::new(rect.x, rect.y - 8), imgproc::FONT_HERSHEY_SIMPLEX, 0.55, draw_color, 1, imgproc::LINE_AA, false)?;
    }

    Ok((result, count))
}

// ══════════════════════════════════════════════
//  DASHBOARD
// ══════════════════════════════════════════════

fn draw_dashboard(frame: &mut Mat, counts_total: &HashMap<&str, usize>, counts_frame: &HashMap<&str, usize>, fps: f64, frame_num: usize, color_ranges: &[ColorRange]) -> opencv::Result<()> {
    let panel_h = 40 + color_ranges.len() as i32 * 24 + 30;
    let panel_w = 290i32;
    let mut overlay = frame.clone();
    imgproc::rectangle(&mut overlay, core::Rect::new(8, 8, panel_w, panel_h), Scalar::new(15.0, 15.0, 15.0, 0.0), -1, imgproc::LINE_AA, 0)?;
    let mut blended = Mat::default();
    core::add_weighted(&overlay, 0.75, frame, 0.25, 0.0, &mut blended, -1)?;
    blended.copy_to(frame)?;
    imgproc::put_text(frame, "CONTADOR DE OBJETOS POR COLOR", Point::new(16, 30), imgproc::FONT_HERSHEY_SIMPLEX, 0.52, Scalar::new(0.0, 220.0, 180.0, 0.0), 2, imgproc::LINE_AA, false)?;

    let mut y = 54i32;
    let total_general: usize = counts_total.values().sum();

    for cr in color_ranges {
        let total    = counts_total.get(cr.name).copied().unwrap_or(0);
        let en_frame = counts_frame.get(cr.name).copied().unwrap_or(0);
        if total == 0 && en_frame == 0 { continue; }
        let pct     = if total_general > 0 { total * 100 / total_general } else { 0 };
        let bar_len = (panel_w - 130) * pct as i32 / 100;
        let color   = Scalar::new(cr.display.0, cr.display.1, cr.display.2, 0.0);
        imgproc::put_text(frame, &format!("{:<10}", cr.name), Point::new(16, y), imgproc::FONT_HERSHEY_SIMPLEX, 0.42, color, 1, imgproc::LINE_AA, false)?;
        if bar_len > 0 { imgproc::rectangle(frame, core::Rect::new(110, y - 10, bar_len, 10), color, -1, imgproc::LINE_AA, 0)?; }
        imgproc::put_text(frame, &format!("{} ({} ahora)", total, en_frame), Point::new(110 + bar_len + 4, y), imgproc::FONT_HERSHEY_SIMPLEX, 0.36, Scalar::new(200.0, 200.0, 200.0, 0.0), 1, imgproc::LINE_AA, false)?;
        y += 24;
    }

    imgproc::put_text(frame, &format!("FPS: {:.1}  |  Frame: {}  |  Total: {}", fps, frame_num, total_general), Point::new(16, y + 14), imgproc::FONT_HERSHEY_SIMPLEX, 0.38, Scalar::new(120.0, 120.0, 120.0, 0.0), 1, imgproc::LINE_AA, false)?;
    Ok(())
}

// ══════════════════════════════════════════════
//  REPORTE FINAL
// ══════════════════════════════════════════════

fn build_report(counts_total: &HashMap<&str, usize>, total_frames: usize, elapsed: f64) -> String {
    let mut output = String::new();
    output.push_str(&format!("{}\n", "═".repeat(52)));
    output.push_str("      REPORTE FINAL — CONTADOR DE COLORES\n");
    output.push_str(&format!("{}\n", "═".repeat(52)));
    output.push_str(&format!("  Frames procesados : {}\n", total_frames));
    output.push_str(&format!("  Tiempo total      : {:.1} segundos\n\n", elapsed));
    output.push_str("  OBJETOS DETECTADOS POR COLOR:\n");
    output.push_str(&format!("  {}\n", "─".repeat(40)));

    let total: usize = counts_total.values().sum();
    let mut sorted: Vec<(&&str, &usize)> = counts_total.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (nombre, cantidad) in &sorted {
        let pct   = if total > 0 { *cantidad * 100 / total } else { 0 };
        let barra = "█".repeat(pct / 4);
        output.push_str(&format!("  {:<12} {:>5} detecciones  {} {}%\n", nombre, cantidad, barra, pct));
    }

    if let Some((nombre, _)) = sorted.first() {
        output.push_str(&format!("\n  COLOR MÁS FRECUENTE: {}\n", nombre.to_uppercase()));
    }
    output.push_str(&format!("{}\n", "═".repeat(52)));
    output
}

// ══════════════════════════════════════════════
//  MAIN
// ══════════════════════════════════════════════

fn main() -> opencv::Result<()> {
    let args = parse_args();

    let source = if let Some(ref yt_url) = args.youtube {
        get_youtube_stream(yt_url)
    } else {
        args.video.clone().unwrap()
    };

    println!("[INFO] Abriendo video...");
    let mut cap = VideoCapture::from_file(&source, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        eprintln!("[ERROR] No se pudo abrir el video: {}", source);
        std::process::exit(1);
    }

    let fps_orig = cap.get(CAP_PROP_FPS).unwrap_or(30.0);
    let width    = cap.get(CAP_PROP_FRAME_WIDTH).unwrap_or(0.0) as i32;
    let height   = cap.get(CAP_PROP_FRAME_HEIGHT).unwrap_or(0.0) as i32;
    println!("[INFO] Resolución: {}x{}  |  FPS: {:.1}", width, height, fps_orig);

    let window_name = "Contador de Objetos por Color";

    if !args.headless {
        println!("[INFO] Iniciando detección. Presioná 'q' para salir.\n");
        highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(window_name, 900, 540)?;
    } else {
        println!("[INFO] Modo headless activado. Sin ventana gráfica.");
        println!("[INFO] Iniciando detección...\n");
    }

    let color_ranges  = get_color_ranges();
    let mut counts_total: HashMap<&str, usize> = HashMap::new();
    let mut frame_count  = 0usize;
    let mut processed    = 0usize;
    let start_time       = Instant::now();

    let mut frame     = Mat::default();
    let mut resized   = Mat::default();
    let mut hsv_frame = Mat::default();

    loop {
        if !cap.read(&mut frame)? || frame.empty() {
            println!("[INFO] Fin del video.");
            break;
        }

        frame_count += 1;
        if frame_count % args.skip != 0 { continue; }
        processed += 1;

        if args.headless && processed % 100 == 0 {
            println!("[INFO] Procesando frame {}...", frame_count);
        }

        let target_w = 720i32;
        if frame.cols() > target_w {
            let scale = target_w as f64 / frame.cols() as f64;
            let new_h = (frame.rows() as f64 * scale) as i32;
            imgproc::resize(&frame, &mut resized, Size::new(target_w, new_h), 0.0, 0.0, imgproc::INTER_LINEAR)?;
        } else {
            resized = frame.clone();
        }

        imgproc::cvt_color(&resized, &mut hsv_frame, imgproc::COLOR_BGR2HSV, 0)?;

        let mut counts_frame: HashMap<&str, usize> = HashMap::new();
        let mut result_frame = resized.clone();

        for color in &color_ranges {
            let (detected_frame, count) = detect_color_objects(&resized, &hsv_frame, color, args.min_area)?;
            if count > 0 {
                counts_frame.insert(color.name, count);
                *counts_total.entry(color.name).or_insert(0) += count;
                let mut blended = Mat::default();
                core::add_weighted(&detected_frame, 0.6, &result_frame, 0.4, 0.0, &mut blended, -1)?;
                result_frame = blended;
            }
        }

        let elapsed  = start_time.elapsed().as_secs_f64();
        let fps_real = processed as f64 / elapsed.max(0.001);

        draw_dashboard(&mut result_frame, &counts_total, &counts_frame, fps_real, frame_count, &color_ranges)?;

        if !args.headless {
            highgui::imshow(window_name, &result_frame)?;
            let delay = ((1000.0 / fps_orig) * args.skip as f64) as i32;
            let key = highgui::wait_key(delay.max(1))?;
            if key == b'q' as i32 || key == 27 {
                println!("[INFO] Saliendo por solicitud del usuario.");
                break;
            }
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();

    if !args.headless {
        highgui::destroy_all_windows()?;
    }

    let report = build_report(&counts_total, processed, elapsed);
    print!("{}", report);

    if args.headless {
        use std::io::Write;
        let path = "/root/reporte_colores.txt";
        match std::fs::File::create(path) {
            Ok(mut f) => { let _ = f.write_all(report.as_bytes()); println!("[INFO] Reporte guardado en: {}", path); }
            Err(e)    => eprintln!("[ERROR] No se pudo guardar el reporte: {}", e),
        }
    }

    Ok(())
}