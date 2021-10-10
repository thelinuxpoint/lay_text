//std includes  ####################
use std::collections::{HashMap};
use std::path::{PathBuf,Path};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32,Ordering};
//fltk includes ####################
use fltk;
use fltk::app;
use fltk::group::{Tabs,Group,Tile,Scroll,VGrid,Pack,PackType};
use fltk::tree::*;
use fltk::dialog::{FileChooser,FileChooserType,NativeFileChooser,NativeFileChooserType,FileDialogType};
use fltk::window::Window;
use fltk::{prelude::*, *};
use fltk::enums::{Color,FrameType,Event};
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
use crate::messg::Message;
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
    editors:     Vec<lay_editor::LayEditor>,  /*editors with automic tab count and mapping*/
    tabs:        lay_tabs::ClosableTab, /**/
    tree:        Tree,
    tile:        Tile,
    send:        fltk::app::Sender<Message>,
    receive:     fltk::app::Receiver<Message>,
}

impl LayText{
    //############################################
    pub fn new() -> Self {
        let (s,r) = fltk::app::channel::<Message>();
        let mut lay_window = Window::new(0,0,900,650,"");
        let shape = prep_shape();
        // lay_window.set_shape(Some(shape));
        // lay_window.fullscreen(true);
        lay_window.set_color(Color::from_rgb(24,25,21));
        // lay_window.make_resizable(true);

        // lay_window.set_frame(FrameType::RoundedBox);
        let mut img = PngImage::load("./src/Icon/lay.png").unwrap();
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

        
        let mut tile = Tile::new(10,30,890,560,"");

        let mut tree = Tree::new(0,50,10,575,None);
        tile.handle(move |_,ev| match ev{
            Event::Drag =>{
                app::redraw();
                return true
            }
            _=>{
                return false
            }
        });


        tree.set_color(Color::from_rgb(24,25,21));
        tree.set_frame(FrameType::FlatBox);
        tree.set_scrollbar_size(5);
        tree.set_root_label("FOLDERS");
        tree.set_connector_style(TreeConnectorStyle::None);
        tree.set_select_frame(FrameType::NoBox);
        tree.clear_visible_focus();

        let tmp = lay_tabs::ClosableTab::new(10,30, 890, 560,&s);

        tile.end();


        Self{
            main_window: lay_window,
            tab_count:   0,
            app:         fltk::app::App::default().with_scheme(app::Scheme::Base),
            tabs:        tmp,
            tree:        tree,
            tile:        tile,
            editors:     Vec::new(),
            send:        s,
            receive:     r,
            
        }
    }
    //############################################
    pub fn layapp(&mut self){
        
        let _menu = lay_menubar::LayMenuBar::new(&self.send);

        let mut menu_bar_bottom = SysMenuBar::new(0,626,900,24,"");
        menu_bar_bottom.set_frame(FrameType::FlatBox);
        menu_bar_bottom.set_text_color(Color::from_rgb(255,255,255));
        menu_bar_bottom.set_selection_color(Color::from_rgb(0,0,0));
        menu_bar_bottom.set_text_size(13);
        menu_bar_bottom.set_color(Color::from_rgb(19,20,17));

        let mut _start   = lay_menubar::LayBarStart::new(&self.send);
        let mut _mid = lay_menubar::LayBarMid::new();
        let mut _end = lay_menubar::LayBarEnd::new();


        
        let y = self.tabs.grp.clone();
        self.main_window.resizable(&y);
        self.main_window.size_range(600,400,app::screen_size().0 as i32,app::screen_size().1 as i32);
        self.main_window.end();
        self.main_window.show();
        self.launch(&mut _end);
    }
    //############################################
    fn new_tab(&mut self,name:&'static str )-> group::Group {

        let tab = self.tab_count.clone();
        println!("{:?}",self.tabs.grp.x());
        let mut grp = group::Group::new(self.tabs.grp.x(),self.tabs.grp.y(),self.tabs.grp.w(),self.tabs.grp.h(),None);
        grp.set_label_color(Color::from_rgb(255,255,255));
        grp.set_label(name);
        self.editors.push(lay_editor::LayEditor::new(fltk::text::TextBuffer::default(),&self.tabs.grp));
        grp.end();
        self.tabs.add(&mut grp);
        self.tab_count+=1;
        grp
        // println!("{}",self.tabs.active_tab.load(Ordering::SeqCst));
    }
    //############################################
    pub fn launch(&mut self,menu:&mut lay_menubar::LayBarEnd){
        
        while self.app.wait(){

            if let Some(x) = self.receive.recv() {
                match x {
                    // Handle the new file event ##############################################
                    Message::New => {
                        self.new_tab("");
                        println!("LayText~> New Tab (Count : \x1b[36m{}\x1b[0m)",self.tab_count);
                        // redraw the window to see the changes
                        self.main_window.redraw();
                    },
                    // file opening Protocols ///////////////////////////
                    Message::Open =>{
                        print!("LayText~> Opening ... ");
                        let mut chooser = NativeFileChooser::new(FileDialogType::BrowseFile);
                        chooser.show();
                        if chooser.filename().is_file(){
                            match chooser.filename().file_name(){
                                Some(_xyz) => {
                                    self.new_tab("");
                                    self.tabs.c[(self.tabs.children-1) as usize].set_label((String::from("  ")+chooser.filename().file_name().unwrap().to_str().unwrap()+"   \u{2713}").as_str());          

                                    if let Some(x) = self.editors.get_mut((self.tabs.children-1) as usize) {
                                        x.path = chooser.filename();
                                        x.is_defined=true;
                                        let mut buf = x.buffer().unwrap();
                                        buf.set_tab_distance(4);
                                        buf.load_file(chooser.filename());
                                        x.length=buf.length();
                                        x.is_saved = true;
                                        file_type_changer(&x.path,menu);
                                        app::redraw();
                                        println!("{:?} length : {} bytes",chooser.filename(),buf.length());
                                    }        
                                },
                                None => {
                                    
                                    dialog::alert_default("Please Select a file");
                                }
                            }
                        }else{}
                    },

                    // delete the editor and release the memory
                    Message::Closed(idx) =>{
                        println!("LayText~> Closing ... {}",idx);
                        self.editors.remove(idx as usize);
                        self.tabs.c.remove(idx as usize);
                    },

                    // Saving Protocols ######################################
                    Message::Save => {
                        print!("LayText~> Saving ... ");
                        if !self.editors[self.tabs.active_tab.load(Ordering::SeqCst) as usize].is_defined {
                            let mut chooser = NativeFileChooser::new(
                                FileDialogType::BrowseSaveFile
                            );
                            chooser.show();
                            match chooser.filename().file_name(){
                                Some(_xyz) =>{
                                    println!("{:?}",chooser.filename());
                                    if let Some(x) = self.editors.get_mut( self.tabs.active_tab.load(Ordering::SeqCst) as usize) {
                                        self.tabs.c[self.tabs.active_tab.load(Ordering::SeqCst) as usize].set_label((String::from("  ")+chooser.filename().file_name().unwrap().to_str().unwrap()+"   \u{2713}").as_str());
                                        x.path = chooser.filename();
                                        x.is_defined=true;
                                        x.buffer().unwrap().save_file(chooser.filename());
                                        x.length=x.buffer().unwrap().length();
                                        x.is_saved = true;
                                        app::redraw();
                                    }
                                }
                                None => {
                                    dialog::alert(100,100,"Please give a filename");
                                }
                            }
                        }

                        else {
                            if let Some(x) = self.editors.get_mut( self.tabs.active_tab.load(Ordering::SeqCst) as usize) {                     
                                x.buffer().unwrap().save_file(x.path.clone());
                                x.length=x.buffer().unwrap().length();
                                x.is_saved = true;
                            }
                        }
                    }
                    
                    Message::SideBar(id) =>{
                        let w=self.main_window.w();
                        let x0 = self.tabs.parent_grp.x();
                        let y0 = self.tabs.parent_grp.y();

                        let x = self.tabs.parent_grp.w();
                        let y = self.tabs.parent_grp.h();
                        if id==0{
                            self.tree.resize(0,50,200,self.tabs.grp.h());
                            self.tree.activate();
                            self.tabs.parent_grp.resize(200,y0,x-200,y);
                            self.tabs.parent_grp.redraw();
                            self.main_window.redraw();
                            app::redraw();
                        }
                        else{
                            self.tree.resize(0,50,0,self.tabs.grp.h());    
                            self.tabs.parent_grp.resize(0,y0,w,y);
                            self.tabs.parent_grp.redraw();
                            self.tree.deactivate();
                            self.main_window.redraw();
                            app::redraw();
                        }

                    }
                    _=>{}
                }
            }
            //################################################
            else {
                match app::event(){
                    Event::KeyUp => {
                        if self.editors.len() > 0{
                            let i = self.tabs.active_tab.load(Ordering::SeqCst);
                            let mut temp = self.tabs.c[i as usize].label();
                            temp.pop();
                            if let Some(x) = self.editors.get_mut(i as usize) {
                                file_type_changer(&x.path,menu);
                                let len = x.length;
                                let buf_len = x.buffer().unwrap().length();
                                if len!=buf_len {
                                    temp+="\u{25aa}";
                                    x.is_saved = false;
                                    self.tabs.c[i as usize].set_label(temp.as_str());
                                }
                                else {
                                    if len==buf_len && self.editors[i as usize].is_saved{
                                        temp+="\u{2713}";
                                        self.tabs.c[i as usize].set_label(temp.as_str());
                                    }
                                }
                                self.tabs.grp.redraw();
                            }
                        }
                    }
                    _=>{

                    }
                }
            } 
        }
    }
}

//########################################################################
