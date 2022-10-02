mod ui;
mod types;
mod constants;

use constants::{BOARD_POSITION_Y, BOARD_POSITION_X, MAX_INPUT_BUFFER_SIZE, THEME_BOARD_HINT, THEME_BOARD_CELL_WHITE_WHITE_PIECE, THEME_BOARD_CELL_WHITE_BLACK_PIECE, THEME_BOARD_CELL_BLACK_WHITE_PIECE, THEME_BOARD_CELL_BLACK_BLACK_PIECE, THEME_BOARD_TEXT_BLACK, THEME_BOARD_TEXT_WHITE};
use owlchess::{board::Board, Coord, Color, Piece, Move};
use pancurses::{Input, Window, init_pair, COLOR_PAIR, COLOR_WHITE, COLOR_BLACK, COLOR_YELLOW, init_color, COLOR_GREEN};
use types::{BoardColor, Player};
use ui::{run, App};

pub struct LichessApp {
    input_buffer: String,
    input_win: Option<Window>,
    board: Board
}

impl LichessApp {
    fn new() -> Self {
        Self {
            input_buffer: String::new(),
            input_win: None,
            board: Board::initial()
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
        // Board hint
        win.attrset(COLOR_PAIR(THEME_BOARD_HINT));
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

        // Pieces
        for ry in 0..8 {
            for x in 0..8 {
                let is_white_cell = if ry % 2 == 0 {
                    x % 2 == 0
                } else {
                    x % 2 != 0
                };

                let mut y = ry;
                if player_side == BoardColor::Black {
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
                    Some(Piece::Queen) => '♛'
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

                win.mvprintw(
                    ry + BOARD_POSITION_Y,
                    x * 2 + BOARD_POSITION_X,
                    cell_str
                );
            }
        }
    }

    fn draw_player_info(&self, win: &Window, player_w: &Player, player_b: &Player) {
        let title_w = player_w.title.as_deref().unwrap_or("");
        let title_b = player_b.title.as_deref().unwrap_or("");

        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_WHITE));
        win.mv(BOARD_POSITION_Y, BOARD_POSITION_X + 18);
        win.printw(format!("● {} {} ({})", &title_w, player_w.name, player_w.rate));

        win.attrset(COLOR_PAIR(THEME_BOARD_TEXT_BLACK));
        win.mv(BOARD_POSITION_Y + 7, BOARD_POSITION_X + 18);
        win.printw(format!("● {} {} ({})", &title_b, player_b.name, player_b.rate));
    }
}

impl App for LichessApp {
    fn init(&mut self, win: &Window) {
        init_color(COLOR_BLACK, 70, 74, 94);
        init_color(COLOR_WHITE, 1000, 1000, 1000);
        init_color(COLOR_YELLOW, 509, 545, 721);
        init_color(COLOR_GREEN, 258, 278, 368);

        init_pair(THEME_BOARD_HINT as i16, COLOR_GREEN, -1);
        init_pair(THEME_BOARD_TEXT_WHITE as i16, COLOR_WHITE, -1);
        init_pair(THEME_BOARD_TEXT_BLACK as i16, COLOR_BLACK, -1);

        init_pair(THEME_BOARD_CELL_WHITE_WHITE_PIECE as i16, COLOR_WHITE, COLOR_YELLOW);
        init_pair(THEME_BOARD_CELL_WHITE_BLACK_PIECE as i16, COLOR_BLACK, COLOR_YELLOW);

        init_pair(THEME_BOARD_CELL_BLACK_WHITE_PIECE as i16, COLOR_WHITE, COLOR_GREEN);
        init_pair(THEME_BOARD_CELL_BLACK_BLACK_PIECE as i16, COLOR_BLACK, COLOR_GREEN);

        self.input_win = win.subwin(3, 20, 12, 0).ok();
    }

    fn update(&mut self, input: Input, _win: &Window) -> bool {
        match input {
            // Enter
            Input::Character('\n') => {
                if let Ok(parsed_move) = Move::from_san(&self.input_buffer, &self.board) {
                    if let Ok(new_board) = self.board.make_move(parsed_move) {
                        self.board = new_board;
                    }
                }
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
        self.draw_board(win, &self.board, BoardColor::White);
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
