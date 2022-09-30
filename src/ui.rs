use pancurses::{endwin, half_delay, initscr, noecho, raw, Input, Window, curs_set};

pub trait App {
    fn init(&mut self, win: &Window);
    fn update(&mut self, input: Input, win: &Window) -> bool;
    fn render(&self, win: &Window);
}

pub fn run(app: impl App, raw_mode: bool) {
    let mut app = app;

    let window = initscr();
    if raw_mode {
        raw();
    }
    curs_set(0);
    half_delay(2);
    noecho();
    window.nodelay(true);
    window.keypad(true);

    app.init(&window);

    loop {
        app.render(&window);

        match window.getch() {
            Some(input) => {
                if !app.update(input, &window) {
                    break;
                }
            }
            None => (),
        }
    }

    endwin();
}
