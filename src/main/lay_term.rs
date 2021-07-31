use fltk::{
    app,
    enums::{Color, Event, Font, Key},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{SimpleTerminal, StyleTableEntry, TextBuffer},
    window::Window,
};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct  LayTerm {
    pub term: SimpleTerminal,
    pub dir: String,
    pub user: String
}

impl LayTerm {  
    pub fn new(buf:fltk::text::TextBuffer,x:i32,y:i32,z:i32,w:i32) -> Self{
        let mut term = SimpleTerminal::new(x,y,z,w,"");
        term.set_buffer(Some(buf));
        term.set_text("$");
        term.set_text_color(Color::from_rgb(0,25,255));
        Self {term:term,dir:String::new(),user:String::new()}
    }
}
