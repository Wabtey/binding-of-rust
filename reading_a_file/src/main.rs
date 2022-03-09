use std::io::Write;

use serde_json::from_str;
use std::fs::File;
use std::io::copy;
use std::io::stdout;

// use serde_json::Value;
// for untyped_example
	use serde_json::{Result, Value};


// typed_example
	use serde::{Deserialize, Serialize};
	// use serde_json::Result;

/*
 * DEBUG with Elzapat <3
 * 
 * https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file
 * https://docs.serde.rs/serde_json/
 */

fn main()
{
    let the_file = r#"
	{
	  "constituent_id": 1,
	  "display_name": "Robert Arneson",
	  "artist_bio": "American, 1930–1992",
	  "nationality": "American",
	  "gender": "Male",
	  "begin_date": 1930,
	  "end_date": 1992,
	  "wiki_qid": "Q1063584",
	  "ulan": "500027998"
	}"#;

	let path = "E:/Code/projects Rust/MoMA/Artists-reformed.json";

    let mut json_file =
		File::open(path).unwrap();
    //let mut stdout = stdout(); //god !


	print!(json_file);
	let mut content = std::fs::read_to_string(path).unwrap().to_strz;

    // let str = &copy(&mut json_file, &mut content)
	// 	.unwrap().to_string();

    // the file : E:/Code/projects Rust/MoMA/Artists.json
	// the file : /private/student/n/in/fepain/L2_2021-2022/DSB/MoMA/Artist.json

//-----test des methodes-----

	// option_example(content);
	// untyped_example(the_file);
	typed_example(content);

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
	wiki_qid: Option<String>, //Vec<String>
	ulan: Option<String>,
}

fn typed_example(str: &str) -> Result<()>
{
	println!("-----typed_example-----");
	
    // Parse the string of data into a Artist object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Artist as output.
    let a: Vec<Artist> = serde_json::from_str(str).unwrap();

	println!("{:?}", a);
	
	println!("-----marker-----");

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", a[0].display_name, a[0].constituent_id);

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