use rand::seq::SliceRandom;
use rand::prelude::*;
use std::io;

const UNICODE_CARDS: [char; 54] =       [' ',  '🂲', '🂳', '🂴', '🂵', '🂶', '🂷', '🂸', '🂹', '🂺', '🂻', '🂽', '🂾', '🂱',
			 		       '🃂', '🃃', '🃄', '🃅', '🃆', '🃇', '🃈', '🃉', '🃊', '🃋', '🃍', '🃎', '🃁',
					       '🃒', '🃓', '🃔', '🃕', '🃖', '🃗', '🃘', '🃙', '🃚', '🃛', '🃝', '🃞', '🃑',
					       '🂢', '🂣', '🂤', '🂥', '🂦', '🂧', '🂨', '🂩', '🂪', '🂫', '🂭', '🂮', '🂡', '🂠'];

const BLANK_CARD: char = UNICODE_CARDS[53];
const FACE_NAMES: &'static [&'static str] = &[" ", " ", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

//const JACK: usize = 11;
//const QUEEN: usize = 12;
//const KING: usize = 13;
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
        	/*if rank == 0 {
        		symbol = UNICODE_CARDS[0];
        		id = 0; //special handling
        	} else {*/
			symbol = UNICODE_CARDS[a*13 + rank-1];
			id = a*13 + rank-1;
			//}
		
		Self {rank, suit, symbol, id,}
		}
	/*fn tell_rank(&self) -> &str {
		&FACE_NAMES[self.rank-1]
		}*/
	/*fn tell_suit(&self) -> &str {
		let name: &str;
		match self.suit {
			Suit::Hearts   => name = "hearts",
			Suit::Diamonds => name = "diamonds",
			Suit::Clubs    => name = "clubs",
			Suit::Spades   => name = "spades",
        		};
        	&name
        	}*/
	}

fn is_a_straight(cards: &Vec<Card>) -> bool {
	for i in (0..cards.len()-4).rev() {
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

fn remove_sameranks(cards: &mut Vec<Card>) {
    for i in 0..cards.len() {
        let mut j = i + 1;
        while j < cards.len() {
            if cards[i].rank == cards[j].rank {
                cards.remove(j);
            } else {
                j += 1;
            }
        }
    }
}

fn sort_by_rank(cards: &mut Vec<Card>) {
		cards.sort_by_key(|card| card.rank);
		}
	
fn which_flush(cards: &Vec<Card>) -> Option<Suit> {
	for i in Suit::iter() {
		if cards.iter().filter(|x| x.suit == *i).count() >= 5 {
			return Some(*i);
			}
		}
	return None;	
	}
	
fn compare_hands(player1: &Hand, player2: &Hand) -> usize { // 0 - nobody wins
	let ranking = player1.ranking();
	let ranking_2 = player2.ranking();
	
	let mut cards = [player1.cards.clone(), player2.cards.clone()];
	
	if ranking > ranking_2 {
		return 1; }
	if ranking < ranking_2 {
		return 2; }
	
	if ranking == FLUSH || ranking == STRAIGHT_FLUSH {
		let the_suit = which_flush(&cards[0]);
		for i in 0..2 { cards[i].retain(|&x| x.suit == the_suit.unwrap()); }
		for i in 0..2 { sort_by_rank(&mut cards[i]); }
		}
	
	for i in 0..2 { sort_by_rank(&mut cards[i]); }
	
	if ranking == STRAIGHT || ranking == STRAIGHT_FLUSH {
		for i in 0..2 {
			remove_sameranks(&mut cards[i]);
			while   cards[i].len() > 1 &&
				cards[i][cards[i].len()-2].rank != cards[i][cards[i].len()-1].rank-1 
				{ cards[i].pop(); }
			}}
			
	//sort again because PARANOIA
	for i in 0..2 { sort_by_rank(&mut cards[i]); }
	
	//4 a kind go to the end, then 3, then pairs
	if ranking == ONE_PAIR  ||
	   ranking == TWO_PAIRS ||
	   ranking == THREE_OF_A_KIND ||
	   ranking == FULL_HOUSE ||
	   ranking == FOUR_OF_A_KIND {
	   	for p in 0..2 {
			for n_of_a_kind in 2..5 {
			
			for i in 2..RANKS {
				let mut buffer: Vec<Card> = vec![];
				if cards[p].iter().filter(|x| x.rank == i).count() == n_of_a_kind {
					cards[p].retain(|x| {
						if x.rank == i {
							buffer.push(*x);
							false
						} else {
							true
							}
						});
					sort_by_rank(&mut buffer);
					cards[p].extend(buffer);
					}
				}
				}
			}
		}
	
	for p in 0..2 {
		while cards[p].len() > 5 {
			cards[p].remove(0);
		}}
	
	//COMPARE FROM HIGHEST TO LOWEST
	for (x, y) in cards[0].iter()
				.rev()
			.zip(cards[1].iter()
				.rev())
				.map(|(a, b)| (a.rank, b.rank)) {
			if x > y { return 1; }
			if x < y { return 2; }
			}
	return 0;
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
	
	/*fn size(&self) -> usize {
		self.cards.len()
		}
		
	fn sort(&mut self) {
		self.cards.sort_by_key(|card| card.rank);
		}*/
		
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
				sort_by_rank(&mut flush_set);
				if flush_set[flush_set.len()-1].rank == ACE {
					value = ROYAL_FLUSH;
				}}
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
			ROYAL_FLUSH 	=> return "royal flush!",
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
		
	/*fn size(&self) -> usize {
		self.cards.len()
		}*/
		
	/*fn show(&self) -> String {
		let symbols: Vec<String> = self.cards.iter().map(|card| card.symbol.to_string()).collect();
        	symbols.join(FACE_NAMES[0])
		}*/
		
	fn deal(&mut self) -> Card {
		self.cards.pop().unwrap()
		}
	}

fn ask() -> i32 {
	let mut value: i32;
	loop {
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read your bet");
		if (&input[0..1]).chars().next().expect("Expect letter").is_alphabetic() {
			let a = (&input[0..1]).chars().next().unwrap();
			if a == 'q' {value = -9; break}
			//if a == 'c' {value = -1; break}
			//if a == 'p' {value = -3; break}
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
	
fn print_table(community: &String, player_hand: &String, dealer_hand: &String, player_cash: i32, pot: u32) {
	clearscreen::clear().unwrap();
	println!("\nDealer hand: {}\n\n\n", dealer_hand);
	println!("   {}  {}\n\n", BLANK_CARD, community);
	println!("Your hand: {}\n", player_hand);
	println!("Your Cash: {}", player_cash);
	println!("This pot:  {}", pot);
	}
	

fn main() {
	let mut player_cash: i32 = 1000;
	let ante: u32 = 20; 
		
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
		
		//ante
		pot += 2*ante; player_cash = player_cash.checked_sub((ante) as i32).unwrap();
		
		//pre-flop
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask(); if bet <0 {break;} pot = pot.checked_add((2*bet) as u32).unwrap(); player_cash -= bet;
		
		
		//flop
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		community.take(deck.deal());
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask(); if bet <0 {break;} pot = pot.checked_add((2*bet) as u32).unwrap(); player_cash -= bet;
		
		//turn
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask(); if bet <0 {break;} pot = pot.checked_add((2*bet) as u32).unwrap(); player_cash -= bet;
		
		//river
		/* burn card*/ deck.deal();
		community.take(deck.deal());
		print_table(&community.show(), &player_hand.show(), &dealer_hand.tease(), player_cash, pot);
		bet = ask(); if bet <0 {break;} pot = pot.checked_add((2*bet) as u32).unwrap(); player_cash -= bet;
		
		//showdown
		print_table(&community.show(), &player_hand.show(), &dealer_hand.show(), player_cash, pot);
		let player_7 = community.join(&player_hand);
		let dealer_7 = community.join(&dealer_hand);

		println!("  You have {}", player_7.verbose());
		println!("Dealer has {}", dealer_7.verbose());
		match compare_hands(&player_7, &dealer_7) {
			0 => { println!("It's a draw!");
				let mut input = String::new(); io::stdin().read_line(&mut input).expect("Fail");
				player_cash = player_cash.checked_add((pot/2) as i32).unwrap();
				},
			1 => { println!("You won!");
				let mut input = String::new(); io::stdin().read_line(&mut input).expect("Fail");
				player_cash = player_cash.checked_add(pot as i32).unwrap();
				},
			2 => { println!("You lost...");
				let mut input = String::new(); io::stdin().read_line(&mut input).expect("Fail");
				},
			_ => println!("I AM ERROR."),
			};
		}
	}
	
