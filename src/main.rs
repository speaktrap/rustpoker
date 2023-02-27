use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::io;

//const DECK_SIZE: usize = 52;
const CARDS: &'static [&'static str] = &["  ", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10", "hJ", "hQ", "hK", "hA",
			 		       "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "dJ", "dQ", "dK", "dA",
					       "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10", "cJ", "cQ", "cK", "cA",
					       "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "sJ", "sQ", "sK", "sA"];

fn ask() -> i32 {
	let mut value: i32 = 0;
	loop {
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read your bet");
		if (&input[0..1]).chars().next().expect("Expect letter").is_alphabetic() {
			let a = (&input[0..1]).chars().next().unwrap();
			if a == 'q' {value = -9; break}
			if a == 'c' {value = -1; break}
			if a == 'p' {value = -3; break}
			}
		if (&input[0..]).chars().next().expect("Expect number").is_numeric() {
			value = match input.trim().parse() {
				Ok(num) => num,
				Err(_) => continue,
				};
			if value >= 0 {break}
			}
		}
	return value;
	}
	
fn print_table(community: [usize; 5], player_hand: [usize; 2]) {
	clearscreen::clear().unwrap();
		println!();println!();println!();println!();
		println!("      {}  {}  {}  {}  {}",
			CARDS[community[0]],
			CARDS[community[1]],
			CARDS[community[2]],
			CARDS[community[3]],
			CARDS[community[4]],
			);
		println!();println!();
		println!("Your hand: {} {}", CARDS[player_hand[0]], CARDS[player_hand[1]]);
	}

fn draw_card(dealt: &mut [bool], rng: &mut impl Rng) -> usize {
	let deal = Uniform::from(1..52);
	loop {
		let card = deal.sample(rng);
		if !dealt[card] {
			dealt[card] = true;
			return card;
			}
		}
	}

fn main() {
	let mut rng = rand::thread_rng();
	let mut player_cash: i32 = 0;
		
	loop {
		//Start a hand
		let mut dealt: [bool; 53] = [false; 53];
		let mut community = [0, 0, 0, 0, 0];
		let mut player_hand = [draw_card(&mut dealt, &mut rng), 99];
		let mut dealer_hand = [draw_card(&mut dealt, &mut rng), 99];
		let mut bet: i32;
		player_hand[1] = draw_card(&mut dealt, &mut rng);
		dealer_hand[1] = draw_card(&mut dealt, &mut rng);
		
		//pre-flop
		print_table(community, player_hand);
		bet = ask();
		
		//flop
		/* burn card*/ draw_card(&mut dealt, &mut rng);
		community[0] = draw_card(&mut dealt, &mut rng);
		community[1] = draw_card(&mut dealt, &mut rng);
		community[2] = draw_card(&mut dealt, &mut rng);
		print_table(community, player_hand);
		bet = ask();
		
		//turn
		/* burn card*/ draw_card(&mut dealt, &mut rng);
		community[3] = draw_card(&mut dealt, &mut rng);
		print_table(community, player_hand);
		bet = ask();
		
		//river
		/* burn card*/ draw_card(&mut dealt, &mut rng);
		community[4] = draw_card(&mut dealt, &mut rng);
		print_table(community, player_hand);
		bet = ask();
		
		}
	}
	
