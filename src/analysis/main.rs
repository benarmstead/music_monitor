use crate::analysis;

pub fn start(file_location: String) {
    let data = analysis::utils::file::read(file_location);

    analysis::features::most_played_bands::display(data);
}
