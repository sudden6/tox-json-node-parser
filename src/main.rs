/*
    Copyright © 2016 sudden6 <sudden6@gmx.at>
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

// parses tox bootstrap nodes data in json format from stdin and outputs them
// in a format compatible with https://github.com/zetok/tox-check-online-bootstraps

extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::io;
use std::io::prelude::*;
use std::env;

fn main() {
	let mut ipv4_en: bool = false;
	let mut ipv6_en: bool = false;

	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		match args[1].as_str() {
			"-6" => ipv6_en = true,
			"-4" => ipv4_en = true,
			_ => {ipv6_en = true; ipv4_en = true},
		}
	} else {
		ipv4_en = true;
		ipv6_en = true;
	}

	// read data from stdin
	let mut stdin = io::stdin();
	let mut input_json = String::new();
	stdin.read_to_string(&mut input_json).unwrap();
    
	// parse it to json
	let json = Json::from_str(&input_json).unwrap();

	// find the part that holds the bootstrap nodes
	let nodes = json.find("nodes").unwrap().as_array().unwrap();

	for node in nodes {
		let node_obj = node.as_object().unwrap();

		// get relevant parts of the node
		let ipv4 = node_obj.get("ipv4").unwrap().as_string().unwrap();
		let ipv6 = node_obj.get("ipv6").unwrap().as_string().unwrap();
		let port = node_obj.get("port").unwrap();
		let pk   = node_obj.get("public_key").unwrap().as_string().unwrap();
		let name = node_obj.get("maintainer").unwrap().as_string().unwrap();

		if ipv4 != "-" && ipv4_en {
			println!("{} {} {} {}", ipv4, port, pk, name);
		}
		// also print ipv6 part if it exists
		if ipv6 != "-" && ipv6_en {
			println!("{} {} {} {}", ipv6, port, pk, name);
		}
	}
}
