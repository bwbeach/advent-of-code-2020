
use std::collections::VecDeque;
use std::fs::read_to_string;

/// Cards are non-negative integers
type Card = usize;

/// The front of the queue is the top of the deck.
type Deck = VecDeque<Card>;

/// The names of the two players
#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

use crate::Player::*;

/// The state of a game is the two decks
#[derive(Debug, Eq, Hash, PartialEq)]
struct Game(Deck, Deck);

impl Game {
    fn winner(&self) -> Player {
        if ! self.0.is_empty() && self.1.is_empty() {
            Player1
        } else if self.0.is_empty() && ! self.1.is_empty() {
            Player2
        } else {
            panic!("game still in progress")
        }
    }

    fn winning_deck<'a>(&'a self) -> &'a Deck {
        if ! self.0.is_empty() && self.1.is_empty() {
            &self.0
        } else if self.0.is_empty() && ! self.1.is_empty() {
            &self.1
        } else {
            panic!("game still in progress")
        }
    }
}

/// Parses one deck, which has a header line (ignored) saying which 
/// plaper it is, followed by one card per line, with the top of
/// the deck first.
fn parse_deck(text: &str) -> Deck {
    text.split("\n")
        .skip(1)
        .filter(|t| ! t.is_empty())
        .map(|t| t.parse::<Card>().unwrap())
        .collect()
}

/// Reads an input file containing two decks
fn read_input(file_name: &str) -> Game {
    let text = read_to_string(file_name).unwrap();
    let parts: Vec<_> = text.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    Game(parse_deck(parts[0]), parse_deck(parts[1]))
}

/// Plays a game, returning the state when the game is done.
fn play_game(init: &Game) -> Game {
    let mut a: Deck = init.0.clone();
    let mut b: Deck = init.1.clone();

    while ! a.is_empty() && ! b.is_empty() {
        let card_a = a.pop_front().unwrap();
        let card_b = b.pop_front().unwrap();
        assert_ne!(card_a, card_b);
        if card_a < card_b {
            b.push_back(card_b);
            b.push_back(card_a);
        } else {
            a.push_back(card_a);
            a.push_back(card_b);
        }
    }

    Game(a, b)
}

/// Returns the score to report for a winning deck.
fn score_winner(deck: &Deck) -> usize {
    let mut result = 0;
    let mut multiplier = deck.len();
    for card in deck {
        result += multiplier * card;
        multiplier -= 1
    }
    result
}

fn main() {
    let sample = read_input("sample.txt");
    let finished_sample = play_game(&sample);
    println!("{:?}", finished_sample);
    println!("{:?}", score_winner(finished_sample.winning_deck()));

    let input = read_input("input.txt");
    println!("Part 1: {:?}", score_winner(play_game(&input).winning_deck()));  // 35370

}
