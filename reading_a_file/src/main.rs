use std::fs;

use serde_json::from_str;

// for untyped_example
	use serde_json::{Result, Value};

// typed_example
	use serde::{Deserialize, Serialize};
	// use serde_json::Result;

// to read a json file
	// extern crate serialize;
	// use serialize::json;

/*
 * DEBUG with Elzapat <3
 * 
 * https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file
 * https://docs.serde.rs/serde_json/
 * https://doc.rust-lang.org/std/io/trait.Read.html
 * https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
 */

fn main()
{
    let _the_file = r#"
	{
	  "constituent_id": 1,
	  "display_name": "Robert Arneson",
	  "artist_bio": "American, 1930–1992",
	  "nationality": "American",
	  "gender": "Male",
	  "begin_date": 1930,
	  "end_date": 1992,
	  "wiki_qid": null,
	  "ulan": null
	}"#;

	//TODO : put a .json example in the reading_a_file to make this tutorial independent
	let path = "/private/student/n/in/fepain/R/ArtManipulation/MoMA/Artists-reformed.json";
	// the file : E:/Code/projects Rust/MoMA/Artists-reformed.json
	// the file : /private/student/n/in/fepain/L2_2021-2022/DSB/MoMA/Artist.json

	let content = fs::read_to_string(path)
		.expect("Unable to read file");

	// println!("start {} end", content);
    // let mut stdout = stdout(); //god !

//-----test des methodes-----

	// option_example(content);
	// untyped_example(the_file);
	typed_example(&content);

//-----------demo------------
	println!("-----This is a freaking demo-----");

	typed_example_person();

 }



//--------------------OPTION_EXAMPLE----------------------------------------

 
 fn option_example(str: &str)
 {
	println!("-----option_example-----");
	
	// let root: Value = serde_json::from_str(the_file)?;
	// let artist: Option<&str> = root.get(0)
    //     .and_then(|value| value.get(1))
    //     .and_then(|value| value.get("DisplayName"))
    //     .and_then(|value| value.as_str());

	// print!("artist : {:?}", artist)


	let json: serde_json::Value =
		serde_json::from_str(str)
			.expect("JSON was not well-formatted");

    let artist: Option<&str> = json.get(0)
        .and_then(|value| value.get(1))
        .and_then(|value| value.get("DisplayName"))
        .and_then(|value| value.as_str());

    print!("artist : {:?}", artist)
 }


//-------------------UNTYPED_EXAMPLE----------------------------------------


 fn untyped_example(str: &str) -> Result<()> 
 {
	println!("-----untyped_example-----");

	// Parse the string of data into serde_json::Value.
	let v: Value = serde_json::from_str(str)?;
	println!("here the value you get from here : {}", v);

	// Access parts of the data by indexing with square brackets.
	println!("Please call {} at the number {}", v["DisplayName"], v["ConstituentID"]);

	Ok(())
 }


//---------------------TYPED_EXAMPLE----------------------------------------


 #[derive(Serialize, Deserialize, Debug)]
struct Artist {
	constituent_id: i128,
	display_name: String,
	artist_bio: Option<String>,
	nationality: Option<String>,
	gender: Option<String>,
	begin_date: i16,
	end_date: i16,
	wiki_qid: Option<String>, 
	ulan: Option<String>
}

fn typed_example(str: &str) -> Result<()>
{
	println!("-----typed_example-----");
	
    let artists: Vec<Artist> = serde_json::from_str(str).unwrap();

	// println!("{:?}", artists);
	
	println!("--------marker---------");

	let artist_nationality: &str=
		match &artists[0].nationality {
			Some(s) => s,
			None => "",
		};

    let foo = "Please introduce display_name who's a nationality";
	let mut artist1 = foo.replace("display_name", &artists[0].display_name);
	artist1 = artist1.replace("nationality", artist_nationality);

	fs::write("/private/student/n/in/fepain/R/binding-of-rust/reading_a_file/tmp/foo.txt",
			 artist1)
		.expect("Unable to write file");

    Ok(())
}

//---------------------TYPED_EXAMPLE_ORI------------------------------------

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example_person() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}