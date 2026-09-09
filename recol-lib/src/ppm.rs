use crate::color::{COLOR_SIZE, Color};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, Read};

/// Reads a P6 PPM file and returns unique colors in first-seen order.
pub fn unique_colors_from_ppm(path: impl AsRef<std::path::Path>) -> io::Result<Vec<Color>> {
    let mut buf = Vec::new();
    BufReader::new(File::open(path)?).read_to_end(&mut buf)?;
    let mut pos = 0;
    let mut next_token = || -> io::Result<String> {
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
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad header token"))
    };
    if next_token()? != "P6" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "not a P6 PPM"));
    }
    let width: usize = next_token()?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad width"))?;
    let height: usize = next_token()?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad height"))?;
    let maxval: usize = next_token()?
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad maxval"))?;
    if maxval > 255 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "16-bit PPM not supported",
        ));
    }
    pos += 1; // skip single whitespace byte after maxval
    let expected = width * height * COLOR_SIZE;
    let data = buf
        .get(pos..pos + expected)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "truncated pixel data"))?;

    // seen: dedup check by packed hex (cheap, avoids requiring Hash/Eq on Color)
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
