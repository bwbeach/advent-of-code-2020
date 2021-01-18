
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

/// Cards are non-negative integers
type Card = usize;

/// The front of the queue is the top of the deck.
type Deck = VecDeque<Card>;

/// The names of the two players
#[derive(Debug, Eq, PartialEq)]
enum Player {
    Player1,
    Player2,
}

use crate::Player::*;

/// The state of a game is the two decks
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

/// Plays a recursive game, returning the state when the game is done
/// 
/// Modifies the game state passed as the game progresses, resulting
/// in the final game state.  Returns the winner
fn play_recursive_game(game: &mut Game) -> Player {
    // all of the states we've seen so far
    let mut history: HashSet<Game> = HashSet::new();

    // play until done
    while ! game.0.is_empty() && ! game.1.is_empty()  {
        // first rule: if we've seen this state before, player 1 wins
        if history.contains(&game) {
            return Player1;
        }
        history.insert(game.clone());

        // draw the top cards
        let card1 = game.0.pop_front().unwrap();
        let card2 = game.1.pop_front().unwrap();

        // determine the winner
        let winner = 
            if card1 <= game.0.len() && card2 <= game.1.len() {
                // there are enough cards to play a recursive game
                let mut inner_game = Game(
                    game.0.iter().take(card1).map(|&c| c).collect(),
                    game.1.iter().take(card2).map(|&c| c).collect()
                );
                play_recursive_game(&mut inner_game)
            } else {
                // not enaugh cards; use original rules
                assert!(card1 != card2);
                if card1 < card2 {
                    Player2
                } else {
                    Player1
                }
            };

        // put the cards in place
        match winner {
            Player1 => { game.0.push_back(card1); game.0.push_back(card2); },
            Player2 => { game.1.push_back(card2); game.1.push_back(card1); },
        }
    }

    game.winner()
}

#[test]
fn test_infinite() {
    let mut game = read_input("infinite.txt");
    assert_eq!(play_recursive_game(&mut game), Player1);
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

fn run_part2(label: &str, file_name: &str) {
    let mut game = read_input(file_name);
    let winner = play_recursive_game(&mut game);
    let winning_deck =
        match winner {
            Player1 => &game.0,
            Player2 => &game.1,
        };
    let score = score_winner(winning_deck);
    println!("{}: {:?}", label, score);
}

fn main() {
    let sample = read_input("sample.txt");
    let finished_sample = play_game(&sample);
    println!("{:?}", finished_sample);
    println!("{:?}", score_winner(finished_sample.winning_deck()));

    let input = read_input("input.txt");
    println!("Part 1: {:?}", score_winner(play_game(&input).winning_deck()));  // 35370

    run_part2("Part 2 sample", "sample.txt");
    run_part2("Part 2", "input.txt");  // 36246
}
