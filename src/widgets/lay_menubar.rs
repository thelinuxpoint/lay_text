use std::sync::Arc;
use std::sync::atomic::{AtomicI32,Ordering};

use fltk::app;
use fltk::prelude::*;
use fltk::menu::*;
use fltk::enums::{Color,Event,Font,Shortcut,FrameType};
use fltk::frame::Frame;
use fltk::button::Button;
use fltk::image::{SvgImage,PngImage};
use fltk::menu::SysMenuBar;

use crate::widgets::Message;


pub struct LayMenuBar{
    pub menu: fltk::menu::SysMenuBar
}

impl LayMenuBar{
    pub fn new(s: &app::Sender<Message>)-> Self{
        let mut menu_bar = SysMenuBar::new(0,0,900,30,"");
        menu_bar.set_frame(FrameType::FlatBox);
        menu_bar.set_text_color(Color::from_rgb(255,255,255));
        menu_bar.set_selection_color(Color::from_rgb(255,0,0));
        menu_bar.set_text_size(15);
        menu_bar.set_text_font(Font::Symbol);
        menu_bar.set_down_frame(FrameType::FlatBox);
        menu_bar.set_color(Color::from_rgb(24,25,21));
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
        );menu_bar.add_emit("&File/Open Recent \t",
            Shortcut::Ctrl | 'o',
            fltk::menu::MenuFlag::Submenu,
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
        let mut sidebar = Button::new(0,635,35,20,"");
        let mut image_open = SvgImage::load("./src/Icon/sidebar.svg").unwrap();
        image_open.scale(18,18,true,true);
        
        let mut image_close = SvgImage::load("./src/Icon/sidebarc.svg").unwrap();
        image_close.scale(18,18,true,true);
        
        sidebar.set_image(Some(image_open.clone()));
        sidebar.set_frame(FrameType::NoBox);
        sidebar.clear_visible_focus();
        let mut random = Arc::new(AtomicI32::new(0));
        sidebar.handle({
            let x = s.clone();
            let t = random.clone();
        move |w, ev| 
            match ev {
                Event::Push => {
                    if t.load(Ordering::SeqCst) == 0{                        
                        t.store(1,Ordering::SeqCst);
                        w.set_image(Some(image_close.clone()));
                        x.send(Message::SideBar(t.load(Ordering::SeqCst)));
                        app::redraw();
                    }else {
                        t.store(0,Ordering::SeqCst);
                        w.set_image(Some(image_open.clone()));
                        x.send(Message::SideBar(t.load(Ordering::SeqCst))); 
                        app::redraw();

                    }
                    true
                },
                _=>{false}
            }
        });

        let mut terminal = Button::new(250,635,35,20,"");
        let mut image = SvgImage::load("./src/Icon/lay-terminal.svg").unwrap();
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
    pub menu: Frame,
}

impl LayBarMid{
    pub fn new()-> Self{
        let mut menu_bar = Frame::new(62,626,150,23,"Line 1, Column 1");
        menu_bar.set_label_size(12);
        menu_bar.set_label_color(Color::from_rgb(255,255,255));
        Self{menu:menu_bar}
    }
}

//##############################################################
pub struct LayBarEnd{
    pub menu: Frame
}

impl LayBarEnd{
    pub fn new()-> Self{
        let mut menu_bar = Frame::new(800,626,90,23,"Plain Text");
        menu_bar.set_label_size(12);
        menu_bar.set_label_color(Color::from_rgb(255,255,255));
        Self{menu:menu_bar}
    }
}
//##############################################################
