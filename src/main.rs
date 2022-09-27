use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

enum ColorPk {
    Red,
    Green,
    Blue,
    E,
}

enum Cl {
    Green(termion::color::Green),
    Red(termion::color::Red),
    Blue(termion::color::Blue),
    E(termion::color::Reset),
}

struct ColorP {
    ay: ColorPk,
}

impl ColorP {
    fn init(&mut self) -> ColorP {
        self.ay = ColorPk::Red;

        return ColorP { ay: ColorPk::Red };
    }
    fn get(&self) {
        match self.ay {
            ColorPk::Blue => color::Rgb(255, 0, 0),
            ColorPk::Red => color::Rgb(0, 0, 255),
            ColorPk::Green => color::Rgb(0, 255, 0),
            ColorPk::E => color::Rgb(255, 255, 255),
        };
    }
    fn set(&mut self, cl: ColorPk) {
        self.ay = cl;
    }
}

fn main() {
    let mut clsys: ColorPk = ColorPk::Red;
    let mut e: bool = false;
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let size = termion::terminal_size().unwrap();

    write!(
        stdout,
        "{}{}{}q to exit. Terminal Size x: {}, y: {}{} {}Pick Color:{} {}  {} Red(r)  {} {}  {}   {}Blue(t){}   {}  {}   {}Green(y){}   {}  {}  {} White(e){}   {}  {}  {} eraser(u)  {} {} {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::color::Bg(termion::color::LightBlack),
        size.0,
        size.1,
        termion::color::Bg(termion::color::Reset),
        termion::style::Bold,
        termion::style::Reset,
        termion::color::Bg(termion::color::Red),
        termion::color::Fg(termion::color::Black),
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset),
        termion::color::Bg(termion::color::Blue),
        termion::color::Fg(termion::color::Black),
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset),
        termion::color::Bg(termion::color::Green),
        termion::color::Fg(termion::color::Black),
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset),
        termion::color::Bg(termion::color::White),
        termion::color::Fg(termion::color::Black),
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset),
        termion::color::Bg(termion::color::LightMagenta),
        termion::color::Fg(termion::color::Black),
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                write!(
                    stdout,
                    "{} {} {}",
                    termion::cursor::Show,
                    termion::clear::All,
                    "\x1B[2J\x1B[1;1H"
                );
                break;
            }

            Event::Key(Key::Char('e')) => {
                e = false;
                clsys = ColorPk::E
            }
            Event::Key(Key::Char('r')) => {
                e = false;
                clsys = ColorPk::Red
            }
            Event::Key(Key::Char('t')) => {
                e = false;
                clsys = ColorPk::Blue
            }
            Event::Key(Key::Char('y')) => {
                e = false;
                clsys = ColorPk::Green
            }
            Event::Key(Key::Char('u')) => e = true,
            Event::Mouse(me) => match me {
                MouseEvent::Hold(x, y) | MouseEvent::Press(_, x, y) => {
                    if y <= 3 {
                        continue;
                    } else {
                        if !e {
                            write!(
                                stdout,
                                "{}{}{} {}",
                                termion::cursor::Goto(x, y),
                                termion::style::Bold,
                                termion::color::Bg(match clsys {
                                    ColorPk::Red => color::Rgb(255, 0, 0),
                                    ColorPk::Blue => color::Rgb(0, 0, 255),
                                    ColorPk::Green => color::Rgb(0, 255, 0),
                                    ColorPk::E => color::Rgb(255, 255, 255),
                                }),
                                termion::style::Reset
                            )
                            .unwrap();
                        } else {
                            write!(
                                stdout,
                                "{}{}{} {}",
                                termion::cursor::Goto(x, y),
                                termion::style::Bold,
                                termion::color::Bg(termion::color::Reset),
                                termion::style::Reset
                            )
                            .unwrap();
                        }
                    }
                }
                _ => (),
            },
            x => {}
        }
        stdout.flush().unwrap();
    }
}
