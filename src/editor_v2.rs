use console::Term;
use console::Key;
use console::style;

pub fn init()  {
    let term = Term::stdout();
    let mut screen_buffer = String::new();

    let console_res = term.size();

    let mut cursor_pos: (usize, usize) = (0, 0);

    term.set_title("SCAWIR");

    term.clear_screen().unwrap();

    while term.read_key().unwrap() == Key::Unknown {}

    let mut editor_buffer: Vec<String> = vec![String::new()];

    loop {
        //  run this in parallel with the keyboard input.
        //  and run this as a widget in the editor. 
        // "term.clear_screen().unwrap();
        // for (line_number, line) in editor_buffer.iter().enumerate() {
        //     println!("{}\t| {}", 
        //         style((line_number + 1).to_string()).color256(8),
        //         line);
        // }";
        k_io(&term, &mut screen_buffer, &mut editor_buffer, &mut cursor_pos);
    }

}
     

pub fn k_io( term: &Term, screen_buffer: &mut String, editor_buffer: &mut Vec<String>, cursor_position: &mut (usize, usize)){
    term.move_cursor_to(cursor_position.1+10, cursor_position.0).unwrap();

    match term.read_key().unwrap(){
        Key::ArrowRight => {
            if cursor_position.1 < editor_buffer[cursor_position.0].len() {
                cursor_position.1 += 1;
            } else if cursor_position.0 + 1 < screen_buffer.len() {
                cursor_position.0 += 1; 
                cursor_position.1 = 0;
            }
        },
        Key::ArrowLeft => {
            if cursor_position.1 > 0 {
                cursor_position.1 -= 1;
            } else if cursor_position.0 > 0 {
                cursor_position.0 -= 1;
            }
        },
        Key::ArrowUp => {
            if cursor_position.0 > 0 { cursor_position.0 -= 1; }
        },
        Key::ArrowDown => {
            if cursor_position.0 + 1 < screen_buffer.len() 
                                            { cursor_position.0 += 1; }
        },
        Key::Char ('\u{18}') => {
            
        },
        Key::Backspace => {
            if (editor_buffer[cursor_position.0].len() == 0 && editor_buffer.len() > 1) {
                editor_buffer.remove(cursor_position.0); // remove the line
                cursor_position.0 -= 1;                         // we removed a line, so we need to go back 1 line
                cursor_position.1 = editor_buffer[cursor_position.0].len(); // set the character index to the last character of prev line
            } 
            else if (cursor_position.1 > 0) { 
                if (cursor_position.1 == editor_buffer[cursor_position.0].len()) {editor_buffer[cursor_position.0].pop();} // if the cursor is at the end of the line, pop the last character
                else {editor_buffer[cursor_position.0].remove(cursor_position.1);}
                cursor_position.1 -= 1;
            }
        },
        Key::Enter => {
            editor_buffer.insert(cursor_position.0 + 1, String::new());
            cursor_position.0 += 1;
            cursor_position.1 = 0;
        },
        Key::Char(char) => {
            editor_buffer[cursor_position.0] += &char.to_string();
            cursor_position.1 += 1;
        },
        _ => {
        }
    }
}