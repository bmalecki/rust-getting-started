extern crate termcolor;

type Result<T> = std::result::Result<T, std::io::Error>;

fn green_print(text: &str) -> Result<()> {
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "{}", text)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

    Ok(())
}

fn main() {
    println!("Hello, world!");

    green_print("HELLO").unwrap();
}
