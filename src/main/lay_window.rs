use std::sync::atomic::{AtomicI32,Ordering};
use std::sync::Arc;
use std::collections::VecDeque;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
// fltk's 
use fltk::tree::TreeConnectorStyle;
use fltk::prelude::*;
use fltk::tree::*;
use fltk::app;
use fltk::dialog;
use fltk::window::*;
use fltk::menu::*;
use fltk::app::Scheme;
use fltk::app::MouseButton;
use fltk::button::Button;
use fltk::text::TextBuffer;
use fltk::image::PngImage;
use fltk::group::{Tabs,Group,Tile,Scroll,VGrid,Pack,PackType};
use fltk::enums::{Color,Shortcut,FrameType,Event,Align};
use fltk::dialog::{FileChooser,FileChooserType,NativeFileChooser,NativeFileChooserType,FileDialogType};
//########################################################
pub mod lay_editor;
pub mod lay_menubar;
pub mod lay_version;
pub mod lay_term;
//###########################################################################################
pub struct LayText{
    tabcount:      i32,
    receive:       fltk::app::Receiver<lay_menubar::Message>, /*message receiver*/ 
    send:          fltk::app::Sender<lay_menubar::Message>,   /*message sender*/
    applet:        fltk::app::App,  /*The app*/                       
    window:        OverlayWindow,   /*The Main Window*/
    current_tab:   Arc<AtomicI32>,  /*ARC for setting Current TAB*/
    editors:       HashMap<i32,lay_editor::LayEditor>,  /*editors with automic tab count and mapping*/
    map:           VecDeque<i32>,/*the useful member for mapping tabs with index*/
    // folders:       VecDeque<VecDeque<String>>/**/
}

impl LayText{
    //#########################################################################################
    pub fn new()-> Self{
        let (s,r) = fltk::app::channel::<lay_menubar::Message>();
        app::background(22,23,19);
        app::foreground(200,200,200);
        let mut window = OverlayWindow::new(0, 0, 900, 600, "Lay Text").center_screen();
        window.set_color(Color::from_rgb(24,25,21));
        window.make_resizable(true);

        Self {
            tabcount:      0,
            receive:       r,
            send:          s,
            applet:        fltk::app::App::default().with_scheme(app::Scheme::Plastic),
            window:        window,
            current_tab:   Arc::new(AtomicI32::new(1)),
            editors:       HashMap::new(),
            map:           VecDeque::new(),
            // folders:       VecDeque::new()
        }
    }
    //#########################################################################################
    pub fn layapp(&mut self){

        // Menu Bar Setting ################################
        let _menu = lay_menubar::LayMenuBar::new(&self.send);
        let _menu2 = lay_menubar::LayBarBottom::new(&self.send);

        let mut tile = Tile::new(0,35,890,542,"");
        let mut tree = Tree::new(0,35,10,542,None);
        tree.set_color(Color::from_rgb(24,25,21));
        tree.set_frame(FrameType::FlatBox);
        tree.set_scrollbar_size(10);
        tree.set_root_label("FOLDERS");
        tree.set_connector_style(TreeConnectorStyle::None);
        tree.set_select_frame(FrameType::NoBox);
        // self.terminals.insert(0,lay_term::LayTerm::new(fltk::text::TextBuffer::default(),0,577,900,2));        
        let mut tabs = Tabs::new(10,35,890,542,"");
        tabs.set_label_color(Color::from_rgb(255,255,255));
        tabs.set_selection_color(Color::from_rgb(40,41,35));
        tabs.set_frame(FrameType::FlatBox);

        let cur = Arc::clone(&self.current_tab);
        
        tile.handle(move |_,ev| match ev{
            Event::Drag =>{
                app::redraw();
                return true
            }
            _=>{
                return false
            }
        });


        tile.end();
   
        self.window.resizable(&tile);
        self.window.size_range(400,500,app::screen_size().0 as i32,app::screen_size().1 as i32);
        self.window.end();
        self.window.show();
        // now finally launch it
        println!("Launching LayText(\x1b[37m{}\x1b[0m) ... \u{2191}",lay_version::VERSION);
        self.launch(&mut tabs,&mut tree);
        println!("LayText~> GoodBye ...");
    }
    //#########################################################################################
    //#########################################################################################
    pub fn insert_tab(&mut self,tabs:i32)-> Group {

        self.tabcount+=1;
        let tab = self.tabcount.clone(); 
        let mut group = Group::new(tabs,60,self.window.width()-10,self.window.height()-87," untitled    \u{2713}");
        group.set_color(Color::from_rgb(255,255,255));
        group.set_label_size(12);
        group.begin();
        self.editors.insert(tab,lay_editor::LayEditor::new(fltk::text::TextBuffer::default(),self.window.width(),self.window.height(),tabs));        
        group.resizable(&self.editors[&tab].editor); 
        group.end();
        self.map.push_back(tab);

        // atomic variable for setting the current tab
        let cur = Arc::clone(&self.current_tab);
        group.handle(move |_,x| match x{
            Event::Focus => {
                cur.store(tab, Ordering::SeqCst);
                return true
            }
            Event::Push => {
                cur.store(tab, Ordering::SeqCst);
                return true
            }
            _ => {
                false
            }
        });
        group
    }
    //#########################################################################################
    pub fn launch(&mut self,tabs:&mut Tabs,tree:&mut Tree){

        while self.applet.wait() {
            if let Some(msg) = self.receive.recv(){
                match msg {
                    // Handle the new file event ##############################################
                    lay_menubar::Message::New => {
                        tabs.begin();
                        tabs.add_resizable(&self.insert_tab(tabs.x()));
                        tabs.end();
                        println!("LayText~> New Tab (Count : \x1b[36m{}\x1b[0m)",self.tabcount);
                        // redraw the window to see the changes
                        self.window.redraw();
                    }
                    // Handle new Terminal Event #############################################
                    lay_menubar::Message::OpenTerm =>{
                        
                        // println!("LayText~> New Terminal (Count : \x1b[36m{}\x1b[0m)",self.tabcount);
                        // redraw the window to see the changes
                        self.window.redraw();
                    }
                    // ###########################################
                    lay_menubar::Message::OpenFolder =>{

                        let mut chooser = NativeFileChooser::new(
                                FileDialogType::BrowseDir
                            );
                        chooser.show();
                        self.put_dirs(chooser.filename(),tree);
                        
                    }
                    // Handle the save file event ############################################
                    lay_menubar::Message::Save => {

                        print!("LayText~> Saving ... ");
                        if !self.editors[&self.current_tab.load(Ordering::SeqCst)].is_defined {
                            let mut chooser = NativeFileChooser::new(
                                FileDialogType::BrowseSaveFile
                            );
                            chooser.show();
                            match chooser.filename().file_name(){
                                Some(_xyz) =>{
                                    println!("{:?}",chooser.filename());
                                    tabs.child(self.index_of(self.current_tab.load(Ordering::SeqCst))).unwrap().set_label((String::from(chooser.filename().file_name().unwrap().to_str().unwrap())+"    \u{2713}").as_str());
                                    if let Some(x) = self.editors.get_mut(&self.current_tab.load(Ordering::SeqCst)) {
                                        x.path = chooser.filename();
                                        x.is_defined=true;
                                        x.buffer().unwrap().save_file(chooser.filename());
                                        x.length=x.buffer().unwrap().length();
                                        x.is_saved = true;
                                        tabs.redraw();
                                    }
                                }
                                None => {
                                    dialog::alert(100,100,"Please give a filename");
                                }
                            }
                        }
                        else {
                            if let Some(x) = self.editors.get_mut(&self.current_tab.load(Ordering::SeqCst)) {                     
                                x.buffer().unwrap().save_file(x.path.clone());
                                x.length=x.buffer().unwrap().length();
                                x.is_saved = true;
                                println!("{:?}",x.path);
                            }
                        }
                    }
                    // Handle the open event ##################################################
                    lay_menubar::Message::Open =>{

                        print!("LayText~> Opening ... ");
                        let mut chooser = NativeFileChooser::new(
                                FileDialogType::BrowseFile
                            );
                        chooser.show();
                        match chooser.filename().file_name(){
                            Some(_xyz) => {
                                tabs.begin();
                                tabs.add_resizable(&self.insert_tab(tabs.x()));
                                tabs.end();
                                tabs.child(tabs.children()-1).unwrap().set_label((String::from(chooser.filename().file_name().unwrap().to_str().unwrap())+"    \u{2713}").as_str());
                                let t = self.map[(tabs.children()-1) as usize];
                                if let Some(x) = self.editors.get_mut(&t) {
                                    x.path = chooser.filename();
                                    x.is_defined=true;
                                    let mut buf = x.buffer().unwrap();
                                    buf.set_tab_distance(4);
                                    buf.load_file(chooser.filename());
                                    x.length=buf.length();
                                    x.is_saved = true;
                                    tabs.redraw();
                                    println!("{:?} length : {}",chooser.filename(),buf.length());
                                }
                            }
                            None => {
                                dialog::alert(100,100,"Please Select a file");
                            }
                        }
                    }
                    // Handle side bar toggle ###################################################
                    lay_menubar::Message::SideBar => {
                        // no idea ... do it later

                    }
                    //  Handle Close Event ######################################################
                    lay_menubar::Message::Close => {

                        if tabs.children() > 0 && self.map.contains(&self.current_tab.load(Ordering::SeqCst)){
                            let v = self.current_tab.load(Ordering::SeqCst);
                            let indx = self.index_of(v);
                            println!("LayText~> Closing Tab {}", (self.current_tab.load(Ordering::SeqCst) as i32));
                            self.editors.remove(&v);
                            self.map.remove(indx as usize);
                            tabs.remove_by_index(indx);
                            tabs.redraw();
                            app::redraw();
                        }
                    }
                    // else do nothing ##########################################################
                    _ => {}
                }
            }
            else {
                match app::event(){

                    Event::Focus => {
                        if self.map.len() > 0 && self.map.contains(&self.current_tab.load(Ordering::SeqCst)){
                            tabs.redraw();
                            let i = self.current_tab.load(Ordering::SeqCst);
                            let mut temp = tabs.child(self.index_of(i)).unwrap().label();
                            temp.pop();
                            if let Some(x) = self.editors.get_mut(&i) {
                                let len = x.length;
                                let buf_len = x.buffer().unwrap().length();
                                if len!=buf_len {
                                    temp+="\u{25aa}";
                                    x.is_saved = false;
                                    tabs.child(self.index_of(i)).unwrap().set_label(temp.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[&i].is_saved{
                                        temp+="\u{2713}";
                                        tabs.child(self.index_of(i)).unwrap().set_label(temp.as_str());
                                        tabs.redraw();
                                    }
                                }
                            }      
                        }
                    }
                    // Check for editted files and focus shifting ...
                    Event::Push => {
                        if self.map.len() > 0 && self.map.contains(&self.current_tab.load(Ordering::SeqCst)) {
                            tabs.redraw();
                            let i = self.current_tab.load(Ordering::SeqCst);
                            let mut temp = tabs.child(self.index_of(i)).unwrap().label();
                            temp.pop();
                            if let Some(x) = self.editors.get_mut(&i) {
                                let len = x.length;
                                let buf_len = x.buffer().unwrap().length();
                                if len!=buf_len {
                                    temp+="\u{25aa}";
                                    x.is_saved = false;
                                    tabs.child(self.index_of(i)).unwrap().set_label(temp.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[&i].is_saved{
                                        temp+="\u{2713}";
                                        tabs.child(self.index_of(i)).unwrap().set_label(temp.as_str());
                                        tabs.redraw();
                                    }
                                }
                            }
                        }
                    }
                    Event::KeyUp => {
                        if self.map.len() > 0 && self.map.contains(&self.current_tab.load(Ordering::SeqCst)) {                            
                            let i = self.current_tab.load(Ordering::SeqCst);
                            let mut temp = tabs.child(self.index_of(i)).unwrap().label();
                            temp.pop();
                            if let Some(x) = self.editors.get_mut(&i) {
                                let len = x.length;
                                let buf_len = x.buffer().unwrap().length();
                                if len!=buf_len {
                                    temp+="\u{25aa}";
                                    x.is_saved = false;
                                    tabs.child(self.index_of(i)).unwrap().set_label(temp.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[&i].is_saved{
                                        temp+="\u{2713}";
                                        tabs.child(self.index_of(i)).unwrap().set_label(temp.as_str());
                                        tabs.redraw();
                                    }
                                }
                            } 
                        }
                    }
                    _=>{

                    }
                }
            }
        }
    }
    //#########################################################################################
    fn put_dirs(&mut self,file:PathBuf,tree:&mut Tree){

        let mut image_open = PngImage::load("./src/open.png").unwrap();
        image_open.scale(20,20,true,true);
        tree.set_open_icon(Some(image_open));
        let mut image_close = PngImage::load("./src/close.png").unwrap();
        image_close.scale(20,20,true,true);
        tree.set_close_icon(Some(image_close));
        self.visit_dirs(file,tree);

    }
    
    fn visit_dirs(&mut self, dir:PathBuf,tree:&mut Tree){
        if dir.is_dir(){
            for entry in fs::read_dir(dir.as_path()).unwrap() {
                let entry = entry.unwrap();
                let path:PathBuf = entry.path();
                if path.is_dir() {
                    self.visit_dirs(path,tree);
                }
                else {
                    tree.add(path.to_str().unwrap());
                }
            }
        }
        else{
            tree.add(dir.to_str().unwrap());
        }
    }
    //#########################################################################################
    fn file_type(arg:PathBuf){


    }

    //#########################################################################################
    fn index_of(&mut self,get:i32) -> i32{
        let mut x = 0;
        for i in self.map.iter() {
            if *i==get{
                break;
            }
            x+=1;
        }
        x
    }
}
//#################################################################################################
