use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

pub fn write_content_inside_text_block<P>(
    path: P,
    content: &[u8],
    mark: (&str, &str),
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(&path)?;

    let reader = io::BufReader::new(file);

    let mut buf = Vec::new();
    let mut lines = reader.lines();
    let mut inserted = false;

    while let Some(line) = lines.next() {
        let line = line?;
        writeln!(&mut buf, "{}", &line)?;
        if line == mark.0 {
            let _ = buf.pop();
            buf.write_all(content)?;
            if buf.last() != Some(&b'\n') {
                buf.push(b'\n');
            }
            inserted = true;
            break;
        }
    }
    if inserted {
        let mut replace_buf = Vec::new();
        let mut found_end = false;

        while let Some(line) = lines.next() {
            let line = line?;
            writeln!(&mut replace_buf, "{}", &line)?;
            if line == mark.1 {
                found_end = true;
                break;
            }
        }

        if found_end {
            writeln!(&mut buf, "{}", mark.1)?;
        } else {
            buf.write_all(&replace_buf)?;
        }

        while let Some(line) = lines.next() {
            let line = line?;
            writeln!(&mut buf, "{}", &line)?;
        }
    } else {
        write!(&mut buf, "\n{}", mark.0)?;
        buf.write_all(content)?;
        writeln!(&mut buf, "\n{}", mark.1)?;
    }

    buf.flush()?;

    fs::write(&path, &buf)?;

    Ok(())
}

pub fn io_other_error<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}
