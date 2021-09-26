# Music Monitor and Analyser


![workflow](https://github.com/benarmstead/music_monitor/actions/workflows/rust.yml/badge.svg)

Program written in Rust to monitor music listening habbits and creates graphical visualisations of the data collected.

[Supported music players](#support-id)


## Running

### Logger

`./music_monitor <path to CSV you want to log to>`

e.g. `./music_monitor ~/new.csv`

### Analyser and visualiser

`./music_monitor <path to CSV where your logged data is> -d <analysis of choice>`

#### Analysis available to chose from

Most played:

`-s` : Song's

`-b` : Band's

`-a` : Album's

`-g` : Genre's

`-l` : Song length's

`-y` : Year's

`-d` : Date's

`-all` : All of the above

e.g. `./music_monitor ~/new.csv -d -y`

![image](https://user-images.githubusercontent.com/70973680/134824416-9e83ca04-5220-4cf0-b668-8ec0980c947c.png)

![image](https://user-images.githubusercontent.com/70973680/134824099-a9f3a131-5dbe-4291-9356-231961d14517.png)

## Compiling from source

### Requirements:

- `cargo`

### Build

`cargo build --release`

The loggging program is intended and designed to be run in the background on boot.

# Output
`<Title>, <Artist>, <Album>, <Genre>, <Song Length>, <Track number>,	<Year>,	<Play date>, <Play time>, <Volume>`

When a tag is not found, then nothing is added except ",", meaning that the columns are always the same for each field.


## Previous versions

Music Monitor is a derivative of some previous programs I have written. However I have re-written them in Rust for music monitor to ensure it is pure Rust.

Including a shell music monitor [here](https://github.com/benarmstead/music_monitor).

And a music log analyser written in python [here](https://github.com/benarmstead/music-grapher).

## <a name="support-id">Support</a>

Each music player needs unique support for logging.

**The analysis and visualiser will work on and .csv following the above described format.**

Supported Music Players:
- Cmus

**Currently only CMUS is supported.**

***I would greatly appreciate help adding support for logging with other music players.***

***This is only a matter of adding a few vairables, but I don't use any other music player but cmus, so cannot do any extensive testing.***
