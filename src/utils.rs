use std::{
    fmt::Write,
    fs,
    io::{self, BufRead},
    path::Path,
};

pub fn write_content_inside_text_block<P>(
    path: P,
    content: &str,
    blocks_mark: (&str, &str),
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut buf = String::new();
    let mut lines = reader.lines();
    let mut inserted = false;

    while let Some(line) = lines.next() {
        let line = line?;
        let _ = writeln!(&mut buf, "{}", &line);
        if line == blocks_mark.0 {
            let _ = writeln!(&mut buf, "{}", &content);
            inserted = true;
            break;
        }
    }
    if inserted {
        let mut replace_buf = String::new();
        let mut found_end = false;
        while let Some(line) = lines.next() {
            let line = line?;
            let _ = writeln!(&mut replace_buf, "{}", &line);
            if line == blocks_mark.1 {
                found_end = true;
                break;
            }
        }
        if found_end {
            let _ = writeln!(&mut buf, "{}", blocks_mark.1);
        } else {
            let _ = writeln!(&mut buf, "{}", &replace_buf);
        }
        while let Some(line) = lines.next() {
            let line = line?;
            let _ = writeln!(&mut buf, "{}", &line);
        }
    } else {
        let _ = writeln!(&mut buf, "\n{}{content}\n{}", blocks_mark.0, blocks_mark.1,);
    }

    fs::write(&path, &buf)?;
    Ok(())
}

pub fn io_other_error<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}
