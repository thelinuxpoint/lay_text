use fltk::enums::Color;
use fltk::enums::Font;
use fltk::text::*;
use fltk::prelude::*;
use std;
use fltk::enums::FrameType;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LayEditor {
    pub editor:     fltk::text::TextEditor,
    pub is_defined: bool,
    pub path:       PathBuf,
    pub length:     i32,
    pub is_saved:   bool,

}
impl LayEditor {
    //#######################################################
    pub fn new(buf:fltk::text::TextBuffer)->Self {

        let mut term = TextEditor::new(10,63,880,530,"");
        term.set_color(Color::from_rgb(40,41,35));
        term.set_buffer(Some(buf));
        term.set_cursor_style(Cursor::Simple);
        term.set_text_color(Color::from_rgb(255,255,255));
        term.set_text_font(Font::Courier);
        term.set_frame(FrameType::FlatBox);
        term.set_text_size(14);
        term.set_scrollbar_size(0);
        term.set_cursor_color(Color::White);
        term.set_linenumber_width(35); 
        term.set_linenumber_bgcolor(Color::from_rgb(40,41,35));
        term.set_linenumber_fgcolor(Color::from_rgb(216,205,175));
        term.set_selection_color(Color::from_rgb(60,60,55));
 
        Self{
            editor:      term,
            is_defined:  false,
            path:        PathBuf::new(),
            length:      0,
            is_saved:    false
        }
    }
    //#######################################################
}
impl std::ops::Deref for LayEditor{
    type Target = fltk::text::TextEditor;
    fn deref(&self)-> &Self::Target{
        &self.editor
    }
}
impl std::ops::DerefMut for LayEditor{

    fn deref_mut(&mut self)-> &mut Self::Target{
        &mut self.editor
    }
}
