#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
// Struct for Bands
pub struct Band {
    name: String,
    total_songs_played: u32,
}

// impl for bands allowing retreiving of data and incrementing total songs played
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

// Takes the csv (data) as a string, and the index which is wanted (e.g. index 1 in the csv rows for bands).
// Returns a Vector of Band structs
pub fn gen_band(data: String, index: usize) -> Vec<Band> {
    // Create bands vector
    let mut bands_vector = Vec::<Band>::new();

    //Split the csv at each new line.
    let split_lines = data.split('\n');

    for i in split_lines {
        let vec: Vec<&str> = i.split(',').collect();

        // If the record in csv is not empty and the index of the record is not empty.
        // e.g. if the record isn't "", and the index wanted of the csv isnt empty e.g. ",,"
        if !i.is_empty() && !vec[index].is_empty() {
            let band = vec[index].to_string();

            let mut present = false;
            let mut band_index = 0;

            // Find the location in the vector of items being looked for the item is in.
            while band_index < bands_vector.len() {
                if bands_vector[band_index].name.eq_ignore_ascii_case(&band) {
                    present = true;
                    break;
                }
                band_index += 1;
            }

            // If the item is in the vector of items, increment it by one. Else, add it to the vector.
            if present {
                bands_vector[band_index].increment_total_songs_played();
            } else if !band.starts_with(' ') {
                let new_band = Band {
                    name: band,
                    total_songs_played: 1,
                };

                bands_vector.push(new_band);
            }
        }
    }
    // Sort the vector by the amount of counts of the item.
    bands_vector.sort_by(|a, b| b.total_songs_played.cmp(&a.total_songs_played));

    // Return the vector
    bands_vector
}
