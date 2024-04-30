//! # Command Line Interface (CLI) Utilities
//! This module provides helper functions pertaining to CLI terminal output and behavior.

/// Helper function for styling CLI terminal output.
/// The function uses the `anstyle` crate to create custom styles, which are passed to the
/// 'clap' builder 'Styles' struct:
/// ```
/// clap::builder::Styles::styled()
/// .usage(
///     anstyle::Style::new()
///         .bold()
///         .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
/// )
/// .header(
///     anstyle::Style::new()
///         .bold()
///         .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
/// )
/// ```
/// The function returns the 'Styles' struct, which is then used to style the CLI output.
pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles
        ::styled()
        .usage(
            anstyle::Style
                ::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue)))
        )
        .header(
            anstyle::Style
                ::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue)))
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)))
        )
        .invalid(
            anstyle::Style
                ::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)))
        )
        .error(
            anstyle::Style
                ::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)))
        )
        .valid(
            anstyle::Style
                ::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)))
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White)))
        )
}
