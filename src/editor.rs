use std::io::{stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum Focus { Editor, Command }

// since this is a remake...
// ! perhaps switch to the termion crate?

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
    // ! replace manually storing the cursor pos with using relative cursor pos (cursor up, cursor down, etc), and use get_cursor_pos() to get the absolute cursor pos

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
    
    
    fn EditorIOHandle   (&mut self, event: &Event) {
        self.header="editor".to_string();
        print!("{}{}",termion::cursor::Save, termion::cursor::BlinkingBar);
        match event {
            Event::Key(Key::Char('\n')) => {
                self.editor.insert(self.cursor.0 + 1, String::from(""));
                self.cursor.0 += 1;
                self.cursor.1 = 0;
            },
            Event::Key(Key::Char(char)) => {
                self.editor[self.cursor.0].insert(self.cursor.1, *char);
                self.cursor.1 += 1;
            },
            Event::Key(Key::Backspace) => {
                if self.editor[self.cursor.0].len() == 0 && self.editor.len() > 1 {
                    self.editor.remove(self.cursor.0); // remove the line
                    self.cursor.0 -= 1;                         // we removed a line, so we need to go back 1 line
                    self.cursor.1 = self.editor[self.cursor.0].len(); // set the character index to the last character of prev line
                } 
                else if self.cursor.1 > 0 { 
                    if self.cursor.1 == self.editor[self.cursor.0].len() 
                        {self.editor[self.cursor.0].pop();} // if the cursor is at the end of the line, pop the last character
                    else {self.editor[self.cursor.0].remove(self.cursor.1);}
                    self.cursor.1 -= 1;
                }
            },
            _ => {}
        }
    }

    fn CommandIOHandle  (&mut self, event: &Event) {
        self.header= "emCLI".to_string();
        match event {
            
            _ => {}
        }
    }
    pub fn Run(&mut self) { 
        for event in std::io::stdin().events() {
                match event.as_ref().unwrap() {
                    Event::Key(Key::Ctrl('c')) => {
                        self.cleanup(); 
                        print !("\r\x1b[2J\r\x1b[H");
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
                            Focus::Editor  => self.EditorIOHandle (&event.unwrap()),
                            Focus::Command => self.CommandIOHandle(&event.unwrap()),
                        }

                    }
                }
                self.Render(); self.term.by_ref().flush().unwrap();
            }
    }
    /*
    temporary render function
    future render will have a "scene" rendering system
    and easier prototyping
    */
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
        
        // self.header += "a";
        
        // self.Editor_dim = (termion::terminal_size().unwrap().0 as usize, termion::terminal_size().unwrap().1 as usize - 5);
        
        // find the dimensions of the terminal first
        // explain the \r\x1b[2J\r\x1b[H
        // \r = carriage return
        // \x1b[2J = clear screen
        // \x1b[H = move cursor to top left

        
        // reserve 4 lines for Header, Div (2x), Command (off set -1 from size)
        // set background color
        
        // There's 6 objects
        // Header, Div, Editor+LineNumbers, div, Command
        // if !self.RendererHideHeader {
        //     if self.header.len() > termion::terminal_size().unwrap().0 as usize {
        //         print!("{}{}{}",
        //             termion::color::Fg(self.RendererHeaderColor),
        //             &self.header[..termion::terminal_size().unwrap().0 as usize],
        //             termion::color::Fg(termion::color::Reset),
        //         );
        //     } 
        //     else {
        //         print!("{}{}{}\n",
        //             termion::color::Fg(self.RendererHeaderColor),
        //             &self.header,
        //             termion::color::Fg(termion::color::Reset),
        //         );
        //     }
        // }
        // println!("{}\r", self.RendererDivChar.to_string().repeat(termion::terminal_size().unwrap().0 as usize - 1));
        // if !self.RendererHideLineNumbers {
        //     for (line_num, line_content) in self.editor[start_line..end].iter().enumerate() {
        //         print!("{}{}{}\r\n", format!("{:0>1}\t| ", line_num+1), line_content, termion::cursor::Goto(self.cursor.1 as u16 + 11, self.cursor.0 as u16 + 3));
        //     }
        // }
    }

    fn cleanup (&mut self) {
        print!("{}{}{}",
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset),
            termion::clear::All,
        );
    }
}
