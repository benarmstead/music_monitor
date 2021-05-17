# Cmus Music Monitor Rust

# This program is not fully finished yet. (ETA: Few days), for now use [this](https://github.com/benarmstead/cmus-music-monitor)

Rust program for Cmus to monitor music listening habits.

This is a rust implementation of my shell version of this program found [here](https://github.com/benarmstead/cmus-music-monitor)

Gets all the metadata of the song being listened to, and writes it to a CSV.

## Program to analyse data

I have written a small python program utilizing matplotlib to analyse and effectively display the data stored in the .csv.

It can be found [here](https://github.com/benarmstead/music-grapher)

# Compiling from source

Requirements:

- `rustc` compiler

## Compiling command

`rustc cmus_music_monitor.rs`

## Running after compilation
`./cmus_music_monitor`

# Installation

Use this is you are running an X86 CPU architecture. If not, you can compile it from source [here](# Compiling).

`git clone https://github.com/benarmstead/cmus-music-monitor-rs/`

`cd cmus-music-monitor-rs`

`chmod +x cmus_music_monitor`

`./cmus_music_monitor`

This program is intended and designed to be run in the background on boot.

# Output
` <Title>, <Artist>,	<Album>,	<Genre>,	<Song Length>,	<Track number>,	<Year>,	<Play date>,	<Play time>,	<Volume>`

When a tag is not found, then nothing is added except "," , meaning that the colums are always the same for each field.
