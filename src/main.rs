//use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::prelude::*;
use std::io;

const UNICODE_CARDS: [char; 54] =       [' ',  '🂲', '🂳', '🂴', '🂵', '🂶', '🂷', '🂸', '🂹', '🂺', '🂻', '🂽', '🂾', '🂱',
			 		       '🃂', '🃃', '🃄', '🃅', '🃆', '🃇', '🃈', '🃉', '🃊', '🃋', '🃍', '🃎', '🃁',
					       '🃒', '🃓', '🃔', '🃕', '🃖', '🃗', '🃘', '🃙', '🃚', '🃛', '🃝', '🃞', '🃑',
					       '🂢', '🂣', '🂤', '🂥', '🂦', '🂧', '🂨', '🂩', '🂪', '🂫', '🂭', '🂮', '🂡', '🂠'];

const BLANK_CARD: char = UNICODE_CARDS[53];
const FACE_NAMES: &'static [&'static str] = &[" ", " ", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

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
#[derive(Copy, Clone, PartialEq)]
enum Suit {
	None,
	Hearts,
	Diamonds,
	Clubs,
	Spades,
	}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
struct Card {
	value: usize,
	suit: Suit,
	symbol: char,
	id: usize,
	}
	
impl Card {
	fn new(value: usize, suit: Suit) -> Self {
		let a: usize;
		let id;
		let symbol: char;
		match suit {
			Suit::Hearts   => a = 0,
			Suit::Diamonds => a = 1,
			Suit::Clubs    => a = 2,
			Suit::Spades   => a = 3,
			Suit::None     => a = 0,
        		};
        	if value == 0 {
        		symbol = UNICODE_CARDS[0];
        		id = 0; //special handling
        	} else {
			symbol = UNICODE_CARDS[a*13 + value-1];
			id = a*13 + value-1;
			}
		
		Self {value, suit, symbol, id,}
		}
	fn verbose(&self) -> &str {
		&FACE_NAMES[self.value-1]
		}
	fn tell_suit(&self) -> &str {
		let mut name: &str;
		match self.suit {
			Suit::Hearts   => name = "hearts",
			Suit::Diamonds => name = "diamonds",
			Suit::Clubs    => name = "clubs",
			Suit::Spades   => name = "spades",
			Suit::None     => name = "<NONE>",
        		};
        	&name
        	}
	}
	
struct Hand {
	cards: Vec<Card>,
	}

impl Hand {
	fn new() -> Self {
		Self { cards: Vec::new(), }
		}
	
	fn size(&self) -> usize {
		self.cards.len()
		}
		
	fn show(&self) -> String {
		let text: Vec<String> = self.cards.iter().map(|card| card.symbol.to_string()).collect();
        	text.join(FACE_NAMES[0])
		}
		
	fn tease(&self) -> String {
		let text: Vec<String> = self.cards.iter().map(|_| BLANK_CARD.to_string()).collect();
		text.join(FACE_NAMES[0])
		}
		
	fn take(&mut self, card: Card) {
		self.cards.push(card);
		//DO NOT SORT BEFORE CHECKING!!!
		//self.cards.sort_by_key(|card| card.value);
		}
		
	fn compare(&self, ahand1: Hand, ahand2: Hand) {
		//let mut hand1 = self.cards.clone();
		//hand1.extend(ahand1.cards);
		let hand1 = [self.cards.clone(), ahand1.cards].concat();
		let hand2 = [self.cards.clone(), ahand2.cards].concat();
		}
	}
	
struct Deck {
	cards:Vec<Card>,
	}

impl Deck {
	fn new() -> Self {
		let mut cards = Vec::new();
		for i in 2..15 {
			cards.push(Card::new(i, Suit::Hearts));
			cards.push(Card::new(i, Suit::Diamonds));
			cards.push(Card::new(i, Suit::Clubs));
			cards.push(Card::new(i, Suit::Spades));
			}
		cards.shuffle(&mut thread_rng());
		Self { cards }
		}
		
	fn size(&self) -> usize {
		self.cards.len()
		}
		
	fn show(&self) -> String {
		let symbols: Vec<String> = self.cards.iter().map(|card| card.symbol.to_string()).collect();
        	symbols.join(FACE_NAMES[0])
		}
		
	fn deal(&mut self) -> Card {
		self.cards.pop().unwrap()
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








//DEAD CODE WALKIN'

fn check_hand(player_hand: [Card; 2], community: [Card; 5]) -> usize {
	let cards = [player_hand[0], player_hand[1],
		community[0], community[1], community[2], community[3], community[4]]; 

	let mut value = HIGH_CARD;
	let mut kinds: [usize; 15] = [0; 15];
	
	let mut kicker = 0;
	
	let mut count_hearts @ mut count_diamonds @ mut count_clubs @ mut count_spades = 0;
	
	let mut pairs = 0;
	let mut triple = false;
	let mut fourofakind = false;
	let mut flush = false;
	
	let mut straight_counter = 0;

	let mut straight_set: [Card; 5] = [Card::new(0, Suit::None); 5];
	
	//for i in 0..cards.len() {
	for card in cards {
		match card.suit {
			Suit::Hearts => {
				count_hearts += 1;
				//kinds[card.value][0] = card;
				},
			Suit::Diamonds => {
				count_diamonds += 1;
				//kinds[card.value][1] = card
				},
			Suit::Clubs => {
				count_clubs += 1;
				//kinds[card.value][2] = card
				},
			Suit::Spades => {
				count_spades += 1;
				//kinds[card.value][3] = card
				},
			Suit::None => (),
        		};
        	kinds[card.value] += 1;
        	}
        	
        //CHECK IF WE HAVE ONE OF THE FLUSHES
        if count_hearts == 5 || count_diamonds == 5 || count_clubs == 5 || count_spades == 5 {
	        value = value.max(FLUSH);
	        println!("Flush!");
	        //MOVE THIS LOWER! TO BOTTOM
	        /*let kicker = cards.iter()
    			.filter(|x| x.suit == Suit::Hearts)
   			.max_by(|a, b| a.value.cmp(&b.value));*/
	        }
	        
	//SEARCH FOR STRAIGHT
        /*for i in 2..15 {
		match kinds[i] {
			4 => {value = value.max(FOUR_OF_A_KIND); kicker = i},
			3 => triple      = true,
			2 => pairs      += 1,
			_ => (),
			};
		if kinds[i]>1 {
			kicker = kinds[i]-1;
			println!("{} of {}'s!", kicker, FACE_NAMES[i]);
			}
		if kinds[i]>0 && kinds[i-1]>0 {
			straight_counter += 1;
		} else {
			straight_counter = 0;
			}
		if straight_counter >= 4 {
			value = value.max(STRAIGHT);
			println!("Straight");
			
			//kicker = i;
			//println!("Straight with {} high!", FACES[i]);
			//let straight_set = [high_straigh]
			}
		}*/
        	
	//FLUSH DEBUG - UNCOMMENT
	//println!("H {}  D {}  C {}  S {}",count_hearts,count_diamonds,count_clubs,count_spades);
	
	
	
	/*
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
		}}*/
	return value;
	}
	
fn print_table(community: &String, player_hand: &String, dealer_hand: &String, player_cash: i32, pot: u32) {
	clearscreen::clear().unwrap();
	println!("\nDealer hand: {}\n\n\n", dealer_hand);
	println!("   {}  {}\n\n", BLANK_CARD, community);
	println!("Your hand: {}\n", player_hand);
	println!("Your Cash: {}", player_cash);
	println!("This pot:  {}", pot);
	}
	

fn main() {
	let mut rng = rand::thread_rng();
	let mut player_cash: i32 = 1000;
		
	loop {
		//Start a hand
		let mut deck	    = Deck::new();
		let mut community   = Hand::new();
		let mut player_hand = Hand::new();
		let mut dealer_hand = Hand::new();
		let mut bet: i32;
		let mut pot: u32 = 0;
		
		player_hand.take(deck.deal());
		dealer_hand.take(deck.deal());
		player_hand.take(deck.deal());
		dealer_hand.take(deck.deal());
		
		//pre-flop
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask();
		
		//flop
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		community.take(deck.deal());
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask();
		
		//turn
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask();
		
		//river
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask();
		
		//showdown
		print_table(&community.show(), &player_hand.show(), &dealer_hand.show(), player_cash, pot);
		ask();
		
		}
	}
	
