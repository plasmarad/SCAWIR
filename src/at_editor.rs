use std::io::stdout;

use termion::raw::IntoRawMode;
use termion::event::Key;

pub enum Focus { Editor, CL}

pub struct scawir {
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

impl scawir {
    pub fn default() -> scawir {
        scawir {
            term                :   stdout().into_raw_mode().unwrap(),
            focus               :   Focus::Editor,
            header              :   String::from(""),
            editor              :   vec![String::from("")],
            command             :   String::from(""),
            Editor_dim          :   (0, 0),
            cursor              :   (0, 0, 0),

            RendererHideLineNumbers :   false,
            RendererHideHeader      :   false,
            RendererDivChar         :   '-',
            RendererHeaderColor     :   termion::color::AnsiValue(83),
            RendererLineNumberColor :   termion::color::AnsiValue(220),
            RendererDivColor        :   termion::color::AnsiValue(245),
            RendererBackgroundColor :   termion::color::AnsiValue(0),

        }
    }

    pub fn RendererHideLineNumbers  (mut self, display: bool                    )    ->     Self { self.RendererHideLineNumbers  = display; self }
    pub fn RendererHideHeader       (mut self, display: bool                    )    ->     Self { self.RendererHideHeader       = display; self }
    pub fn RendererDivChar          (mut self, charact: char                    )    ->     Self { self.RendererDivChar          = charact; self }
    pub fn RendererHeaderColor      (mut self, color: termion::color::AnsiValue )    ->     Self { self.RendererHeaderColor      = color  ; self }
    pub fn RendererLineNumberColor  (mut self, color: termion::color::AnsiValue )    ->     Self { self.RendererLineNumberColor  = color  ; self }
    pub fn RendererDivColor         (mut self, color: termion::color::AnsiValue )    ->     Self { self.RendererDivColor         = color  ; self }
    pub fn RendererBackgroundColor  (mut self, color: termion::color::AnsiValue )    ->     Self { self.RendererBackgroundColor  = color  ; self }

    

}
