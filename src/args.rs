#[derive(Debug, Clone, Copy)]
pub(crate) struct Args {
    pub(crate) width: i32,
    pub(crate) offset_right: i32,
}

fn try_parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut width = 300;
    let mut offset_right = 200;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("width") => {
                width = parser.value()?.parse()?;
            }
            Long("offset-right") => {
                offset_right = parser.value()?.parse()?;
            }
            Long("help") => {
                println!("Usage: waybar-network-applet [--width=N] [--offset-right=N]");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        width,
        offset_right,
    })
}

pub(crate) fn parse_args() -> Args {
    try_parse_args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    })
}
