mod constants;
mod types;
mod ui;

use std::time::{Instant, Duration};

use constants::{
    BOARD_POSITION_X, BOARD_POSITION_Y, MAX_INPUT_BUFFER_SIZE, THEME_BOARD_CELL_BLACK_BLACK_PIECE,
    THEME_BOARD_CELL_BLACK_WHITE_PIECE, THEME_BOARD_CELL_WHITE_BLACK_PIECE,
    THEME_BOARD_CELL_WHITE_WHITE_PIECE, THEME_BOARD_HINT, THEME_BOARD_TEXT_BLACK,
    THEME_BOARD_TEXT_WHITE,
};
use owlchess::{board::Board, Color, Coord, Move, Piece};
use pancurses::{
    init_color, init_pair, Input, Window, COLOR_BLACK, COLOR_GREEN, COLOR_PAIR, COLOR_WHITE,
    COLOR_YELLOW, A_STANDOUT, A_NORMAL,
};
use types::{BoardColor, Player};
use ui::{run, App};

pub struct LichessApp {
    input_buffer: String,
    input_win: Option<Window>,
    player_side: BoardColor,
    board: Board,
    board_message: String,
    players: (Player, Player),
    last_tick: Instant
}

impl LichessApp {
    fn new() -> Self {
        Self {
            input_buffer: String::new(),
            input_win: None,
            player_side: BoardColor::White,
            board: Board::initial(),
            board_message: String::new(),
            players: (
                Player::new("huy", 2400, ""),
                Player::new("huygm", 3000, "GM"),
            ),
            last_tick: Instant::now()
        }
    }

    fn draw_board_message(&self, win: &Window) {
        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_WHITE));
        win.mv(BOARD_POSITION_Y + 9, BOARD_POSITION_X);
        win.clrtoeol();
        win.mvprintw(BOARD_POSITION_Y + 9, BOARD_POSITION_X, &self.board_message);
    }

    fn draw_input_box(&self) {
        if let Some(input_win) = &self.input_win {
            input_win.clear();
            input_win.draw_box(0, 0);
            input_win.mvprintw(1, 2, &format!("Your move: {}█", self.input_buffer));
            input_win.refresh();
        }
    }

    fn draw_board(&self, win: &Window) {
        let player_side = &self.player_side;
        let board = &self.board;

        // Board hint
        win.attrset(COLOR_PAIR(THEME_BOARD_HINT));
        if *player_side == BoardColor::White {
            win.mvprintw(BOARD_POSITION_Y + 8, BOARD_POSITION_X, "a b c d e f g h");
        } else {
            win.mvprintw(BOARD_POSITION_Y + 8, BOARD_POSITION_X, "h g f e d c b a");
        }
        for i in 0..8 {
            let rank = if *player_side == BoardColor::White {
                8 - i
            } else {
                i + 1
            };
            win.mvprintw(
                i + BOARD_POSITION_Y,
                BOARD_POSITION_X - 2,
                format!("{}", rank),
            );
        }

        // Pieces
        for ry in 0..8 {
            for x in 0..8 {
                let is_white_cell = if ry % 2 == 0 { x % 2 == 0 } else { x % 2 != 0 };

                let mut y = ry;
                if *player_side == BoardColor::Black {
                    y = 8 - ry - 1;
                }
                let i = (y * 8 + x) as usize;

                let cell = board.get(Coord::from_index(i));
                let piece_str = match cell.piece() {
                    None => ' ',
                    Some(Piece::Pawn) => '♟',
                    Some(Piece::King) => '♚',
                    Some(Piece::Knight) => '♞',
                    Some(Piece::Bishop) => '♝',
                    Some(Piece::Rook) => '♜',
                    Some(Piece::Queen) => '♛',
                };
                let cell_str = format!("{} ", piece_str);

                let piece_color = cell.color().unwrap_or(Color::White);

                if is_white_cell {
                    if piece_color == Color::White {
                        win.attrset(COLOR_PAIR(THEME_BOARD_CELL_WHITE_WHITE_PIECE));
                    } else {
                        win.attrset(COLOR_PAIR(THEME_BOARD_CELL_WHITE_BLACK_PIECE));
                    }
                } else {
                    if piece_color == Color::White {
                        win.attrset(COLOR_PAIR(THEME_BOARD_CELL_BLACK_WHITE_PIECE));
                    } else {
                        win.attrset(COLOR_PAIR(THEME_BOARD_CELL_BLACK_BLACK_PIECE));
                    }
                };

                win.mvprintw(ry + BOARD_POSITION_Y, x * 2 + BOARD_POSITION_X, cell_str);
            }
        }
    }

    fn draw_player_info(&self, win: &Window) {
        let (player_w, player_b) = &self.players;

        let title_w = player_w.title.as_deref().unwrap_or("");
        let title_b = player_b.title.as_deref().unwrap_or("");

        // Player 1
        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_WHITE));
        win.mv(BOARD_POSITION_Y, BOARD_POSITION_X + 18);
        win.printw("● ");
        if title_w.len() > 0 {
            win.printw(format!("{} ", &title_w));
        }
        win.printw(format!("{} ", player_w.name));
        win.attrset(COLOR_PAIR(THEME_BOARD_HINT));
        win.printw(format!("({})", player_w.rate));

        // Player 2
        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_BLACK));
        win.mv(BOARD_POSITION_Y + 7, BOARD_POSITION_X + 18);
        win.printw("● ");
        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_WHITE));
        if title_b.len() > 0 {
            win.printw(format!("{} ", &title_b));
        }
        win.printw(format!("{} ", player_b.name));
        win.attrset(COLOR_PAIR(THEME_BOARD_HINT));
        win.printw(format!("({})", player_b.rate));
    }

    fn draw_player_clock(&self, win: &Window, clock: u64, color: Color) {
        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_WHITE));

        if self.board.side() == color {
            win.attrset(A_STANDOUT);
        } else {
            win.attrset(A_NORMAL);
        }

        let min = clock / 60;
        let sec = clock % 60;

        win.printw(format!("{:02}:{:02}", min, sec));
    }

    fn draw_clock(&self, win: &Window) {
        let (player_w, player_b) = &self.players;
        let player_side = &self.player_side;

        if *player_side == BoardColor::White {
            win.mv(BOARD_POSITION_Y + 1, BOARD_POSITION_X + 18);
            self.draw_player_clock(win, player_b.clock, Color::Black);
            win.mv(BOARD_POSITION_Y + 8, BOARD_POSITION_X + 18);
            self.draw_player_clock(win, player_w.clock, Color::White);
        } else {
            win.mv(BOARD_POSITION_Y + 1, BOARD_POSITION_X + 18);
            self.draw_player_clock(win, player_w.clock, Color::White);
            win.mv(BOARD_POSITION_Y + 8, BOARD_POSITION_X + 18);
            self.draw_player_clock(win, player_b.clock, Color::Black);
        }
    }
}

impl App for LichessApp {
    fn update(&mut self, _win: &Window) {
        let now = Instant::now();
        if now.duration_since(self.last_tick) >= Duration::from_secs(1) {
            if self.board.side() == Color::White {
                if self.players.0.clock > 0 {
                    self.players.0.clock -= 1;
                }
            } else {
                if self.players.1.clock > 0 {
                    self.players.1.clock -= 1;
                }
            }
            self.last_tick = now;
        }
    }

    fn init(&mut self, win: &Window) {
        init_color(COLOR_BLACK, 70, 74, 94);
        init_color(COLOR_WHITE, 1000, 1000, 1000);
        init_color(COLOR_YELLOW, 509, 545, 721);
        init_color(COLOR_GREEN, 258, 278, 368);

        init_pair(THEME_BOARD_HINT as i16, COLOR_GREEN, -1);
        init_pair(THEME_BOARD_TEXT_WHITE as i16, COLOR_WHITE, -1);
        init_pair(THEME_BOARD_TEXT_BLACK as i16, COLOR_BLACK, -1);

        init_pair(
            THEME_BOARD_CELL_WHITE_WHITE_PIECE as i16,
            COLOR_WHITE,
            COLOR_YELLOW,
        );
        init_pair(
            THEME_BOARD_CELL_WHITE_BLACK_PIECE as i16,
            COLOR_BLACK,
            COLOR_YELLOW,
        );

        init_pair(
            THEME_BOARD_CELL_BLACK_WHITE_PIECE as i16,
            COLOR_WHITE,
            COLOR_GREEN,
        );
        init_pair(
            THEME_BOARD_CELL_BLACK_BLACK_PIECE as i16,
            COLOR_BLACK,
            COLOR_GREEN,
        );

        self.input_win = win.subwin(3, 20, 12, 0).ok();
    }

    fn input(&mut self, input: Input, _win: &Window) -> bool {
        match input {
            // Enter
            Input::Character('\n') => {
                self.board_message.clear();
                if let Ok(parsed_move) = Move::from_san(&self.input_buffer, &self.board) {
                    if let Ok(new_board) = self.board.make_move(parsed_move) {
                        self.board = new_board;
                    }
                } else {
                    self.board_message = format!("{} is not a valid move!", self.input_buffer);
                }
                self.input_buffer.clear();
            }
            // Backspace
            Input::KeyBackspace => {
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
        self.draw_board(win);
        self.draw_board_message(win);
        self.draw_input_box();
        self.draw_player_info(win);
        self.draw_clock(win);
    }
}

fn main() {
    let app = LichessApp::new();
    run(app, false);
}
