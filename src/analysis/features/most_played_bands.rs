use crate::analysis;
use piechart::{Chart, Color, Data};
use rand::Rng;

// Takes the csv data as a string and draws a pie chart of the most played bands from it.
pub fn display(data: String) {
    // Convert the csv data into a vector of structs containing artist name and songs played of that artist
    let bands_vector = analysis::functions::generate_band::gen_band(data, 1);

    let mut data = vec![];
    let mut rng = rand::thread_rng();

    // Print all bands played and the amount of songs of which have been played respectively.
    for i in 0..bands_vector.len() {
        let name = bands_vector[i].get_name();
        let total_songs_played = bands_vector[i].get_total_songs_played();
        println!("{} : {}", name, total_songs_played);

        // Add Data struct to data vector for the pie chart to use.
        // Does top most listened to 8 artists.
        if i < 8 {
            data.push(Data {
                label: name,
                value: total_songs_played as f32,
                color: Some(
                    Color::RGB(
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                    )
                    .into(),
                ),
                fill: '•',
            });
        }
    }

    // Create the pie chart and print it.
    Chart::new()
        .radius(15)
        .aspect_ratio(4)
        .legend(true)
        .draw(&data);
}
