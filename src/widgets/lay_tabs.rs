// std includes #######################
use std::sync::Arc;
use std::sync::atomic::{AtomicI32,Ordering};
use core::fmt::Debug;
// fltk includes ######################
use fltk::prelude::*;
use fltk::enums::{Color,Align, Event, FrameType,Shortcut};
use fltk::group::{Group,Pack,PackType,Scroll,ScrollType,Row};
use fltk::button::{Button};
use fltk::image::{SvgImage};
use fltk::app;
use fltk::enums;
use fltk::table::*;
use fltk::valuator::Scrollbar;
use fltk::widget::{Widget};
use fltk;
use std::sync::mpsc::channel;
//#####################################
use crate::messg::Message;


pub struct TabButton {
    grp: Group,
    but: Button,
}

impl TabButton {
    pub fn default() -> Self {

        let mut grp = Group::new(0, 0,150,15, None);
        let mut but = Button::new(grp.x() + 130, grp.y()+2, 10, 10,None);
        let mut image_cl = SvgImage::load("./src/Icon/close_main.svg").unwrap();

        image_cl.scale(9,9,true,true);
        but.set_image(Some(image_cl));
        but.set_frame(FrameType::NoBox);
        grp.end();
        grp.set_frame(FrameType::FlatBox);
        grp.set_label_size(12);
        grp.set_label_color(Color::from_rgb(255,255,255));
        grp.set_color(Color::from_rgb(40,41,35));
        Self { grp, but }
    }
}
pub struct ClosableTab {
    pub grp: Group,
    pub pk: Pack,
    pub hscroll: Scroll,
    pub active_tab:Arc<AtomicI32>,
    pub children:i32,
    pub parent_grp:Group,
    pub s: app::Sender<Message>,
    pub c: Vec<Group>
}

impl ClosableTab {
    pub fn new(x: i32, y: i32, w: i32, h: i32,s: &app::Sender<Message>) -> Self {
        //######################################################################
        let parent_grp = Group::new(x, y+13, w, h, None);

        // let mut rowgr = Row::new(x+48, y+10,w-48,31, None);
        let mut scrgr = Scroll::new(x+48, y+13,w-48,30, None);
        scrgr.set_color(Color::from_rgb(24,25,21));
        scrgr.set_scrollbar_size(1);
        scrgr.set_type(ScrollType::None);
        scrgr.hscrollbar().hide();
        
        let mut pk = Pack::new(x+48, y+13, w-48, 30, None);
        pk.set_spacing(3);
        pk.set_type(PackType::Horizontal);
        pk.end();

        scrgr.end();
        
        let mut gp = Group::new(5,42,15,15,None);
        let mut prev = Button::new(5,42,15,15,None);
        let mut image = SvgImage::load("./src/Icon/mono-navigator-prev.svg").unwrap();
        image.scale(13,13,true,true);
        prev.set_image(Some(image));
        prev.set_frame(FrameType::NoBox);
        prev.set_color(Color::from_rgb(180,180,0));
        prev.set_frame(enums::FrameType::NoBox);
        prev.set_tooltip("Scroll Tabs");
        prev.clear_visible_focus();
        prev.handle({
            let mut self_grp = scrgr.clone();
            move |w, ev| match ev {
                enums::Event::Push => {
                    // println!("xpos: {} : Maximum: {} : Minimum: {}",self_grp.xposition(),self_grp.hscrollbar().maximum(),self_grp.hscrollbar().minimum());
                    if self_grp.xposition()!=(self_grp.hscrollbar().minimum()) as i32{
                        self_grp.scroll_to(self_grp.xposition()-15,self_grp.yposition());
                    }
                    true
                }, 
                enums::Event::MouseWheel => {
                    // println!("xpos: {} : Maximum: {} : Minimum: {}",self_grp.xposition(),self_grp.hscrollbar().maximum(),self_grp.hscrollbar().minimum());
                    match app::event_dy(){

                        app::MouseWheel::Up => {
                            if self_grp.xposition()!=(self_grp.hscrollbar().maximum())as i32{
                                self_grp.scroll_to(self_grp.xposition()+15,self_grp.yposition());
                            }
                        }
                        app::MouseWheel::Down => {
                            if self_grp.xposition()!=(self_grp.hscrollbar().minimum()) as i32{
                                self_grp.scroll_to(self_grp.xposition()-15,self_grp.yposition());
                            }
                        }
                        _=>{  }
                    }
                    true
                },
                _ => false,
            }
        });
        
        let mut next = Button::new(25,42,15,15,None);
        let mut image = SvgImage::load("./src/Icon/mono-navigator-next.svg").unwrap();
        image.scale(13,13,true,true);
        next.set_image(Some(image));
        next.set_frame(FrameType::NoBox);
        next.set_color(Color::from_rgb(180,180,0));
        next.set_frame(enums::FrameType::NoBox);
        next.set_tooltip("Scroll Tabs");
        next.clear_visible_focus();
        next.handle({
            let mut self_grp = scrgr.clone();
            move |w, ev| match ev {
                enums::Event::Push => {

                    // println!("xpos: {} : Maximum: {} : Minimum: {}",self_grp.xposition(),self_grp.hscrollbar().maximum(),self_grp.hscrollbar().minimum());

                    if (self_grp.xposition()!=(self_grp.hscrollbar().maximum())as i32){
                        self_grp.scroll_to(self_grp.xposition()+15,self_grp.yposition());
                    }
                    true
                },
                _ => false,
            }
        });
        gp.end();
        gp.make_resizable(false);
        // rowgr.end();
        // this the group containing text editor
        let mut grp = Group::new(x, y + 40, w, h-5, None);
        grp.set_color(Color::from_rgb(40,41,35));
        grp.set_frame(FrameType::FlatBox);
        grp.end();

        parent_grp.end();
        //######################################################################
        // Ok this is Finally the End of creating a closeable Tab Group
        //######################################################################
        Self { 
            parent_grp,
            grp, 
            pk ,
            hscroll:scrgr ,
            active_tab: Arc::new(AtomicI32::new(0)) ,
            children:0,
            s:*s ,
            c: Vec::new(),

        }
    }

    pub fn add(&mut self, grp: &mut Group) {
        grp.resize(self.grp.x(), self.grp.y(), self.grp.w(), self.grp.h());
        grp.show();

        // if self.grp.children() == 0 {
        //     grp.show();
        // } else {
        //     grp.hide();
        // }
        
        self.grp.add(grp);
        let children=self.grp.children();
        self.children=children;
        self.active_tab.store(children-1,Ordering::SeqCst);
        let label = grp.label();
        grp.set_label("");
        let mut but = TabButton::default();
        but.grp.set_align(Align::Left | Align::Inside);
        but.grp.set_label("  untitled");
        but.but.clear_visible_focus();

        but.grp.handle({

            let self_grp = self.grp.clone();
            
            let mut curr_grp = grp.clone();

            let curr_tab = Arc::clone(&self.active_tab);
            
            let mut x = 0;
            
            let mut y = 0;
            move |w, ev| match ev {
                // handle the events on the tabs
                Event::Push => {
                    let coords = app::event_coords();
                    x = coords.0;
                    y = coords.1;
                    for child in 0..self_grp.children() {
                        self_grp.child(child).unwrap().hide();
                    }

                    let idx = self_grp.find(&curr_grp);
                    curr_tab.store(idx, Ordering::SeqCst);
                    curr_grp.set_label_color(Color::from_rgb(255,255,255));
                    w.set_color(Color::from_rgb(40,41,35));
                    curr_grp.show();
                    app::redraw();

                    true
                },
                Event::Enter =>{
                    if !curr_grp.visible() {
                        w.set_color(Color::from_rgb(30,31,25));
                        app::redraw();
                    }
                    true
                },
                Event::Leave =>{
                    if !curr_grp.visible() {
                        w.set_color(Color::from_rgb(24,25,21));
                        app::redraw();
                    }
                    true
                },
                _ => false,
            }
        });

        self.pk.add(&but.grp);
        self.c.push(but.grp);

        but.but.set_callback({
            let curr_grp = grp.clone();
            let curr_tab = Arc::clone(&self.active_tab);
            let mut self_grp = self.grp.clone();
            let mut self_pack = self.pk.clone();
            let x = self.s.clone();
            move |_| {
                let idx = self_grp.find(&curr_grp);
                x.send(Message::Closed(idx));
                self_grp.remove_by_index(idx);
                self_pack.remove_by_index(idx);
                if let Some(mut grp) = self_grp.child(self_grp.children() - 1) {
                    curr_tab.store(self_grp.children() - 1, Ordering::SeqCst);
                    grp.show();
                }
                app::redraw();
            }
        });
    }
}
//########################################################################
