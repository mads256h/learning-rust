use ncurses::*;

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

        let up = Vector::new(h.x, h.y - 1);

        self.move_to(up);
    }

    fn move_down(&mut self) {
        let h = self.head();

        let down = Vector::new(h.x, h.y + 1);

        self.move_to(down);
    }

    fn move_right(&mut self) {
        let h = self.head();

        let right = Vector::new(h.x + 1, h.y);

        self.move_to(right);
    }

    fn move_left(&mut self) {
        let h = self.head();

        let left = Vector::new(h.x - 1, h.y);

        self.move_to(left);
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
        positions: vec![Vector::new(6, 0), Vector::new(5, 0), Vector::new(4, 0), Vector::new(3, 0), Vector::new(2, 0), Vector::new(1, 0), Vector::new(0, 0)],
        win,
    };

    noecho();

    keypad(win, true);
    wtimeout(win, 16);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let bounds = get_bounds(win);

    ply.full_render();

    loop {
        let ch = wgetch(win);
        let some = Some(ch);

        match some {
            Some(KEY_UP) => {
                ply.move_up();
            }
            Some(KEY_DOWN) => {
                ply.move_down();
            }
            Some(KEY_RIGHT) => {
                ply.move_right();
            }
            Some(KEY_LEFT) => {
                ply.move_left();
            }
            Some(113) => {
                break;
            }
            _ => {
//                addstr(&format!("char {}\n", ch));
            }
        }
    }

    endwin();
}
