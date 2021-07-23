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
struct  LayTerm {

}

impl LayTerm {
    
    fn new() -> Self{

        Self{}
    }

}
