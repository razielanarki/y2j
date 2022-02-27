extern crate serde_json;
extern crate serde_yaml;

use std::io::{self};

fn main()
{
    let value: serde_json::Value = serde_yaml::from_reader( io::stdin() ).unwrap();
    println!( "{}", value.to_string() );
}
