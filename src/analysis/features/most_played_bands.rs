use crate::analysis;
use piechart::{Chart, Color, Data};
use rand::Rng;

pub fn display(data: String) {
    let bands_array = analysis::functions::generate_band::gen_band(data, 1);
    let mut data = vec![];
    let mut rng = rand::thread_rng();

    for i in 0..bands_array.len() {
        let name = bands_array[i].get_name();
        let total_songs_played = bands_array[i].get_total_songs_played();
        println!("{} : {}", name, total_songs_played);
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

    Chart::new()
        .radius(15)
        .aspect_ratio(4)
        .legend(true)
        .draw(&data);
}
