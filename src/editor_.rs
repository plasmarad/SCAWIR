use std::{io::Write};

use console::{self, Key, style, };
#[allow(dead_code)]
enum Focus {Editor, CommandLine}

// #[repr(u8)]
// #[derive(Copy, Clone, Debug, PartialEq)]
// enum Flags {
//     RendererHideLineNumbers (bool)          ,
//     RendererHideEditor      (bool)          ,
//     RendererHideHeader      (bool)          ,
//     RendererDivChar         (char)          ,
//     RendererHeaderColor     (console::Color),
//     RendererLineNumberColor (console::Color),
//     RendererDivColor        (console::Color),
//     RendererBackgroundColor (console::Color),
// }

#[allow(dead_code)]
pub struct Editor {
    term        :   console::Term,
    focus       :   Focus,
    header      :   String,
    editor      :   Vec<String>,
    command     :   String,
    editor_dim  :   (usize, usize), // (width and height)
    cursor      :   (usize, usize, usize), // idx 0 & 1 are for editor cursor, idx 2 is for command line cursor
}

impl Editor{
    #[allow(dead_code)]
    pub fn switch_focus(&mut self){
        match self.focus {
            Focus::Editor       => self.focus = Focus::CommandLine,
            Focus::CommandLine  => self.focus = Focus::Editor,
        }
    }
    #[allow(dead_code)]
    pub fn draw(&mut self){
        self.term.hide_cursor() .unwrap();
        self.term.clear_screen().unwrap();


        // draw header (if not hidden)
        // print div (with custom char & color)

        //check if header is hidden
        

        // ! TODO: refactor code.

        self.term.show_cursor() .unwrap();
    }
}

#[allow(dead_code)]
pub fn editor(){
    let mut term = console::Term::stdout();

    let mut focus  : Focus                  = Focus::Editor;
    let mut header : String                 = "Scawir - Text Editor".to_string();
    let mut editor : Vec<String>            = vec!["".to_string()];
    let mut command: String                 = "".to_string();
    let mut cursor : (usize, usize, usize)  = (0, 0, 0); // idx 0 & 1 are for editor cursor, idx 2 is for command line cursor

    // let mut Editor = Editor{
    //     term        :   term    ,
    //     focus       :   focus   ,
    //     header      :   header  ,
    //     editor      :   editor  ,
    //     command     :   command ,
    //     editor_dim  :   (0, 0)  ,
    //     cursor      :   cursor  ,
    // };

    loop {
        draw(&mut term, &mut header, &mut editor, &command);
            
        match focus {
            Focus::Editor       => editor_kio (&mut focus, &mut term, &mut editor, &mut cursor, &mut header),
            Focus::CommandLine  => cl_kio     (&mut focus, &mut term, &mut command, &mut cursor, &mut header),
        }
    }
}

fn editor_kio (focus: &mut Focus,term: &mut console::Term, editor: &mut Vec<String>, cursor: &mut (usize, usize, usize), header: &mut String){
    term.move_cursor_to(cursor.1+10, cursor.0+2).unwrap();

    match term.read_key().unwrap(){
        Key::ArrowRight => {
            if cursor.1 < editor[cursor.0].len(){
                cursor.1 += 1;
            } else if cursor.0 < editor.len()-1 {
                cursor.0 += 1;
                cursor.1 = 0;
            }
        },

        Key::ArrowLeft => {
            if cursor.1 > 0 {
                cursor.1 -= 1;
            } else if cursor.0 > 0 {
                cursor.0 -= 1;
            }
        },
        Key::ArrowUp => {
            if cursor.0 > 0 {
                if editor[cursor.0-1].len() > cursor.1 {
                    cursor.0 -= 1;
                } else {
                    cursor.0 -= 1;
                    cursor.1 = editor[cursor.0].len();
                }
            }
        },
        Key::ArrowDown => {
            if cursor.0 < editor.len()-1 {
                if editor[cursor.0+1].len() > cursor.1 {
                    cursor.0 += 1;
                } else {
                    cursor.0 += 1;
                    cursor.1 = editor[cursor.0].len();
                }
            }
        },
        Key::Backspace => {
            if (editor[cursor.0].len() == 0 && editor.len() > 1) {
                editor.remove(cursor.0); // remove the line
                cursor.0 -= 1;                         // we removed a line, so we need to go back 1 line
                cursor.1 = editor[cursor.0].len(); // set the character index to the last character of prev line
            } 
            else if (cursor.1 > 0) { 
                if (cursor.1 == editor[cursor.0].len()) {editor[cursor.0].pop();} // if the cursor is at the end of the line, pop the last character
                else {editor[cursor.0].remove(cursor.1);}
                cursor.1 -= 1;
            }
        },
        Key::Enter => {
            if cursor.1 == editor[cursor.0].len() {
                editor.insert(cursor.0 + 1, String::new());
                cursor.0 += 1;
                cursor.1 = 0;
            } else {
                let new_line = editor[cursor.0].split_off(cursor.1);
                editor.insert(cursor.0 + 1, new_line);
                cursor.0 += 1;
                cursor.1 = 0;
            }
        },
        Key::Char('\u{18}') =>{
            *header += "_2_";
            match focus {
                Focus::Editor       => *focus = Focus::CommandLine,
                Focus::CommandLine  => *focus = Focus::Editor,
            }
        },
        Key::Char(char) => {
            editor[cursor.0].insert(cursor.1, char);
            cursor.1 += 1;
        },
        _ => {}
    }
}
fn cl_kio(focus: &mut Focus , term: &mut console::Term, cmd_line: &mut String, cursor: &mut (usize, usize, usize), hdr: &mut String){
    term.move_cursor_to(cursor.2, (term.size().0 - 1).into() ).unwrap();
    match term.read_key().unwrap(){
        Key::Char('\u{18}') =>{
            *hdr += "_1_";
            match focus {
                Focus::Editor       => *focus = Focus::CommandLine,
                Focus::CommandLine  => *focus = Focus::Editor,
            }
        },
        Key::ArrowRight => {
            if cursor.2 < cmd_line.len() {
                cursor.2 += 1;
            }
        },
        Key::ArrowLeft => {
            if cursor.2 > 0 {
                cursor.2 -= 1;
            }
        },
        Key::Enter => {
            _process_command(cmd_line);
            *cmd_line = "".to_string();
            cursor.2 = 0;
        },
        Key::Backspace => {
            if cursor.2 > 0 {
                cmd_line.remove(cursor.2 - 1);
                cursor.2 -= 1;
            }
        },
        Key::Char(char) => {
            cmd_line. insert(
                if cursor.2 == cmd_line.len() {cursor.2} else {cursor.2 + 1}
                , char);
            cursor.2 += 1;
        },
        _ => {}
    }
}

fn draw(term: &mut console::Term, hdr: &String, editor: &mut Vec<String>, command_line: &String){
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();

    println!("{} ", hdr);
    println!("{} ", style("=".repeat((term.size().1-1) as usize)).color256(8));

    let avaible_height = (term.size().0 - 4) as usize;
    let start_line = if editor.len() > avaible_height {editor.len() - avaible_height} else {0};
    let end = editor.len();

    // * this needs to be changed to only draw a slice of the vector
    // * update the line numbers to reflect the slice
    for (line_num, line_content) in editor[start_line..end].iter().enumerate() {
        println!("{}{}", style(format!("{:0>1}\t| ", line_num+1)).cyan(), line_content);
    }
    for _ in 0..(avaible_height - (end - start_line)) {
        println!();
    }

    println!("{} ", style("=".repeat((term.size().1-1) as usize)).color256(8));
    print!("{}", command_line);

    // * https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

    // ! editor:
    // * DONE: implement system where empty lines between editor and command line are drawn 
    // TODO: clear screen on exit
    // TODO: improve cursor movement (currently it's a bit buggy)
    // TODO: implement system where only a slice of the vector (from 0..CommandLine) is shown on the screen to prevent overflow
    // TODO: implement a system where scrolling moves the slice of the vector shown on the screen and hence scrolling is possible
    std::io::stdout().flush().unwrap();
    term.show_cursor().unwrap();
}
    //* if possible: add scrolling support (might have to use crossterm since `Console` lacks support for raw inputs.) */

    // ! program:
    // TODO: disable or manage ctrl + C
    


    // header
//     println!("{} ", hdr);
//     // div.
//     println!("{} ", "=".repeat((term.size().1-1) as usize));
//     // draw editor
   
//    for (line_num, line_content) in editor.iter().enumerate() {
//         println!("{}{}", style(format!("{:0>3}\t| ", line_num)).cyan(), line_content);
//     }
//     // draw command line
//     println!("{} ", "=".repeat((term.size().1-1) as usize));
//     print!("size: {:?}, editor.len(): {}",term.size(), editor.len());

fn _process_command(_command: &mut String) {
    if _command == "exit" {
        print!("{}[2J", 27 as char);
        std::process::exit(0);
    }
}


/* // * From ChatGPT:

 * perhaps you could use a VecDeque instead of a Vec, and then use the rotate method to scroll the buffer

fn scroll_screen(editor_buffer: &Vec<String>, command_line: &str, screen_height: usize, scroll_amount: isize) {
    // Calculate how many lines are available for the editor buffer
    let available_lines = screen_height - 2; // 2 lines are reserved for the command line

    // Only show a slice of the editor buffer that fits on the screen
    let start_line = if editor_buffer.len() > available_lines { editor_buffer.len() - available_lines } else { 0 };
    let end_line = editor_buffer.len();

    // Calculate the new start and end line indices based on the scroll amount
    let new_start_line = (start_line as isize + scroll_amount) as usize;
    let new_end_line = (end_line as isize + scroll_amount) as usize;

    // Ensure that the new slice fits within the bounds of the editor buffer
    let new_start_line = new_start_line.clamp(0, editor_buffer.len());
    let new_end_line = new_end_line.clamp(0, editor_buffer.len());

    // Update the start and end indices
    let start_line = new_start_line;
    let end_line = new_end_line;

    // Draw the screen with the updated start and end indices
    draw_screen(&editor_buffer[start_line..end_line].to_vec(), command_line, screen_height);
}
*/
