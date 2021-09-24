use crate::analysis;

pub fn start(file_location: String) {
    let data = analysis::utils::file::read(file_location);

    analysis::functions::count_row::count(data, 1);
}
