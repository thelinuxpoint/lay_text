use fltk::app;
use fltk::prelude::*;
use fltk::menu::*;
use fltk::enums::Color;
use fltk::enums::Shortcut;
use fltk::enums::FrameType;


#[derive(Clone,Copy,Debug)]
pub enum Message{
    New,
    Open,
    Save,
    SaveAs,
}

pub struct LayMenuBar{
    pub menu: fltk::menu::MenuBar
}

impl LayMenuBar{
    pub fn new(s: &app::Sender<Message>)-> Self{
        let mut menu_bar = MenuBar::new(0,0,900,30,"");
        menu_bar.set_frame(FrameType::FlatBox);
        menu_bar.set_text_color(Color::from_rgb(255,255,255));
        menu_bar.set_selection_color(Color::from_rgb(10,10,100));
        menu_bar.set_text_size(15);

        menu_bar.add_emit("&File/New File\t",
            Shortcut::Ctrl | 'n',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::New
        );
        menu_bar.add_emit("&File/Open File \t",
            Shortcut::Ctrl | 'o',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        );
        menu_bar.add_emit("&File/Save \t",
            Shortcut::Ctrl | 's',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Save
        );
        menu_bar.add_emit("&File/Save As \t",
            Shortcut::Ctrl | 'S',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        ); 
        menu_bar.add_emit("&File/Quit \t",
            Shortcut::Ctrl | 'q',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        );
        menu_bar.add_emit("&Edit/Paste \t",
            Shortcut::Ctrl | 'v',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        ); 
        menu_bar.add_emit("&Edit/Undo \t",
            Shortcut::Ctrl | 'z',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        );
        menu_bar.add_emit("&Edit/Copy Line \t",
            Shortcut::Ctrl | 'l',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        );
        menu_bar.add_emit("&Help/About \t",
            Shortcut::Ctrl | 'x',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Open
        );

        Self{menu:menu_bar}
    }
}
