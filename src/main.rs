mod frame;
mod logic;

pub const PATH: &str = "./Musical.m3u8";
pub const PATH_1: &str = "./a.txt";
pub const PATH_2: &str = "./b.txt";

// fn main() -> iced::Result {
//     iced::run(update, frame::view)
// }

fn main() {
    // println!("Buscando archivo en: {}", RUTA);
    // println!("{}", logic::get_sorted_lines(RUTA));
    println!("{}", logic::merge_playlists(PATH_1, PATH_2));
}

// Función de actualización mínima
// fn update(_state: &mut (), _message: frame::Message) {
//     // Por ahora no hace nada
// }
