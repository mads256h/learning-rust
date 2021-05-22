use ncurses::*;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Vector {
        return Vector { x, y };
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self::new(self.x + other.x, self.y + other.y);
    }
}

struct Player {
    positions: Vec<Vector>,
    win: WINDOW,
}

impl Player {
    fn full_render(&self) {
        werase(self.win);

        for pos in self.positions.iter() {
            wmove(self.win, pos.y, pos.x);
            addstr("o");
        }
    }

    fn move_to(&mut self, position: Vector) {
        let pos_len = self.positions.len();
        let last_pos = self.positions[pos_len - 1];
        let bounds = get_bounds(self.win);

        if position.x < 0 || position.y < 0 || position.x >= bounds.x || position.y >= bounds.y {
            return;
        }

        wmove(self.win, last_pos.y, last_pos.x);
        addstr(" ");

        for i in (1..pos_len).rev() {
            self.positions[i] = self.positions[i - 1];
            let pos = self.positions[i];
            wmove(self.win, pos.y, pos.x);
            addstr("o");
        }

        self.positions[0] = position;
        wmove(self.win, position.y, position.x);
        addstr("o");
    }

    fn move_up(&mut self) {
        let h = self.head();
        const UP: Vector = Vector { x: 0, y: -1 };

        self.move_to(h + UP);
    }

    fn move_down(&mut self) {
        let h = self.head();
        const DOWN: Vector = Vector {x:0, y: 1};

        self.move_to(h + DOWN);
    }

    fn move_right(&mut self) {
        let h = self.head();
        const RIGHT: Vector = Vector {x: 1, y: 0};

        self.move_to(h + RIGHT);
    }

    fn move_left(&mut self) {
        let h = self.head();
        const LEFT: Vector = Vector {x: -1, y: 0};

        self.move_to(h + LEFT);
    }

    fn head(&self) -> Vector {
        return self.positions[0];
    }
}

fn get_bounds(win: WINDOW) -> Vector {
    let mut x = 0;
    let mut y = 0;

    getmaxyx(win, &mut y, &mut x);

    return Vector::new(x, y);
}

fn main() {
    let win = initscr();
    let mut ply = Player {
        positions: vec![
            Vector::new(6, 0),
            Vector::new(5, 0),
            Vector::new(4, 0),
            Vector::new(3, 0),
            Vector::new(2, 0),
            Vector::new(1, 0),
            Vector::new(0, 0),
        ],
        win,
    };

    noecho();

    keypad(win, true);
    wtimeout(win, 16);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    ply.full_render();

    loop {
        let ch = wgetch(win);

        match ch {
            KEY_UP => {
                ply.move_up();
            }
            KEY_DOWN => {
                ply.move_down();
            }
            KEY_RIGHT => {
                ply.move_right();
            }
            KEY_LEFT => {
                ply.move_left();
            }
            113 => {
                break;
            }
            _ => {
                //                addstr(&format!("char {}\n", ch));
            }
        }
    }

    endwin();
}
