use crate::color::{COLOR_SIZE, Color};
use crate::error::{Error, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

// https://ffmpeg.org/ffmpeg-all.html#palettegen-1
pub fn palettegen_from_media(
    path: impl AsRef<std::path::Path>,
    mut max_colors: u8,
) -> crate::error::Result<Vec<Color>> {
    max_colors = max_colors.max(2); // 2..256

    let file_stem = path
        .as_ref()
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| std::io::Error::other("invalid media file name"))?;

    let ppm_path = std::env::temp_dir().join(format!("{file_stem}_{max_colors}.ppm"));

    let output = std::process::Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(path.as_ref())
        .arg("-vf")
        .arg(format!(
            "palettegen=max_colors={max_colors}:reserve_transparent=0"
        ))
        .arg(&ppm_path)
        .output()?;

    if !output.status.success() {
        return Err(crate::error::Error::Ffmpeg(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }

    let colors = unique_colors_from_ppm(&ppm_path);
    let _ = std::fs::remove_file(ppm_path);
    let colors = colors?;

    assert!(max_colors == colors.len() as u8);

    Ok(colors)
}

/// Reads a P6 PPM file and returns unique colors in first-seen order.
fn unique_colors_from_ppm(path: impl AsRef<std::path::Path>) -> Result<Vec<Color>> {
    let mut buf = Vec::new();
    BufReader::new(File::open(path)?).read_to_end(&mut buf)?;
    let mut pos = 0;
    let mut next_token = || -> Result<String> {
        while pos < buf.len() && buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos < buf.len() && buf[pos] == b'#' {
            while pos < buf.len() && buf[pos] != b'\n' {
                pos += 1;
            }
        }
        let start = pos;
        while pos < buf.len() && !buf[pos].is_ascii_whitespace() {
            pos += 1;
        }
        std::str::from_utf8(&buf[start..pos])
            .map(|s| s.to_string())
            .map_err(|_| Error::InvalidPpm("bad header token".into()))
    };
    if next_token()? != "P6" {
        return Err(Error::InvalidPpm("not a P6 PPM".into()));
    }
    let width: usize = next_token()?
        .parse()
        .map_err(|_| Error::InvalidPpm("bad width".into()))?;
    let height: usize = next_token()?
        .parse()
        .map_err(|_| Error::InvalidPpm("bad height".into()))?;
    let maxval: usize = next_token()?
        .parse()
        .map_err(|_| Error::InvalidPpm("bad maxval".into()))?;
    if maxval > 255 {
        return Err(Error::InvalidPpm("16-bit PPM not supported".into()));
    }
    pos += 1; // skip single whitespace byte after maxval
    let expected = width * height * COLOR_SIZE;
    let data = buf
        .get(pos..pos + expected)
        .ok_or_else(|| Error::InvalidPpm("truncated pixel data".into()))?;

    let mut seen: HashSet<u32> = HashSet::with_capacity((width * height).min(1 << 16));
    let mut colors: Vec<Color> = Vec::with_capacity(seen.capacity());
    for px in data.chunks_exact(COLOR_SIZE) {
        let key = (px[0] as u32) << 16 | (px[1] as u32) << 8 | px[2] as u32;
        if seen.insert(key) {
            colors.push(Color::from_rgb(px[0], px[1], px[2]));
        }
    }

    Ok(colors)
}
