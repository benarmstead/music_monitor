# Cmus Music Monitor Rust

Rust program for Cmus to monitor music listening habits.

This is a rust implementation of my shell version of this program found [here](https://github.com/benarmstead/cmus-music-monitor)

Gets all the metadata of the song being listened to, and writes it to a CSV.

## Program to analyze data

I have written a small python program utilizing matplotlib to analyze and effectively display the data stored in the .csv.

It can be found [here](https://github.com/benarmstead/music-grapher)

## Compiling from source

### Requirements:

- `cargo`

### Build

`cargo build`

This program is intended and designed to be run in the background on boot.

# Output
` <Title>, <Artist>, <Album>, <Genre>, <Song Length>, <Track number>,	<Year>,	<Play date>, <Play time>, <Volume>`

When a tag is not found, then nothing is added except "," , meaning that the colums are always the same for each field.
