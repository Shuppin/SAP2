//! Colours and styles for terminal output.
//! 
//! This file is a local copy of the inline_colorization crate.
//! 
//! Credits:
//!  Name: inline_colorization
//!  Author: Elias Jonsson
//!  Source: https://crates.io/crates/inline_colorization

pub const style_bold: &str = "\x1B[1m";
pub const style_un_bold: &str = "\x1B[21m";
pub const style_underline: &str = "\x1B[4m";
pub const style_un_underline: &str = "\x1B[24m";
pub const style_reset: &str = "\x1B[0m";

pub const colour_black: &str = "\x1B[30m";
pub const colour_red: &str = "\x1B[31m";
pub const colour_green: &str = "\x1B[32m";
pub const colour_yellow: &str = "\x1B[33m";
pub const colour_blue: &str = "\x1B[34m";
pub const colour_magenta: &str = "\x1B[35m";
pub const colour_cyan: &str = "\x1B[36m";
pub const colour_white: &str = "\x1B[37m";
pub const colour_reset: &str = "\x1B[39m";

pub const bg_black: &str = "\x1B[40m";
pub const bg_red: &str = "\x1B[41m";
pub const bg_green: &str = "\x1B[42m";
pub const bg_yellow: &str = "\x1B[43m";
pub const bg_blue: &str = "\x1B[44m";
pub const bg_magenta: &str = "\x1B[45m";
pub const bg_cyan: &str = "\x1B[46m";
pub const bg_white: &str = "\x1B[47m";
pub const bg_reset: &str = "\x1B[49m";