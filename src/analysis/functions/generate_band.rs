// Takes a row number, then returns the amount of each item counter.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Band {
    name: String,
    total_songs_played: u32,
}

impl Band {
    fn increment_total_songs_played(&mut self) {
        self.total_songs_played += 1;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_total_songs_played(&self) -> u32 {
        self.total_songs_played
    }
}

pub fn gen_band(data: String, index: usize) -> Vec<Band> {
    let mut bands_array = Vec::<Band>::new();
    let split_lines = data.split('\n');

    for i in split_lines {
        let vec: Vec<&str> = i.split(',').collect();

        if !i.is_empty() && !vec[index].is_empty() {
            let band = vec[index].to_string();

            let mut present = false;
            let mut band_index = 0;

            while band_index < bands_array.len() {
                if bands_array[band_index].name.eq_ignore_ascii_case(&band) {
                    present = true;
                    break;
                }
                band_index += 1;
            }

            if present {
                bands_array[band_index].increment_total_songs_played();
            } else if !band.starts_with(' ') {
                let new_band = Band {
                    name: band,
                    total_songs_played: 1,
                };

                bands_array.push(new_band);
            }
        }
    }
    bands_array.sort_by(|a, b| b.total_songs_played.cmp(&a.total_songs_played));
    bands_array
}
