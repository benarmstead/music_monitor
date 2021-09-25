use crate::analysis;

pub fn start(file_location: String) {
    let data = analysis::utils::file::read(file_location);

    // Most played:

    // Song's
    analysis::features::most_played_x::display(data.clone(), 0);

    // Band's
    analysis::features::most_played_x::display(data.clone(), 1);

    // Album's
    analysis::features::most_played_x::display(data.clone(), 2);

    // Genre's
    analysis::features::most_played_x::display(data.clone(), 3);

    // Song length's
    //analysis::features::most_played_x::display(data, 4);

    // Track number's
    //analysis::features::most_played_x::display(data, 5);

    // Year's
    analysis::features::most_played_x::display(data.clone(), 6);

    // Date's
    analysis::features::most_played_x::display(data.clone(), 7);

    // Time's of day
    //analysis::features::most_played_x::display(data, 8);
}
