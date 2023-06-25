use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum Focus { Editor, Command }

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Scawir {
    pub term        :   termion::raw::RawTerminal<std::io::Stdout>,
    pub focus       :   Focus,
    pub header      :   String,
    pub editor      :   Vec<String>,
    pub command     :   String,
    pub Editor_dim  :   (usize, usize), // column, Row (width and height)
    pub cursor      :   (usize, usize, usize), // .0 & .1 are editor, .2 is command curs pos

    // flags: 
    RendererHideLineNumbers :   bool,
    RendererHideHeader      :   bool,
    RendererDivChar         :   char,
    RendererHeaderColor     :   termion::color::AnsiValue, 
    RendererLineNumberColor :   termion::color::AnsiValue, 
    RendererDivColor        :   termion::color::AnsiValue, 
    RendererBackgroundColor :   termion::color::AnsiValue, 
}

// ! remove macros
#[allow(non_snake_case)]
#[allow(dead_code)]
impl Scawir {
    
    pub fn init() -> Scawir{
        // default object
        Scawir {
            term                    :   stdout().into_raw_mode().unwrap(),
            focus                   :   Focus::Editor,
            header                  :   String::from(""),
            editor                  :   vec![String::from("")],
            command                 :   String::from(""),
            Editor_dim              :   (0, 0),
            cursor                  :   (0, 0, 0),

            RendererHideLineNumbers :   false,
            RendererHideHeader      :   false,
            RendererDivChar         :   '-',
            RendererHeaderColor     :   termion::color::AnsiValue(83),
            RendererLineNumberColor :   termion::color::AnsiValue(220),
            RendererDivColor        :   termion::color::AnsiValue(245),
            RendererBackgroundColor :   termion::color::AnsiValue(0),
        }
    }
    
    pub fn RendererHideLineNumbers  (mut self, display: bool) -> Self { self.RendererHideLineNumbers = display; self }
    pub fn RendererHideHeader       (mut self, display: bool) -> Self { self.RendererHideHeader      = display; self }
    pub fn RendererDivChar          (mut self, charact: char) -> Self { self.RendererDivChar         = charact; self }
    pub fn RendererHeaderColor      (mut self, color: termion::color::AnsiValue) -> Self { self.RendererHeaderColor      = color; self }
    pub fn RendererLineNumberColor  (mut self, color: termion::color::AnsiValue) -> Self { self.RendererLineNumberColor  = color; self }
    pub fn RendererDivColor         (mut self, color: termion::color::AnsiValue) -> Self { self.RendererDivColor         = color; self }
    pub fn RendererBackgroundColor  (mut self, color: termion::color::AnsiValue) -> Self { self.RendererBackgroundColor  = color; self }
    
    
    fn EditorInput   (&mut self, event: &Event) {
        self.header = "editor".to_string();
        
        print!("{}{}",termion::cursor::Save, termion::cursor::BlinkingBar);
        
        match event {
            Event::Key(Key::Char('\n')) => {
                self.editor.insert(self.cursor.0 + 1, String::new());
                self.cursor.0 += 1;
                self.cursor.1 = 0;
            }
            Event::Key(Key::Char(char)) => {
                self.editor[self.cursor.0].push(*char);
                self.cursor.1 += 1;
            }
            Event::Key(Key::Backspace) => {
                match self.cursor.1 {
                    0 => {
                        if self.editor.len() > 1 {
                            self.editor.remove(self.cursor.0);
                            self.cursor.0 -= 1;
                            self.cursor.1 = self.editor[self.cursor.0].len();
                        }
                    }
                    _ => {
                        if self.cursor.1 == self.editor[self.cursor.0].len() {
                            self.editor[self.cursor.0].pop();
                        } else {
                            self.editor[self.cursor.0].remove(self.cursor.1 - 1);
                        }
                        self.cursor.1 -= 1;
                    }
                }
            },
            _ => {}
        }
    }

    fn CLInput  (&mut self, event: &Event) {
        self.header= "emCLI".to_string();
        
        match event {
            _ => {}
        }
    }

    // pub fn run (&mut self) {
    //     // spawn a new thread for rendering
        
    // }


// ! rendering needs to be done in a seperate thread.
pub fn Run(&mut self) { 
    for event in std::io::stdin().events() {
		match event.as_ref().unwrap() {
			Event::Key(Key::Ctrl('c')) => {
				self.cleanup();
				break;
			},
			Event::Key(Key::Ctrl('x')) => {
				match self.focus {
					Focus::Editor => {
						self.focus = Focus::Command;
					},
					Focus::Command => {
						self.focus = Focus::Editor;
					}
				}
			},
			_ => {
				   match self.focus {
					   Focus::Editor  => self.EditorInput (&event.unwrap()),
					   Focus::Command => self.CLInput(&event.unwrap()),
				   }
			}
		}
        
	}
}

    fn Render (&mut self) {
        print!("\r\x1b[2J\r\x1b[H");
        print!("{}{}{}",
            termion::color::Fg(self.RendererBackgroundColor),
            termion::color::Bg(self.RendererBackgroundColor),
            termion::clear::All
        );
    
        // TODO: make a macro(s) for printing lines more neatly and efficiently
        // * Header
        if !self.RendererHideHeader {
            if self.header.len() > termion::terminal_size().unwrap().0 as usize {
                println!("{}{}{}{} ...{}{}\r",
                    termion::color::Bg(termion::color::AnsiValue(231)),
                    termion::color::Fg(termion::color::AnsiValue(16)),
                    
					termion::style::Bold,
                    &self.header[..termion::terminal_size().unwrap().0 as usize - 4],
                    
					termion::color::Fg(termion::color::Reset),
                    termion::color::Bg(self.RendererBackgroundColor)
                );
            }
            else {
                println!("{}{}{}{}{}{}{}\r",
                    termion::color::Bg(termion::color::AnsiValue(231)), 
                    termion::style::Bold,
                    termion::color::Fg(termion::color::AnsiValue(16)), 
                    &self.header, 
                    termion::color::Fg(termion::color::Reset),
                    // fill line with spaces
                    std::iter::repeat(" ").take(termion::terminal_size().unwrap().0 as usize - self.header.len() ).collect::<String>(),
                    termion::color::Bg(self.RendererBackgroundColor)
                );
            }
        }
    }

    

    fn cleanup (&mut self) {
        print!("{}{}{}",
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset),
            termion::clear::All,
        );
		print!("\r\x1b[2J\r\x1b[H");
		self.term.by_ref().flush().unwrap();
	}
}