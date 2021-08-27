//std includes  ####################
use std::collections::{HashMap};
use std::path::{PathBuf,Path};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32,Ordering};
//fltk includes ####################
use fltk;
use fltk::dialog::{FileChooser,FileChooserType,NativeFileChooser,NativeFileChooserType,FileDialogType};
use fltk::window::Window;
use fltk::{prelude::*, *};
use fltk::enums::{Color,FrameType};
use fltk::image::{SvgImage,PngImage};
use fltk::text::{TextBuffer};
use fltk::button::Button;
use fltk::enums;
use fltk::frame::Frame;
use fltk::menu::*;

//##################################
mod lay_menubar;
mod lay_editor;
mod lay_tabs;

//##################################
// General Functions ####################################################
fn prep_shape() -> image::RgbImage {
    let surf = surface::ImageSurface::new(900, 600, false);
    surface::ImageSurface::push_current(&surf);
    draw::draw_rect_fill(0,0,900, 650 ,enums::Color::from_rgb(24,25,21));
    let img = surf.image().unwrap();
    surface::ImageSurface::pop_current();
    img
}
fn file_type_changer(arg:&PathBuf,menu:&mut lay_menubar::LayBarEnd){
    if let Some(c) = arg.extension(){

        if c=="java"{
            menu.menu.set_label("Java");
        }
        else if c=="rs"{
            menu.menu.set_label("Rust");
        }
        else if c=="c++" || c=="cxx" || c=="cpp"{
            menu.menu.set_label("C++");
        }
        else if c=="php" {
            menu.menu.set_label("PHP");
        }
        else if c=="rb" {
            menu.menu.set_label("Ruby");
        }
        else if c=="py" {
            menu.menu.set_label("Python");
        }
        else if c=="sh" {
            menu.menu.set_label("Bashscript");
        }
        else if c=="js" {
            menu.menu.set_label("Javascript");
        }
        else if c=="jsp" {
            menu.menu.set_label("JSP");
            menu.menu.set_tooltip("Java Server Pages");
        }
        else if c=="svg" || c=="xml" || c=="xslt" {
            menu.menu.set_label("XML");
            menu.menu.set_tooltip("Extensible Markup Language");
        }
        else if c=="png" {
            menu.menu.set_label("PNG");
            menu.menu.set_tooltip("Portable Network Graphics");
        }
    }
    else{
        menu.menu.set_label("Plain Text");
    }
}
//##################################################################
pub struct LayText{
    main_window: Window,
    tab_count:   i32,
    app:         fltk::app::App,
    editors:     HashMap<i32,lay_editor::LayEditor>,  /*editors with automic tab count and mapping*/
    tabs:        lay_tabs::ClosableTab,
    send:        fltk::app::Sender<lay_menubar::Message>,
    receive:     fltk::app::Receiver<lay_menubar::Message>,
}

impl LayText{

    pub fn new() -> Self {
        let (s,r) = fltk::app::channel::<lay_menubar::Message>();
        let mut lay_window = Window::new(0,0,900,650,"");
        let shape = prep_shape();
        lay_window.set_shape(Some(shape));
        lay_window.set_color(Color::from_rgb(24,25,21));
        lay_window.make_resizable(true);

        // lay_window.set_frame(FrameType::RoundedBox);
        let mut img = PngImage::load("./src/Icon/48x48.png").unwrap();
        // img.scale(20,20,true,true);
        lay_window.set_icon(Some(img));

        lay_window.handle({
            let mut x = 0;
            let mut y = 0;
            move |w, ev| match ev {
                enums::Event::Push => {
                    let coords = app::event_coords();
                    x = coords.0;
                    y = coords.1;
                    true
                },
                enums::Event::Drag => {
                    if y<50{
                        w.set_cursor(enums::Cursor::Move);
                        w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                        true
                    }else {
                        false
                    }
                },
                enums::Event::Released => {
                    w.set_cursor(enums::Cursor::Default);
                    true
                },
                enums::Event::MouseWheel => {
                    
                    false
                },
                _ => false,
            }
        });
        let mut frame = Frame::new(450,10,15,15,"");
        frame.set_label_color(Color::from_rgb(255,255,255));
        frame.set_label_size(15);
        frame.set_label("LayText Editor");
        frame.set_label_font(enums::Font::ScreenBold);

        let mut close = Button::new(875,10,15,15,None);
        let mut image_open = SvgImage::load("./src/close_main.svg").unwrap();
        image_open.scale(12,12,true,true);
        close.set_image(Some(image_open));
        close.set_frame(FrameType::NoBox);
        close.set_frame(enums::FrameType::OFlatFrame);
        close.set_color(Color::from_rgb(255,0,0));
        close.clear_visible_focus();
        close.set_callback(move |_| {println!("Closing");app::quit();});
        close.redraw(); 
        
        let mut max = Button::new(855,10,15,15,None);
        let mut image = SvgImage::load("./src/maximize.svg").unwrap();
        image.scale(11,11,true,true);
        max.set_image(Some(image));
        max.set_frame(FrameType::NoBox);
        max.set_color(Color::from_rgb(0,180,0));
        max.set_frame(enums::FrameType::OFlatFrame);
        max.clear_visible_focus();
        max.set_callback(move |_| println!("Maximize")); 

        let mut min = Button::new(835,10,15,15,None);
        let mut image = SvgImage::load("./src/minimize.svg").unwrap();
        image.scale(11,11,true,true);
        min.set_image(Some(image));
        min.set_frame(FrameType::NoBox);
        min.set_color(Color::from_rgb(180,180,0));
        min.set_frame(enums::FrameType::OFlatFrame);

        min.clear_visible_focus();
        min.set_callback(move |_| println!("Minimize")); 

        Self{
            main_window: lay_window,
            tab_count:   0,
            app:         fltk::app::App::default().with_scheme(app::Scheme::Base),
            tabs:        lay_tabs::ClosableTab::new(0,60, 900, 525),
            editors:     HashMap::new(),
            send:        s,
            receive:     r,
        }
    }

    pub fn layapp(&mut self){
        
        let _menu = lay_menubar::LayMenuBar::new(&self.send);

        let mut menu_bar_bottom = SysMenuBar::new(0,626,900,23,"");
        menu_bar_bottom.set_frame(FrameType::FlatBox);
        menu_bar_bottom.set_text_color(Color::from_rgb(255,255,255));
        menu_bar_bottom.set_selection_color(Color::from_rgb(0,0,0));
        menu_bar_bottom.set_text_size(13);
        menu_bar_bottom.set_color(Color::from_rgb(19,20,17));

        let _start   = lay_menubar::LayBarStart::new();
        let mut _mid = lay_menubar::LayBarMid::new();
        let mut _end = lay_menubar::LayBarEnd::new();

        let mut prev = Button::new(5,75,15,15,None);
        let mut image = SvgImage::load("./src/mono-navigator-prev.svg").unwrap();
        image.scale(13,13,true,true);
        prev.set_image(Some(image));
        prev.set_frame(FrameType::NoBox);
        prev.set_color(Color::from_rgb(180,180,0));
        prev.set_frame(enums::FrameType::NoBox);
        prev.set_tooltip("Scroll Tabs");
        prev.clear_visible_focus();
        
        
        let mut next = Button::new(25,75,15,15,None);
        let mut image = SvgImage::load("./src/mono-navigator-next.svg").unwrap();
        image.scale(13,13,true,true);
        next.set_image(Some(image));
        next.set_frame(FrameType::NoBox);
        next.set_color(Color::from_rgb(180,180,0));
        next.set_frame(enums::FrameType::NoBox);
        next.set_tooltip("Scroll Tabs");
        next.clear_visible_focus();
        next.handle({
            let mut self_grp = self.tabs.hscroll.clone();
            move |w, ev| match ev {
                enums::Event::Push => {
                    println!("xpos: {} : Maximum: {} : Minimum: {}",self_grp.xposition(),self_grp.hscrollbar().maximum(),self_grp.hscrollbar().minimum());
                    self_grp.scroll_to(self_grp.xposition()+1,self_grp.yposition());
                    true
                },
                _ => false,
            }
        });

        self.main_window.size_range(600,400,app::screen_size().0 as i32,app::screen_size().1 as i32);
        self.main_window.end();
        self.main_window.show();
        self.launch();
    }

    fn new_tab(&mut self){

        self.tab_count+=1;
        let mut grp = group::Group::new(self.tabs.grp.x(),self.tabs.grp.y(),self.tabs.grp.w(),self.tabs.grp.h(),None);
        grp.set_label_color(Color::from_rgb(255,255,255));
        grp.set_label(" untitled");
        lay_editor::LayEditor::new(fltk::text::TextBuffer::default(),&self.tabs.grp);
        grp.end();
        self.tabs.add(&mut grp);
        // println!("{}",self.tabs.active_tab.load(Ordering::SeqCst));
    }

    pub fn launch(&mut self){

        while self.app.wait(){
            if let Some(msg) = self.receive.recv(){
                match msg {
                    // Handle the new file event ##############################################
                    lay_menubar::Message::New => {
                        self.new_tab();
                        println!("LayText~> New Tab (Count : \x1b[36m{}\x1b[0m)",self.tab_count);
                        // redraw the window to see the changes
                        self.main_window.redraw();
                    },
                    lay_menubar::Message::Open =>{
                        print!("LayText~> Opening ... ");
                        let mut chooser = NativeFileChooser::new(FileDialogType::BrowseFile);
                        chooser.show();
                    }
                    _=>{

                    }
                }
            }
        }
    }
}

//########################################################################