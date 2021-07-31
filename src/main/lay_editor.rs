use std;
use fltk::text::*;
use fltk::prelude::*;
use std::path::PathBuf;
use fltk::enums::{Color,Font,FrameType};

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
    pub fn new(buf:fltk::text::TextBuffer,x:i32,y:i32,z:i32)->Self {

        let mut term = TextEditor::new(z,63,x-z,y-87,"");
        term.set_color(Color::from_rgb(40,41,35));
        term.set_buffer(Some(buf));
        term.set_cursor_style(Cursor::Simple);
        term.set_text_color(Color::from_rgb(255,255,255));
        term.set_text_font(Font::Courier);
        term.set_frame(FrameType::FlatBox);
        term.set_text_size(14);
        term.set_scrollbar_size(10);
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
