use std::path::Path;
use std::fs::File;
use serde::Deserialize;

//from https://github.com/nightraiser/daily-code-rust

#[derive(Debug, Deserialize)]
#[serde(rename_all = “camelCase”)]

struct Artist {

constituent_id: i32,

display_name: String,

artist_bio: String,

nationality: String,

gender: String,

begin_date: i32,

end_date: i32,

wiki_qid: String,

ulan: String

}

fn main() {
    let json_file_path = Path::new(“E:\projects Rust\MoMA\Artists.json”);

    let file = File::open(json_file_path)
            .expect("file not found");

    let artists:Vec<Artist> = serde_json::from_reader(file)
            .expect(“error while reading or parsing”);

    for artist in artists {
        println!("Hello {} number : {} from {}", artist.display_name, artist.constituent_id, artist.nationality)
    }
}