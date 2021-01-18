
use std::collections::VecDeque;
use std::fs::read_to_string;

type Card = usize;

/// The front of the queue is the top of the deck.
type Deck = VecDeque<Card>;

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
fn read_input(file_name: &str) -> (Deck, Deck) {
    let text = read_to_string(file_name).unwrap();
    let parts: Vec<_> = text.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    (parse_deck(parts[0]), parse_deck(parts[1]))
}

/// Plays a game, returning the winning deck.
fn play_game(init_a: &Deck, init_b: &Deck) -> Deck {
    let mut a: Deck = init_a.clone();
    let mut b: Deck = init_b.clone();

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

    if a.is_empty() {
        b
    } else {
        a
    }
}

fn score_winner(deck: &Deck) -> usize {
    let mut copy = deck.clone();
    let mut result = 0;
    let mut counter = 1;
    while let Some(card) = copy.pop_back() {
        result += counter * card;
        counter += 1
    }
    result
}

fn main() {
    let sample = read_input("sample.txt");
    let winning_deck = play_game(&sample.0, &sample.1);
    println!("{:?}", winning_deck);
    println!("{:?}", score_winner(&winning_deck));

    let input = read_input("input.txt");
    println!("Part 1: {:?}", score_winner(&play_game(&input.0, &input.1)));  // 35370

}
