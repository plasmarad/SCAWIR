#[allow(unused_parens)]
mod editor_;

mod editor;

// Editor manages the keyboard and what should be displayed on the screen
// The layout manages how the screen is displayed
// The Lang Module manages the language component. (IE: Custom lang's lexer, parser, etc.)
// The File Module manages the file component. (IE: File IO, etc.)

fn main() {
    // * To enable the new editor, uncomment the following lines:
    // let mut editor = editor_::Scawir::init();
    // editor.Run();
    // * To enable the old editor, uncomment the following lines:
    editor_::editor();
}

