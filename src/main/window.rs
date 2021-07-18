use std::sync::atomic::{AtomicUsize,Ordering};
use std::sync::Arc;
// fltk's
use fltk::prelude::*;
use fltk::tree::*;
use fltk::app;
use fltk::dialog;
use fltk::window::*;
use fltk::menu::*;
use fltk::app::Scheme;
use fltk::button::Button;
use fltk::text::TextBuffer;
use fltk::group::{Tabs,Group,Tile,Scroll,VGrid,Pack,PackType};
use fltk::enums::{Color,Shortcut,FrameType,Event,Align};
use fltk::dialog::{FileChooser,FileChooserType,NativeFileChooser,NativeFileChooserType,FileDialogType};
//########################################################
pub mod lay_editor;
pub mod lay_menubar;
pub mod lay_version;
//###########################################################################################
pub struct LayText{
    tabcount:      i32,
    receive:       fltk::app::Receiver<lay_menubar::Message>,
    send:          fltk::app::Sender<lay_menubar::Message>,
    applet:        fltk::app::App,
    window:        OverlayWindow,
    current_tab:   Arc<AtomicUsize>,
    editors:       Vec<lay_editor::LayEditor>
}

impl LayText{
    //#########################################################################################
    pub fn new()-> Self{
        let (s,r) = fltk::app::channel::<lay_menubar::Message>();
        app::background(24,25,21);
        app::foreground(200,200,200);
        let mut window = OverlayWindow::new(0, 0, 900, 600, "Lay Text").center_screen();
        window.set_color(Color::from_rgb(24,25,21));
        window.make_resizable(true);

        Self {
            tabcount:      0,
            receive:       r,
            send:          s,
            applet:        fltk::app::App::default().with_scheme(app::Scheme::Gtk),
            window:        window,
            current_tab:   Arc::new(AtomicUsize::new(1)),
            editors:       vec![]
        }
    }
    //#########################################################################################
    pub fn layapp(&mut self){

        // Menu Bar Setting ################################
        let _menu = lay_menubar::LayMenuBar::new(&self.send);
        let _menu2 = lay_menubar::LayBarBottom::new(&self.send);
        
        let mut tile = Tile::new(0,35,890,542,"");
        
            let mut tree = Tree::new(0,35,10,542,"");
            tree.set_color(Color::from_rgb(24,25,21));
        
        let mut tabs = Tabs::new(10,35,890,542,"");
        tabs.set_label_color(Color::from_rgb(255,255,255));
        tabs.set_selection_color(Color::from_rgb(40,41,35));
        tabs.set_frame(FrameType::FlatBox);
        //tile.resizable(&tabs);
        // tile.resizable(&tree);
        tile.end();      

        self.window.resizable(&tile);
        self.window.end();
        self.window.show();
        // now finally launch it
        println!("Launching LayText(\x1b[37m{}\x1b[0m) ... \u{2191}",lay_version::VERSION);
        self.launch(&mut tabs,&mut tree);
        println!("LayText~> GoodBye ...");
    }
    //#########################################################################################
    pub fn insert_tab(&mut self,tabs:i32)-> Group {

        self.tabcount+=1;
        let tab = self.tabcount.clone(); 

        let mut group = Group::new(tabs,60,self.window.width()-10,self.window.height()-87," untitled    \u{2a2f}");
        // let mut but = Button::new( 88, 37 , 10, 10, "@1+");
        group.set_color(Color::from_rgb(255,255,255));
        group.set_label_size(11);
        group.begin();
            self.editors.push(lay_editor::LayEditor::new(fltk::text::TextBuffer::default(),self.window.width(),self.window.height(),tabs));
            group.resizable(&self.editors[(tab-1) as usize].editor);   
        group.end();
        
        // atomic variable for setting the current tab
        let cur = Arc::clone(&self.current_tab);
        group.handle(move |_,x| match x{
            Event::Focus => {
                cur.store(tab as usize ,Ordering::SeqCst);
                return true
            }
            Event::Push => {
                cur.store(tab as usize ,Ordering::SeqCst);        
                return true
            }
            _ => {
                false
            }
        });
        group
    }
    //#########################################################################################
    pub fn launch(&mut self,tabs:&mut Tabs,tile:&mut Tree){

        while  self.applet.wait() {
            if let Some(msg) = self.receive.recv(){
                match msg {
                    // Handle the new file event ##############################################
                    lay_menubar::Message::New => {
                        tabs.begin();
                        tabs.add_resizable(&self.insert_tab(tabs.x()));
                        tabs.end();
                        
                        println!("LayText~> New Tab (Count : \x1b[36m{}\x1b[0m)",self.tabcount);
                        
                        if self.tabcount>20 {
                            eprintln!(" ~> \x1b[36m Exceeded limit \u{26d4}\x1b[0m");
                        }
                        // redraw the window to see the changes
                        self.window.redraw();self.window.show();
                    }
                    // Handle the save file event ############################################
                    lay_menubar::Message::Save => {

                        print!("LayText~> Saving ... ");

                        if !self.editors[self.current_tab.load(Ordering::SeqCst)-1].is_defined {
                            let mut chooser = NativeFileChooser::new(
                                FileDialogType::BrowseSaveFile 
                            );
                            chooser.show();

                            match chooser.filename().file_name(){
                                Some(_xyz) =>{
                                    println!("{:?}",chooser.filename());
                                    tabs.child((self.current_tab.load(Ordering::SeqCst) as i32)-1).unwrap().set_label((String::from(chooser.filename().file_name().unwrap().to_str().unwrap())+"    *").as_str());
                                    self.editors[self.current_tab.load(Ordering::SeqCst)-1].path = chooser.filename();
                                    self.editors[self.current_tab.load(Ordering::SeqCst)-1].is_defined = true;
                                    let mut buf = self.editors[self.current_tab.load(Ordering::SeqCst)-1].buffer().unwrap();
                                    buf.save_file(chooser.filename());
                                    self.editors[self.current_tab.load(Ordering::SeqCst)-1].length=buf.length();
                                    self.editors[self.current_tab.load(Ordering::SeqCst)-1].is_saved = true;
                                    tabs.redraw();
                                }
                                None => {
                                    dialog::alert(100,100,"Please give a filename");
                                }
                            }
                        }
                        else {
                            let mut buf = self.editors[self.current_tab.load(Ordering::SeqCst)-1].buffer().unwrap();
                            buf.save_file(self.editors[self.current_tab.load(Ordering::SeqCst)-1].path.clone());
                            self.editors[self.current_tab.load(Ordering::SeqCst)-1].is_saved = true;
                            self.editors[self.current_tab.load(Ordering::SeqCst)-1].length=buf.length();
                            println!("{:?}",self.editors[self.current_tab.load(Ordering::SeqCst)-1].path);

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
                            Some(_xyz) =>{
                                tabs.begin();
                                tabs.add_resizable(&self.insert_tab(tabs.x()));
                                tabs.end();
                                tabs.child(tabs.children()-1).unwrap().set_label((String::from(chooser.filename().file_name().unwrap().to_str().unwrap())+"    \u{2a2f}").as_str());
                                let x = tabs.children() as usize;
                                let mut buf = self.editors[x-1].editor.buffer().unwrap();
                                self.editors[x-1].is_defined=true;
                                self.editors[x-1].path = chooser.filename();
                                buf.load_file(self.editors[x-1].path.clone());
                                self.editors[self.current_tab.load(Ordering::SeqCst)-1].length=buf.length();
                                self.editors[self.current_tab.load(Ordering::SeqCst)-1].is_saved = true;
                                tabs.redraw();
                                println!("{:?} length : {}",chooser.filename(),buf.length());
                            }
                            None => {
                                dialog::alert(100,100,"Please Select a file");
                            }
                        }
                    }
                    // else do nothing ##########################################################
                    _ => {}
                }
            }
            else {
                match app::event(){

                    Event::Focus => {
                        tabs.redraw();
                        for i in 0..self.tabcount {
                            if (i+1)==self.current_tab.load(Ordering::SeqCst) as i32{
                                let mut x = tabs.child(i).unwrap().label();
                                x.pop();
                                let len = self.editors[i as usize].length;
                                let buf_len = self.editors[i as usize].buffer().unwrap().length();

                                if len!=buf_len {
                                    x+="*";
                                    self.editors[i as usize].is_saved = false;
                                    tabs.child(i).unwrap().set_label(x.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[i as usize].is_saved{
                                        x+="\u{2a2f}";
                                        tabs.child(i).unwrap().set_label(x.as_str());
                                        tabs.redraw();
                                    }
                                }
                            }
                            else {
                                let mut x = tabs.child(i).unwrap().label();
                                x.pop();
                                let len = self.editors[i as usize].length;
                                let buf_len = self.editors[i as usize].buffer().unwrap().length();
                                
                                if len!=buf_len {
                                    x+="*";
                                    self.editors[i as usize].is_saved = false;
                                    tabs.child(i).unwrap().set_label(x.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[i as usize].is_saved{
                                        x+="\u{2a2f}";
                                        tabs.child(i).unwrap().set_label(x.as_str());
                                        tabs.redraw();
                                    }
                                }
                                
                            }
                        }
                    }
                    // Check for editted files and focus shifting ...
                    Event::Push => {
                        tabs.redraw();
                        for i in 0..self.tabcount {
                            if (i+1)==self.current_tab.load(Ordering::SeqCst) as i32{
                                let mut x = tabs.child(i).unwrap().label();
                                x.pop();
                                let len = self.editors[i as usize].length;
                                let buf_len = self.editors[i as usize].buffer().unwrap().length();
                                
                                if len!=buf_len {
                                    x+="*";
                                    self.editors[i as usize].is_saved = false;
                                    tabs.child(i).unwrap().set_label(x.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[i as usize].is_saved{
                                        x+="\u{2a2f}";
                                        tabs.child(i).unwrap().set_label(x.as_str());
                                        tabs.redraw();
                                    }
                                }   
                            }
                            else {
                                let mut x = tabs.child(i).unwrap().label();
                                x.pop();
                                let len = self.editors[i as usize].length;
                                let buf_len = self.editors[i as usize].buffer().unwrap().length();
                                
                                if len!=buf_len {
                                    x+="*";
                                    self.editors[i as usize].is_saved = false;
                                    tabs.child(i).unwrap().set_label(x.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[i as usize].is_saved{
                                        x+="\u{2a2f}";
                                        tabs.child(i).unwrap().set_label(x.as_str());
                                        tabs.redraw();
                                    }
                                }
                            }
                        }
                    }
                    Event::KeyUp => {
                        tabs.redraw();
                        for i in 0..self.tabcount {
                            if (i+1)==self.current_tab.load(Ordering::SeqCst) as i32{
                                let mut x = tabs.child(i).unwrap().label();
                                x.pop();
                                let len = self.editors[i as usize].length;
                                let buf_len = self.editors[i as usize].buffer().unwrap().length();
                                
                                if len!=buf_len {
                                    x+="*";
                                    self.editors[i as usize].is_saved = false;
                                    tabs.child(i).unwrap().set_label(x.as_str());
                                    tabs.redraw();
                                }
                                else {
                                    if len==buf_len && self.editors[i as usize].is_saved{
                                        x+="\u{2a2f}";
                                        tabs.child(i).unwrap().set_label(x.as_str());
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



}
//#################################################################################################
 
