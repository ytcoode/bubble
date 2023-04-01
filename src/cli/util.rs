use anstyle::Style;
use clap::builder::StyledStr;
use std::fmt::Write;

trait Writeln {
    fn writeln(&mut self, s: &str, style: Style);
}

impl<W: Write> Writeln for W {
    fn writeln(&mut self, s: &str, style: Style) {
        if style.is_plain() {
            writeln!(self, "{}", s).unwrap();
        } else {
            writeln!(self, "{}{}{}", style.render(), s, style.render_reset()).unwrap();
        }
    }
}

pub fn before_help() -> impl Into<StyledStr> {
    let bold = Style::new().bold();
    format!(
        "{}{}{}",
        bold.render(),
        r"   ___          __    __    __
  / _ ) __ __  / /   / /   / / ___
 / _  |/ // / / _ \ / _ \ / / / -_)
/____/ \_,_/ /_.__//_.__//_/  \__/",
        bold.render_reset()
    )
}

pub fn after_help() -> impl Into<StyledStr> {
    let mut r = StyledStr::new();

    let plain = Style::new();
    let bold = plain.bold();
    let bold_underline = bold.underline();

    r.writeln("Examples:\n", bold_underline);

    r.writeln(
        "  Start the socks5 proxy server, listening on '0.0.0.0:1080'\n",
        plain,
    );
    r.writeln("    ./bubble --socks5\n", bold);

    r.writeln(
        "  Start the socks5 proxy server, listening on '127.0.0.1:9999'\n",
        plain,
    );
    r.writeln(
        "    ./bubble --socks5 --socks5-ip=127.0.0.1 --socks5-port=9999\n",
        bold,
    );

    r.writeln(
        "  Start the http proxy server, listening on '0.0.0.0:1081'\n",
        plain,
    );
    r.writeln("    ./bubble --http\n", bold);

    r.writeln(
        "  Start the http proxy server, listening on '0.0.0.0:9999'\n",
        plain,
    );
    r.writeln("    ./bubble --http --http-port=9999\n", bold);

    r.writeln(
        "  Start both the socks5 and http proxy servers simultaneously, listening on their default addresses\n",
        plain,
    );
    r.writeln("    ./bubble --socks5 --http\n", bold);

    r
}
