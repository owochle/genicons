use std::path::PathBuf;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects, Style};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version, styles = CARGO_STYLING)]
pub struct Arguments {
    #[arg(long, short, default_value = "./favicons")]
    pub output_dir: PathBuf,

    #[arg(long, short = 'c', default_value_t = false)]
    /// Do not copy html code in clipboard at the end of the program
    pub no_html_copy: bool,

    #[arg(long, required = false)]
    /// Optional short name of your app
    pub short_name: Option<String>,

    #[arg(long, required = false, default_value_t = String::from("#FFFFFF"))]
    /// Your app color
    pub app_color: String,

    #[arg(long, required = false, default_value_t = String::from("."))]
    /// Your app start url
    pub start_url: String,

    
    #[arg(short, long, required = false, default_value_t = false)]
    /// Removes all prints (except on errors)
    pub silent: bool,

    /// Your master image from which all favicons will be generated
    pub master_image: PathBuf,

    /// Your app name
    pub app_name: String
}

pub(crate) const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub(crate) const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
pub(crate) const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub(crate) const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub(crate) const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
pub(crate) const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
pub(crate) const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

/// Cargo's color style
/// [source](https://github.com/crate-ci/clap-cargo/blob/master/src/style.rs)
pub(crate) const CARGO_STYLING: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);