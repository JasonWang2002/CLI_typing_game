use std::io;
use std::fs;
use std::time;

use rand;
use rand::seq::index::sample;

use clearscreen;
use edit_distance;

fn main() -> io::Result<()> {
	let now = time::SystemTime::now();

	// load text for test
    let contents = fs::read_to_string("./latin.txt")?;
	let text_vec: Vec<&str> = contents.split(' ').collect();
	
	let mut sampler = rand::thread_rng();
	let num_samples = 30;
	let random_sample = sample(&mut sampler, text_vec.len(), num_samples);

	// loop, display test string, collect input
	let stdin = io::stdin();
	let mut input = String::new();

	let mut words_typed = 0;
	let mut chars_typed = 0;
	let mut errors = 0;

    'test: for idx in random_sample.iter() {
        clearscreen::clear().expect("failed to clear screen");
		input.clear();
        println!("Words left: {}", num_samples - words_typed);
		println!("Enter the following word:");
		
		let curr_word =  &text_vec[idx];
		println!("{}\n", curr_word);

		stdin.read_line(&mut input)?;

		if input.eq( "\r\n" ) {
			break 'test;
		}
		
		// this is okay, I've implemented a dynamic programming solution
		// for edit distance before and I don't want to again
		let dist = edit_distance::edit_distance(input.strip_suffix("\r\n").unwrap(), curr_word);

		words_typed += 1;
		chars_typed += curr_word.len();	
		errors += dist;
	}
	
	clearscreen::clear().expect("failed to clear screen");
	println!("Congrats! You finished the Latin typing test! Here are your stats.");

	match now.elapsed() {
		Ok(elapsed) => {
			let secs = elapsed.as_secs() as usize;
			println!("Words per minute: {}", words_typed*60/secs);
			println!("Chars per minute: {}", chars_typed*60/secs);
			println!("Accuracy: {}%", (chars_typed - errors)*100/chars_typed);
		}

		Err(e) => {
			println!("Error: {e:?}");
		}
	}


	Ok(())

}
