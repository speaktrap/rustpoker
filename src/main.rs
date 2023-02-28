use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::io;

//ASCII friendly card noting
/*
const CARDS: &'static [&'static str] = &["  ", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10", "hJ", "hQ", "hK", "hA",
			 		       "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "dJ", "dQ", "dK", "dA",
					       "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10", "cJ", "cQ", "cK", "cA",
					       "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "sJ", "sQ", "sK", "sA"];
*/

const UNICODE_CARDS: [char; 53] =       [' ',  '🂲', '🂳', '🂴', '🂵', '🂶', '🂷', '🂸', '🂹', '🂺', '🂻', '🂽', '🂾', '🂱',
			 		       '🃂', '🃃', '🃄', '🃅', '🃆', '🃇', '🃈', '🃉', '🃊', '🃋', '🃍', '🃎', '🃁',
					       '🃒', '🃓', '🃔', '🃕', '🃖', '🃗', '🃘', '🃙', '🃚', '🃛', '🃝', '🃞', '🃑',
					       '🂢', '🂣', '🂤', '🂥', '🂦', '🂧', '🂨', '🂩', '🂪', '🂫', '🂭', '🂮', '🂡'];


const FACE_NAMES: &'static [&'static str] = &[" ", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

const JACK: usize = 11;
const QUEEN: usize = 12;
const KING: usize = 13;
const ACE: usize = 14;

const HIGH_CARD: usize = 0;
const ONE_PAIR: usize = 1;
const TWO_PAIRS: usize = 2;
const THREE_OF_A_KIND: usize = 3;
const STRAIGHT: usize = 4;
const FLUSH: usize = 5;
const FULL_HOUSE: usize = 6;
const FOUR_OF_A_KIND: usize = 7;
const STRAIGHT_FLUSH: usize = 8;
const ROYAL_FLUSH: usize = 9;

#[allow(dead_code)]
enum Suit {
	Hearts,
	Diamonds,
	Clubs,
	Spades,
	}

#[allow(dead_code)]
struct Card {
	value: usize,
	suit: Suit,
	symbol: char,
	id: usize,
	}
	
impl Card {
	fn new(value: usize, suit: Suit) -> Self {
		let a: usize;
		match suit {
			Suit::Hearts => a = 0,
			Suit::Diamonds => a = 1,
			Suit::Clubs => a = 2,
			Suit::Spades => a = 3,
        		};
		let symbol: char = UNICODE_CARDS[a*13 + value-1];
		let id = a*13 + value-1;
		Self {value, suit, symbol, id,}
		}
	fn verbose(&self) -> &str {
		&FACE_NAMES[self.value-1]
		}
	}
	


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

fn check_hand(player_hand: [usize; 2], community: [usize; 5]) -> usize {
	let cards = [player_hand[0], player_hand[1],
		community[0], community[1], community[2], community[3], community[4]]; 

	let mut value = HIGH_CARD;
	let mut count_faces: [usize; 14] = [0; 14];
	
	let mut count_hearts = 0;
	let mut count_diamonds = 0;
	let mut count_clubs = 0;
	let mut count_spades = 0;

	let mut pairs = 0;
	let mut triple = false;
	let mut fourofakind = false;
	
	let mut high_straight = 0;
	let mut count_straight = 0;

	for i in cards  {
		match i {       //Check for flush
			1..=13  => count_hearts   += 1,
			14..=26 => count_diamonds += 1,
			27..=39 => count_clubs    += 1,
			40..=52 => count_spades   += 1,
			_ => (),
			};
		count_faces[i%13] += 1;
		}
	
	for i in 1..14 {
		match count_faces[i] {
			4 => fourofakind = true,
			3 => triple      = true,
			2 => pairs      += 1,
			_ => (),
			};
		if count_faces[i]>1 {
			println!("{} of {}'s!", count_faces[i], FACE_NAMES[i]);
			}
		if count_faces[i]>0 && count_faces[i-1]>0 {
			count_straight += 1;
		} else {
			count_straight = 0;
			}
		if count_straight >= 4 {
			value = value.max(STRAIGHT);
			high_straight = i;
			//println!("Straight with {} high!", FACES[i]);
			//let straight_set = [high_straigh]
			}
		}
	if fourofakind {
		value = value.max(FOUR_OF_A_KIND);
		println!("Four of a kind!");
		} else {
	 if triple && pairs > 0 {
	 	value = value.max(FULL_HOUSE);
	  	println!("Full house!");
	  	} else {
	   if triple && pairs ==0 {
	   	value = value.max(THREE_OF_A_KIND);
	    	println!("A triple!");
	    	} else {
	     if pairs == 2 {
	     	value = value.max(TWO_PAIRS);
	      	println!("Two pairs!");
	      	} else {
	       if pairs == 1 {
	       	value = value.max(ONE_PAIR);
	        println!("A pair!");
	        }}}}}
	//println!("H {}  D {}  C {}  S {}",count_hearts,count_diamonds,count_clubs,count_spades);
	if count_hearts == 5 || count_diamonds == 5 || count_clubs == 5 || count_spades == 5 {
		if high_straight > 0 {
			value = value.max(STRAIGHT_FLUSH);
			println!("Straight flush!");
		    	} else {
			value = value.max(FLUSH);
			println!("Flush!");
			}
		} else {
	if high_straight > 0 {
	    	println!("Straight with {} high!", high_straight);
		}}
	return value;
	}
	
fn print_table(community: [usize; 5], player_hand: [usize; 2], player_cash: i32, pot: u32) -> usize {
	let mut value = 0;
	clearscreen::clear().unwrap();
	value = check_hand(player_hand, community);
	println!();println!();println!();println!();
	println!("   🂠  {} {} {} {} {}",
		UNICODE_CARDS[community[0]],
		UNICODE_CARDS[community[1]],
		UNICODE_CARDS[community[2]],
		UNICODE_CARDS[community[3]],
		UNICODE_CARDS[community[4]],
		);
	println!();println!();
	println!("Your hand: {} {}", UNICODE_CARDS[player_hand[0]], UNICODE_CARDS[player_hand[1]]);
	println!();
	println!("Your Cash: {}", player_cash);
	println!("This pot:  {}", pot);
	return value;
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
	let mut player_cash: i32 = 1000;
		
	loop {
		//Start a hand
		let mut dealt: [bool; 53] = [false; 53];
		let mut community = [0, 0, 0, 0, 0];
		let mut player_hand = [draw_card(&mut dealt, &mut rng), 99];
		let mut dealer_hand = [draw_card(&mut dealt, &mut rng), 99];
		let mut bet: i32;
		let mut pot: u32 = 0;
		player_hand[1] = draw_card(&mut dealt, &mut rng);
		dealer_hand[1] = draw_card(&mut dealt, &mut rng);
		
		//pre-flop
		print_table(community, player_hand, player_cash, pot);
		//bet = ask();
		
		//flop
		/* burn card*/ draw_card(&mut dealt, &mut rng);
		community[0] = draw_card(&mut dealt, &mut rng);
		community[1] = draw_card(&mut dealt, &mut rng);
		community[2] = draw_card(&mut dealt, &mut rng);
		print_table(community, player_hand, player_cash, pot);
		//bet = ask();
		
		//turn
		/* burn card*/ draw_card(&mut dealt, &mut rng);
		community[3] = draw_card(&mut dealt, &mut rng);
		print_table(community, player_hand, player_cash, pot);
		//bet = ask();
		
		//river
		/* burn card*/ draw_card(&mut dealt, &mut rng);
		community[4] = draw_card(&mut dealt, &mut rng);
		if print_table(community, player_hand, player_cash, pot) == STRAIGHT_FLUSH {
			bet = ask();
			}
		
		}
	}
	
