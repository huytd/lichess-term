use pancurses::{
    curs_set, endwin, half_delay, has_colors, initscr, noecho, raw, start_color,
    use_default_colors, Input, Window,
};

pub trait App {
    fn init(&mut self, win: &Window);
    fn update(&mut self, win: &Window);
    fn input(&mut self, input: Input, win: &Window) -> bool;
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

    if has_colors() {
        use_default_colors();
        start_color();
    }

    app.init(&window);

    loop {
        app.update(&window);
        app.render(&window);
        match window.getch() {
            Some(input) => {
                if !app.input(input, &window) {
                    break;
                }
            }
            None => (),
        }
    }

    endwin();
}
