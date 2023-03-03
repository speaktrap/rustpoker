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
const RANKS: usize = 15;

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
	Hearts,
	Diamonds,
	Clubs,
	Spades,
	}

impl Suit {
    fn iter() -> &'static [Suit] {
        &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
struct Card {
	rank: usize,
	suit: Suit,
	symbol: char,
	id: usize,
	}
	
impl Card {
	fn new(rank: usize, suit: Suit) -> Self {
		let a: usize;
		let id;
		let symbol: char;
		match suit {
			Suit::Hearts   => a = 0,
			Suit::Diamonds => a = 1,
			Suit::Clubs    => a = 2,
			Suit::Spades   => a = 3,
        		};
        	if rank == 0 {
        		symbol = UNICODE_CARDS[0];
        		id = 0; //special handling
        	} else {
			symbol = UNICODE_CARDS[a*13 + rank-1];
			id = a*13 + rank-1;
			}
		
		Self {rank, suit, symbol, id,}
		}
	fn verbose(&self) -> &str {
		&FACE_NAMES[self.rank-1]
		}
	fn tell_suit(&self) -> &str {
		let mut name: &str;
		match self.suit {
			Suit::Hearts   => name = "hearts",
			Suit::Diamonds => name = "diamonds",
			Suit::Clubs    => name = "clubs",
			Suit::Spades   => name = "spades",
        		};
        	&name
        	}
	}

fn is_a_straight(cards: &Vec<Card>) -> bool {
	for i in (0..cards.len()-5).rev() {
		let base_rank = cards[0+i].rank;
		if 
		cards.iter().filter(|x| x.rank == base_rank+1).count() > 0 &&
		cards.iter().filter(|x| x.rank == base_rank+2).count() > 0 &&
		cards.iter().filter(|x| x.rank == base_rank+3).count() > 0 &&
		cards.iter().filter(|x| x.rank == base_rank+4).count() > 0
			{return true;}
		}
	return false;
	}
	
fn which_flush(cards: &Vec<Card>) -> Option<Suit> {
	for i in Suit::iter() {
		if cards.iter().filter(|x| x.suit == *i).count() >= 5 {
			return Some(*i);
			}
		}
	return None;	
	}

#[allow(dead_code)]
#[derive(Clone)]
struct Hand {
	cards: Vec<Card>,
	}

impl Hand {
	fn new(cards: Option<Vec<Card>>) -> Self {
		let x = cards.unwrap_or(vec![]);
		Self { cards: x }
		}
	
	fn size(&self) -> usize {
		self.cards.len()
		}
		
	fn sort(&mut self) {
		self.cards.sort_by_key(|card| card.rank);
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
		}
		
	fn ranking(&self) -> usize {
		let mut value = HIGH_CARD;
		let mut pairs_counter @ mut threes_counter = 0;
		
		let mut cards = self.cards.clone();
		cards.sort_by_key(|card| card.rank);
		
		if let Some(flush_suit) = which_flush(&cards) {
			value = FLUSH;
			let mut flush_set = cards.clone();
			flush_set.retain(|x| x.suit == flush_suit);
			if is_a_straight(&flush_set) {
				value = STRAIGHT_FLUSH;
				}
			}
		
		for i in 2..RANKS {
			let count = cards.iter().filter(|x| x.rank == i).count();
			match count {
				4 => value = value.max(FOUR_OF_A_KIND),
				3 => {value = value.max(THREE_OF_A_KIND); threes_counter += 1;},
				2 => {value = value.max(ONE_PAIR); pairs_counter += 1;},
				_ => (),
				};
			}
		
		if pairs_counter > 1 
			{ value = value.max(TWO_PAIRS); }
		if threes_counter > 0 && pairs_counter > 0 || threes_counter > 1
			{ value = value.max(FULL_HOUSE);}
			
		if value < STRAIGHT && is_a_straight(&cards) {
			value = STRAIGHT;
			}	
			
		//TO-DO: Truncate to 5 cards with best rank for draws
		
		return value;
		}
		
	fn verbose(&self) -> &str {
		match self.ranking() {
			HIGH_CARD 	=> return "high card",
			ONE_PAIR 	=> return "one pair",
			TWO_PAIRS 	=> return "two pairs",
			THREE_OF_A_KIND => return "three of a kind",
			STRAIGHT 	=> return "straight",
			FLUSH 		=> return "flush",
			FULL_HOUSE 	=> return "full house",
			FOUR_OF_A_KIND 	=> return "four of a kind",
			STRAIGHT_FLUSH 	=> return "straight flush",
			ROYAL_FLUSH 	=> return "royal flush",
			_ 		=> return "",
			};
		}
	
	fn join(&mut self, input: &Hand) -> Hand {
		let mut buffer = self.cards.clone();
		buffer.extend(input.cards.clone());
		buffer.sort_by_key(|card| card.rank);
		Hand::new(Some(buffer))
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

fn does_hand_win_with(hand_1: Hand, hand_2: Hand) -> bool {
	if hand_1.ranking() > hand_1.ranking() {return true;}
	if hand_1.ranking() < hand_1.ranking() {return false;}
	hand_1.cards;
	return true;	
	}
	
fn print_table(community: &String, player_hand: &String, dealer_hand: &String, player_cash: i32, pot: u32) {
	clearscreen::clear().unwrap();
	println!("\nDealer hand: {}\n\n\n", dealer_hand);
	println!("   {}  {}\n\n", BLANK_CARD, community);
	println!("Your hand: {}\n", player_hand);
	//println!("Your Cash: {}", player_cash);
	//println!("This pot:  {}", pot);
	}
	

fn main() {
	let mut rng = rand::thread_rng();
	let mut player_cash: i32 = 1000;
		
	loop {
		//Start a hand
		let mut deck	    = Deck::new();
		let mut community   = Hand::new(None);
		let mut player_hand = Hand::new(None);
		let mut dealer_hand = Hand::new(None);
		let mut bet: i32;
		let mut pot: u32 = 0;
		
		player_hand.take(deck.deal());
		dealer_hand.take(deck.deal());
		player_hand.take(deck.deal());
		dealer_hand.take(deck.deal());
		
		//pre-flop
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		//bet = ask();
		
		//flop
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		community.take(deck.deal());
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		//bet = ask();
		
		//turn
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		//bet = ask();
		
		//river
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		//bet = ask();
		
		//showdown
		print_table(&community.show(), &player_hand.show(), &dealer_hand.show(), player_cash, pot);
		let player_7 = community.join(&player_hand);
		let dealer_7 = community.join(&dealer_hand);
		println!("   {}", player_7.show());
		println!("vs {}", dealer_7.show());
		println!("You have {}", player_7.verbose());
		if player_7.ranking() > dealer_7.ranking() {println!("You won!");}
		if player_7.ranking() < dealer_7.ranking() {println!("You lost, you fucking whore.");}
		if player_7.ranking() == dealer_7.ranking() {println!("To be continued...");}
		//ask();
		//let mut input = String::new(); io::stdin().read_line(&mut input).expect("Fail");
		if player_7.ranking() == STRAIGHT_FLUSH { let mut input = String::new(); io::stdin().read_line(&mut input).expect("Fail");}
		}
	}
	
