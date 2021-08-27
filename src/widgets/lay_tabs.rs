use std::sync::Arc;
use std::sync::atomic::{AtomicI32,Ordering};

use fltk::prelude::*;
use fltk::enums::{Color,Align, Event, FrameType};
use fltk::group::{Group,Pack,PackType,Scroll,ScrollType};
use fltk::button::{Button};
use fltk::image::{SvgImage};
use fltk::app;
use fltk::enums;
use fltk::valuator::Scrollbar;

pub struct TabButton {
    grp: Group,
    but: Button,
}

impl TabButton {
    pub fn default() -> Self {

        let mut grp = Group::new(0, 0,100,15, None);
        let mut but = Button::new(grp.x() + 80, grp.y()+2, 10, 10,None);
        let mut image_cl = SvgImage::load("./src/close_main.svg").unwrap();

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
}

impl ClosableTab {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {

        let parent_grp = Group::new(x, y+10, w, h, None);

        let mut scrgr = Scroll::new(x+48, y+10,w-48,31, None);
        scrgr.set_color(Color::from_rgb(24,25,21));
        scrgr.set_scrollbar_size(1);
        scrgr.set_type(ScrollType::Horizontal);

        let mut pk = Pack::new(x+48, y+10, w-48, 30, None);
        pk.set_spacing(3);
        pk.set_type(PackType::Horizontal);
        pk.end();

        scrgr.end();

        // this the group containing text editor
        let mut grp = Group::new(x, y + 40, w, h, None);
        grp.set_color(Color::from_rgb(40,41,35));
        grp.set_frame(FrameType::FlatBox);
        grp.end();

        parent_grp.end();
        
        Self { grp, pk ,hscroll:scrgr ,active_tab: Arc::new(AtomicI32::new(1)) ,children:0 }
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
        but.grp.set_label(&label);
        but.but.clear_visible_focus();
        but.grp.handle({
            let self_grp = self.grp.clone();
            let mut curr_grp = grp.clone();
            let curr_tab = Arc::clone(&self.active_tab);
            let mut x = 0;
            let mut y = 0;
            move |w, ev| match ev {

                Event::Push => {
                    let coords = app::event_coords();
                    x = coords.0;
                    y = coords.1;
                    for child in 0..self_grp.children() {
                        // self_grp.child(child).unwrap().deactivate();
                        self_grp.child(child).unwrap().hide();
                    
                    }

                    let idx = self_grp.find(&curr_grp);
                    // curr_grp.activate();
                    curr_tab.store(idx, Ordering::SeqCst);
                    curr_grp.set_label_color(Color::from_rgb(255,255,255));
                    curr_grp.show();

                    true
                },
                Event::Drag => {
                    true
                },
                _ => false,
            }
        });

        self.pk.add(&but.grp);

        but.but.set_callback({
            let curr_grp = grp.clone();
            let mut self_grp = self.grp.clone();
            let mut self_pack = self.pk.clone();
            move |_| {
                let idx = self_grp.find(&curr_grp);
                println!("{}",idx );
                self_grp.remove_by_index(idx);
                self_pack.remove_by_index(idx);
                if let Some(mut grp) = self_grp.child(self_grp.children() - 1) {
                    grp.show();
                }
                app::redraw();
            }
        });
    }
}
//########################################################################
