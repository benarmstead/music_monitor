use crate::analysis;

pub fn start(file_location: String, arg: String) {
    let data = analysis::utils::file::read(file_location);

    // Most played:
    if arg == "-s" {
        // Song's
        analysis::features::most_played_x::display(data, 0);
    } else if arg == "-b" {
        // Band's
        analysis::features::most_played_x::display(data, 1);
    } else if arg == "-a" {
        // Album's
        analysis::features::most_played_x::display(data, 2);
    } else if arg == "-g" {
        // Genre's
        analysis::features::most_played_x::display(data, 3);
    } else if arg == "-l" {
        // Song length's
        analysis::features::most_played_x::display(data, 4);
    } else if arg == "-y" {
        // Year's
        analysis::features::most_played_x::display(data, 6);
    } else if arg == "-d" {
        // Date's
        analysis::features::most_played_x::display(data, 7);
    } else if arg == "-all" {
        for i in 0..7 {
            let except = vec![5];
            if !except.contains(&i) {
                analysis::features::most_played_x::display(data.clone(), i);
            }
        }
    } else {
        println!("Incorrect argument supplied!");
    }
    // Track number's
    //analysis::features::most_played_x::display(data, 5);

    // Time's of day
    //analysis::features::most_played_x::display(data, 8);
}
