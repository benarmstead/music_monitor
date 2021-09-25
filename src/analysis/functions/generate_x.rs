#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
// Struct for X's
pub struct X {
    name: String,
    total_songs_played: u32,
}

// impl for x allowing retreiving of data and incrementing total songs played
impl X {
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

// Takes the csv (data) as a string, and the index which is wanted (e.g. index 1 in the csv rows for x).
// Returns a Vector of X structs
pub fn gen_x(data: String, index: usize) -> Vec<X> {
    // Create x vector
    let mut x_vector = Vec::<X>::new();

    //Split the csv at each new line.
    let split_lines = data.split('\n');

    for i in split_lines {
        let vec: Vec<&str> = i.split(',').collect();

        // If the record in csv is not empty and the index of the record is not empty.
        // e.g. if the record isn't "", and the index wanted of the csv isnt empty e.g. ",,"
        if !i.is_empty() && !vec[index].is_empty() {
            let x = vec[index].to_string();

            let mut present = false;
            let mut x_index = 0;

            // Find the location in the vector of items being looked for the item is in.
            while x_index < x_vector.len() {
                if x_vector[x_index].name.eq_ignore_ascii_case(&x) {
                    present = true;
                    break;
                }
                x_index += 1;
            }

            // If the item is in the vector of items, increment it by one. Else, add it to the vector.
            if present {
                x_vector[x_index].increment_total_songs_played();
            } else if !x.starts_with(' ') {
                let new_x = X {
                    name: x,
                    total_songs_played: 1,
                };

                x_vector.push(new_x);
            }
        }
    }
    // Sort the vector by the amount of counts of the item.
    x_vector.sort_by(|a, b| b.total_songs_played.cmp(&a.total_songs_played));

    // Return the vector
    x_vector
}
