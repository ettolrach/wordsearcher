/*
   Copyright 2023 Charlotte Ausel

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0
*/

use anyhow::{bail, Context};
use clap::Parser;
use wordsearcher::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    /// Path to words to find, words separated by a new line ((CR)LF).
    words: std::path::PathBuf,
    #[arg(short, long, value_name = "FILE")]
    /// Path to grid of letters, lines separated by a new line ((CR)LF).
    grid: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let word_path_str = cli.words
        .to_str()
        .with_context(|| "Word path given was not valid UTF-8!".to_string())?;
    let grid_path_str = cli.grid
        .to_str()
        .with_context(|| "Grid path given was not valid UTF-8!".to_string())?;
    let words: Vec<Vec<char>> = std::fs::read_to_string(cli.words.as_path())
        .with_context(|| format!("could not read file `{}`", word_path_str))?
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let text_grid: Vec<Vec<char>> = std::fs::read_to_string(cli.grid.as_path())
        .with_context(|| format!("could not read file `{}`", grid_path_str))?
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let grid = match Grid::from_chars(text_grid) {
        Ok(g) => g,
        Err(GridParseError::InconsistentLineLengths) => bail!("Grid line lengths are inconsistent!"),
        Err(GridParseError::GridEmpty) => bail!("Grid file is empty!"),
    };
    let coords: Vec<Option<[usize; 2]>> = words.iter()
        .map(|w| grid.find_word(&w[..]))
        .collect();

    let word_coord_pair: Vec<(String, Option<[usize; 2]>)> = std::iter::zip(
        words.iter().map(|w| w.iter().collect::<String>()),
        coords
    ).collect();
    for pair in word_coord_pair {
        match pair.1 {
            None => println!("{} not found.", pair.0),
            Some([x, y]) => println!("{} at ({}, {})", pair.0, x, y),
        }
    }
    Ok(())
}
