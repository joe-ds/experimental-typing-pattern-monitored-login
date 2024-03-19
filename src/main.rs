extern crate ncurses;
extern crate chrono;

use std::time::Duration as Std_Duration;
use std::thread::sleep;
use ncurses::*;
use std::char;
use chrono::{Duration, Local};
use std::env;

const CMP: usize = 500;

fn parse_args() -> usize {
    match env::args().nth(1) {
        None => CMP,
        Some(a) => match a.trim().parse() {
            Err(_) => CMP,
            Ok(n) => n,
        },
    }
}

fn get_pass(durs: &mut Vec<Duration>) -> String {
    let mut pass = String::new();
    let mut d_last = Local::now();
    let mut first = true;

    loop {
        let ch = char::from_u32(getch() as u32)
            .expect("Invalid char!");

        if first {
            d_last = Local::now();
            durs.push(Duration::zero());
            first = false;
        }
        else {
            let d_now = Local::now();
            durs.push(d_now.signed_duration_since(d_last));
            d_last = d_now.clone();
        };
        
        if ch == '\n' {
            break; }
        else {
            pass.push(ch);
        }
    }
    pass
}

fn init() -> () {
    let mut iteration = 0;
    
    while iteration < 3 {
        if COLS() < 80 || LINES() < 24 {
            iteration += 1;
            wmove(stdscr(), 0, 0);
            printw("Best viewed with a window of 80x24.");
            refresh();
            sleep(Std_Duration::from_secs(2));
        }
        else {
            clear();
            refresh();
            break;
        }
    };
    
    wmove(stdscr(), 0, 0);
    logo();
    box_(stdscr(), 0, 0);

    let mut y = 0;
    let mut x = 0;

    getyx(stdscr(), &mut y, &mut x);
    wmove(stdscr(), y+3, x);
}

fn logo() -> () {
let r = r#"
                    |>>>                        |>>>
                    |                           |
                _  _|_  _                   _  _|_  _
               | |_| |_| |                 | |_| |_| |
               \  .      /                 \ .    .  /
                \    ,  /                   \    .  /
                 | .   |_   _   _   _   _   _| ,   |
                 |    .| |_| |_| |_| |_| |_| |  .  |
                 | ,   | .    .     .      . |    .|
                 |   . |  .     . .   .  ,   |.    |
     ___----_____| .   |.   ,  _______   .   |   , |---~_____
_---~            |     |  .   /+++++++\    . | .   |         ~---_
                 |.    | .    |+++++++| .    |   . |              ~-_
              __ |   . |   ,  |+++++++|.  . _|__   |                 ~-_
     ____--`~    '--~~__ .    |++++ __|----~    ~`---,              ___^~-__
-~--~                   ~---__|,--~'                  ~~----_____-~'   `~----~
ART BY T.L.G."#;
    printw(&r);
}

fn clear_line() -> () {
    let mut y = 0;
    let mut x = 0;
    getyx(stdscr(), &mut y, &mut x);

    wmove(stdscr(), y, 0);
    clrtoeol();
    box_(stdscr(), 0, 0);
}

fn success() -> () {
    let mut y = 0;
    let mut x = 0;
    
    wmove(stdscr(), 15, 31);
    sleep(Std_Duration::from_millis(500));
    for _ in 0..4 {
        addch(0x20);
    };
    getyx(stdscr(), &mut y, &mut x);

    y -= 1; x -= 4;
    for _ in 0..3 {
        wmove(stdscr(), y, x);
        refresh();
        sleep(Std_Duration::from_millis(500));
        for _ in 0..7 {
            addch(0x20);
        };
        y -= 1;
    };
}

fn prompt(s: &str, d: &mut Vec<Duration>) -> String {
    let mut y = 0;
    let mut x = 0;
    getyx(stdscr(), &mut y, &mut x);
    
    wmove(stdscr(), y,  40 - s.len() as i32);
    
    attron(A_BOLD());
    printw(s);
    attroff(A_BOLD());

    attron(A_BLINK());
    printw("_");
    attroff(A_BLINK());

    get_pass(d)
}

fn cmp_durs(d1: &Vec<Duration>, d2: &Vec<Duration>) -> bool {
    let mut s = 0;
    for (x, y) in d1.iter().zip(d2.iter()) {
        s += (x.num_milliseconds() - y.num_milliseconds()).abs();
    }
    (s / d1.len() as i64) < (parse_args() as i64)
}

fn main()
{
    initscr();
    raw();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    box_(stdscr(), 0, 0);

    init();

    let mut durs_one = Vec::new();
    let mut durs_two = Vec::new();
    
    let pass_one = prompt("Enter your password: ", &mut durs_one);
    clear_line();
    let pass_two = prompt("And again: ", &mut durs_two);
    clear_line();

    let mut y = 0;
    let mut x = 0;
    getyx(stdscr(), &mut y, &mut x);
    
    if pass_one == pass_two && cmp_durs(&durs_one, &durs_two) {
        wmove(stdscr(), y,  32);
        attron(A_BLINK() | A_BOLD() | A_STANDOUT());
        printw("Success!");
        attroff(A_BLINK() | A_BOLD() | A_STANDOUT());
        success();
    } else {
        wmove(stdscr(), y,  20);
        printw("Failure... (Press any key to exit)");
    };

    refresh();
    getch();
    endwin();
}
