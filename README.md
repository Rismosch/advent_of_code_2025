# Advent of Code 2025

This repo includes solutions to Advent of Code 2025

https://adventofcode.com/

## Requirements

To compile and run the code in this repo, you will need rustc and cargo. I am using version 1.90.0 [download link](https://rust-lang.org/tools/install/)

## Setup

Clone this repo:

    git clone https://github.com/Rismosch/advent_of_code_2025.git
    cd advent_of_code_2025

Then, make a folder that holds all your puzzle inputs:

    mkdir puzzle_input

Copy your puzzle inputs as files into this folder. 1 file for the input of 1 day.

A puzzle input file must have the format as below. Notice how the file has no extension:

    day_<day number>

# Usage

To run a solution, pass the day number as the first argument:

    cargo run -r <day number>

For example this will run the solution for day 1:

    cargo run -r -- 1