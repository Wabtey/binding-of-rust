use serde_json::from_str;
use std::fs::File;
use std::io::copy;
use std::io::stdout;

fn main() {
    let the_file = r#"[
	{
	  "ConstituentID": 1,
	  "DisplayName": "Robert Arneson",
	  "ArtistBio": "American, 1930–1992",
	  "Nationality": "American",
	  "Gender": "Male",
	  "BeginDate": 1930,
	  "EndDate": 1992,
	  "Wiki QID": null,
	  "ULAN": null
	},
	{
	  "ConstituentID": 2,
	  "DisplayName": "Doroteo Arnaiz",
	  "ArtistBio": "Spanish, born 1936",
	  "Nationality": "Spanish",
	  "Gender": "Male",
	  "BeginDate": 1936,
	  "EndDate": 0,
	  "Wiki QID": null,
	  "ULAN": null
	}]"#;

    let mut json_file = File::open("E:/Code/projects Rust/MoMA/Artists.json").unwrap();
    let mut stdout = stdout(); //god !
    let str = &copy(&mut json_file, &mut stdout).unwrap().to_string();


    // the file : E:/Code/projects Rust/MoMA/Artists.json
	// the file : /private/student/n/in/fepain/L2_2021-2022/DSB/MoMA/Artist.json
    let json: serde_json::Value =
        serde_json::from_str(str)
        	.expect("JSON was not well-formatted");

    let artist: Option<&str> = json.get(0)
        //.and_then(|value| value.get(1))
        .and_then(|value| value.get("DisplayName"))
        .and_then(|value| value.as_str());

    print!("artist : {:?}", artist)
}
