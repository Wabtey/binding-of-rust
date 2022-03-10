use serde_json::from_str;

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

	// the file : /private/student/n/in/fepain/L2_2021-2022/DSB/MoMA/Artist.json
    let json: serde_json::Value =
        serde_json::from_str(the_file)
        	.expect("JSON was not well-formatted");
}
