use fltk::app;
use fltk::prelude::*;
use fltk::menu::*;
use fltk::enums::Color;
use fltk::enums::Shortcut;
use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::button::Button;
use fltk::image::{SvgImage,PngImage};

#[derive(Clone,Copy,Debug)]
pub enum Message{
    New,
    OpenTerm,
    OpenFolder,
    Open,
    Close,
    Save,
    SaveAs,
    SideBar,
    Quit,
    None
}

pub struct LayMenuBar{
    pub menu: fltk::menu::SysMenuBar
}

impl LayMenuBar{
    pub fn new(s: &app::Sender<Message>)-> Self{
        let mut menu_bar = SysMenuBar::new(0,0,900,30,"");
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
        menu_bar.add_emit("&File/Open Terminal\t",
            Shortcut::Ctrl | 't',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::OpenTerm
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
            Message::SaveAs
        ); 
        menu_bar.add_emit("&File/Open Folder \t",
            Shortcut::None,
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::OpenFolder
        );

        menu_bar.add_emit("&File/Close File \t",
            Shortcut::Ctrl | 'w',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Close
        );
        menu_bar.add_emit("&File/Quit \t",
            Shortcut::Ctrl | 'q',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::Quit
        );
        menu_bar.add_emit("&Edit/Paste \t",
            Shortcut::Ctrl | 'v',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::None
        ); 
        menu_bar.add_emit("&Edit/Undo \t",
            Shortcut::Ctrl | 'z',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::None
        );
        menu_bar.add_emit("&Edit/Copy Line \t",
            Shortcut::Ctrl | 'l',
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::None
        );

        menu_bar.add_emit("&Preferences/Save On Focus Lost \t",
            Shortcut::None,
            fltk::menu::MenuFlag::Normal,
            *s,
            Message::None
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
//##################################################################3
pub struct LayBarStart{
    pub sidebar:  Button,
    pub terminal: Button,
    pub stat: bool
}
impl LayBarStart{
    pub fn new(s: &app::Sender<Message>)-> Self{
        let mut sidebar = Button::new(0,584,35,20,"");
        let mut image_open = SvgImage::load("./src/sidebar.svg").unwrap();
        image_open.scale(16,17,true,true);
        sidebar.set_image(Some(image_open));
        sidebar.set_frame(FrameType::NoBox);
        sidebar.clear_visible_focus();
        
        let mut terminal = Button::new(30,587,35,20,"");
        let mut image = SvgImage::load("./src/lay-terminal.svg").unwrap();
        image.scale(16,17,true,true);
        terminal.set_image(Some(image));
        terminal.set_frame(FrameType::NoBox);
        terminal.clear_visible_focus();
        terminal.set_tooltip("Open Terminal");
        
        Self{
            sidebar: sidebar,
            terminal:terminal,
            stat:false
        }
    }
}

//##############################################################
pub struct LayBarMid{
    pub menu: fltk::menu::SysMenuBar,

}
impl LayBarMid{
    pub fn new(s: &app::Sender<Message>)-> Self{
        let mut menu_bar = SysMenuBar::new(62,578,60,23,"");
        menu_bar.set_frame(FrameType::FlatBox);
        menu_bar.set_text_color(Color::from_rgb(255,255,255));
        menu_bar.set_selection_color(Color::from_rgb(10,10,100));
        menu_bar.set_text_size(12);
        menu_bar.set_color(Color::from_rgb(19,20,17));
        menu_bar.add_emit(format!("&Line: {}, Column {}",1,1 ).as_str(),
            Shortcut::Ctrl | 'k',
            fltk::menu::MenuFlag::Value,
            *s,
            Message::SideBar
        ); 
        menu_bar.set_label_size(10);
        menu_bar.set_tooltip("Line ");

        Self{menu:menu_bar}
    }
}

//##############################################################
pub struct LayBarEnd{
    pub menu: Frame
}
impl LayBarEnd{
    pub fn new(s: &app::Sender<Message>)-> Self{
        let mut menu_bar = Frame::new(800,578,90,23,"Plain Text");
        // menu_bar.set_frame(FrameType::FlatBox);
        
        // menu_bar.set_selection_color(Color::from_rgb(0,0,0));
        menu_bar.set_label_size(13);
        menu_bar.set_color(Color::from_rgb(255,255,255));
        Self{menu:menu_bar}
    }
}
//##############################################################
