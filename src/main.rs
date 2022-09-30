mod ui;
mod types;
mod constants;

use constants::{BOARD_POSITION_Y, BOARD_POSITION_X, MAX_INPUT_BUFFER_SIZE};
use owlchess::{board::Board, Coord};
use pancurses::{Input, Window};
use types::{BoardColor, Player};
use ui::{run, App};

pub struct LichessApp {
    input_buffer: String,
    input_win: Option<Window>,
}

impl LichessApp {
    fn new() -> Self {
        Self {
            input_buffer: String::new(),
            input_win: None,
        }
    }

    fn draw_input_box(&self) {
        if let Some(input_win) = &self.input_win {
            input_win.clear();
            input_win.draw_box(0, 0);
            input_win.mvprintw(1, 2, &format!("Your move: {}█", self.input_buffer));
            input_win.refresh();
        }
    }

    fn draw_board(&self, win: &Window, board: &Board, player_side: BoardColor) {
        if player_side == BoardColor::White {
            win.mvprintw(BOARD_POSITION_Y + 8, BOARD_POSITION_X, "a b c d e f g h");
        } else {
            win.mvprintw(BOARD_POSITION_Y + 8, BOARD_POSITION_X, "h g f e d c b a");
        }

        for i in 0..8 {
            let rank = if player_side == BoardColor::White {
                8 - i
            } else {
                i + 1
            };
            win.mvprintw(i + BOARD_POSITION_Y, BOARD_POSITION_X - 2, format!("{}", rank));
        }

        for ry in 0..8 {
            for x in 0..8 {
                let mut y = ry;
                if player_side == BoardColor::Black {
                    y = 8 - ry - 1;
                }
                let i = (y * 8 + x) as usize;
                win.mvprintw(
                    ry + BOARD_POSITION_Y,
                    x * 2 + BOARD_POSITION_X,
                    board.get(Coord::from_index(i)).as_utf8_char().to_string(),
                );
            }
        }
    }

    fn draw_player_info(&self, win: &Window, player_w: &Player, player_b: &Player) {
        win.mv(BOARD_POSITION_Y, BOARD_POSITION_X + 16);
        win.vline('|', 9);

        let title_w = player_w.title.as_ref().map(|x| x.as_str()).unwrap_or("");
        let title_b = player_b.title.as_ref().map(|x| x.as_str()).unwrap_or("");

        win.mv(BOARD_POSITION_Y, BOARD_POSITION_X + 18);
        win.printw(format!("● {} {} ({})", &title_w, player_w.name, player_w.rate));

        win.mv(BOARD_POSITION_Y + 8, BOARD_POSITION_X + 18);
        win.printw(format!("● {} {} ({})", &title_b, player_b.name, player_b.rate));
    }
}

impl App for LichessApp {
    fn init(&mut self, win: &Window) {
        self.input_win = win.subwin(3, 20, 12, 0).ok();
    }

    fn update(&mut self, input: Input, _win: &Window) -> bool {
        match input {
            // Enter
            Input::Character('\n') => {
                self.input_buffer.clear();
            }
            // Backspace
            Input::Character('\x7f') => {
                self.input_buffer.pop();
            }
            Input::Character(c) => {
                if c.is_alphanumeric() && self.input_buffer.len() < MAX_INPUT_BUFFER_SIZE {
                    self.input_buffer.push(c);
                }
            }
            _ => {}
        }
        return true;
    }

    fn render(&self, win: &Window) {
        self.draw_board(win, &Board::initial(), BoardColor::Black);
        self.draw_input_box();
        self.draw_player_info(win,
            &Player::new("huy", 2000, ""),
            &Player::new("gmhuy", 3000, "GM"),
        );
    }
}

fn main() {
    let app = LichessApp::new();
    run(app, false);
}