pub mod helper_enums;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use self::helper_enums::{Directions, MoveOpt};

pub fn get_player_one(input: Event) -> MoveOpt<Directions> {
    match input {
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::LEFT),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::RIGHT),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::UP),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::DOWN),
        _ => MoveOpt::Same,
    }
}

pub fn get_player_two(input: Event) -> MoveOpt<Directions> {
    match input {
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::LEFT),
        Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::RIGHT),
        Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::UP),
        Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::NONE,
        }) => MoveOpt::Some(Directions::DOWN),
        _ => MoveOpt::Same,
    }
}
