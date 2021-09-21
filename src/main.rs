use fltk::{app, button::Button,input, enums::{CallbackTrigger, Color, Key, Shortcut}, frame::Frame, group::{Pack, PackType}, prelude::*, window::{self, Window}};
use fltk_flex::{Flex, FlexType};
use std::{borrow::{Borrow, BorrowMut}, ops::{Deref}};
use std::env;
use std::fs;


#[derive(Debug, Copy, Clone)]
enum Message {
    Menu(u32),
    FlashCard(u32),
    Add(u32)
}

struct FlashCard {
    text_fr: String,
    text_eng: String,
    frame: Frame,
    btn: Button
}

impl FlashCard {
    pub fn new( english_text: &str , french_text:&str) -> FlashCard {
        let frame = Frame::default();
        let btn = Button::default();
        FlashCard
        { 
            text_eng: String::from(english_text), text_fr: 
            String::from(french_text), 
            frame: frame,
            btn: btn
        }
    }
    pub fn create(&self, english_text: &str , french_text:&str) -> FlashCard {
        let frame = Frame::default()
            .with_pos(10, 360)
            .with_size(200, 350);
        let mut btn_frame = Button::default()
            .with_label("Label")
            .size_of(&frame)
            .above_of(&frame, 0);
        btn_frame.set_color(Color::Red.inactive());
        btn_frame.set_label_color(Color::Black);
        FlashCard
        { 
            text_eng: String::from(english_text), text_fr: 
            String::from(french_text), 
            frame: frame, 
            btn: btn_frame 
        }
    }

    pub fn to_written_form(self) -> String {
        format!("[{};{}]", self.text_eng, self.text_fr)        
    }

    pub fn from_written_form(&self, inp: String) ->  FlashCard{
        //format!("[{};{}]", self.text_eng, self.text_fr)
        self.create("", "")
    }

    pub fn set_bg_color(&mut self, clr_bg: Color) {
        self.btn.set_color(clr_bg);
    }
}

fn pos_by_int(screen: i32, widget_w: i32, i: i32) -> i32 {
    screen/2-widget_w/2+(i*150)
}

fn toggle_display(show: bool, btn_vec:&mut Vec<Button>) {
    for btn in btn_vec.iter_mut() {
        if show {
            btn.show();
        } else {
            btn.hide();
        }
    }
}

fn main() {

    // --- INIT MAIN DATA --- //
    let filename = "cards.txt";

    let win_w = 1500;
    let win_h = 1000;

    let btn_w = 300;
    let btn_h = 120;

    let inp_w = 500;
    let inp_h = 120;

    // --- INIT DATA LISTS ---
    let mut btn_vec_main:Vec<Button> = Vec::new();
    
    let (s, r) = app::channel::<Message>();

    let mut app = app::App::default();
    let mut window_main = window::Window::default()
        .with_size(win_w, win_h)
        .with_label("French Flashcards")
        .center_screen();
    
    // --- INIT MAIN MENU SCREEN --- //
    let mut btn_play = Button::new(pos_by_int(win_w,btn_w, 0),pos_by_int(win_h,btn_h, -1), btn_w, btn_h, "PLAY");
    btn_play.set_color(Color::Green.inactive());
    let mut btn_edit = Button::new(pos_by_int(win_w,btn_w, 0),pos_by_int(win_h,btn_h, 0), btn_w, btn_h, "EDIT CARDS");
    btn_edit.set_color(Color::Blue.inactive());
    let mut btn_add = Button::new(pos_by_int(win_w,btn_w, 0),pos_by_int(win_h,btn_h, 1), btn_w, btn_h, "ADD CARD");
    btn_add.set_color(Color::Blue.inactive());
    let mut btn_quit = Button::new(pos_by_int(win_w,btn_w, 0),pos_by_int(win_h,btn_h, 2), btn_w, btn_h, "End");
    btn_quit.set_color(Color::Red.inactive());
    btn_vec_main.push(btn_play);
    btn_vec_main.push(btn_edit);
    btn_vec_main.push(btn_add);
    btn_vec_main.push(btn_quit);

    
    // --- INIT ADD NEW CARD SCREEN --- //
    let mut input_eng = input::Input::new(pos_by_int(win_w,inp_w, 0 ), pos_by_int(win_h,inp_h, -1),inp_w, inp_h, "Please enter your inp");
    input_eng.set_trigger(CallbackTrigger::Changed);
    let mut input_fr = input::Input::new(pos_by_int(win_w,inp_w, 0 ), pos_by_int(win_h,inp_h, 0),inp_w, inp_h, "Please enter your inp");
    input_fr.set_trigger(CallbackTrigger::Changed);
    let mut btn_addnew = Button::new(pos_by_int(win_w,btn_w, 0),pos_by_int(win_h,btn_h, 1), btn_w, btn_h, "ADD");
    btn_addnew.set_color(Color::DarkGreen.inactive());
    btn_addnew.hide();
    input_eng.hide();
    input_fr.hide();

    
    // --- SETUP EVENT HANDLERS --- //
    for i in 1..5 {
        btn_vec_main.get_mut(i-1).unwrap().emit(s, Message::Menu(i as u32)); //wtf is this
    }
    
    btn_addnew.emit(s, Message::Add(1));
    input_eng.emit(s, Message::Add(2));
    input_fr.emit(s, Message::Add(3));
    
    window_main.end();
    window_main.show();

    while app.wait() {
        if let Some(val) = r.recv() {
            match val {
                Message::Menu(num) => {
                    toggle_display(false, btn_vec_main.borrow_mut());
                    match num {
                        1 => {
                            println!("1");
                        },
                        2 => {
                            println!("2");
                        },
                        3 => {
                            println!("3");

                            btn_addnew.show();
                            input_eng.show();
                            input_fr.show();
                        },
                        _ => {
                            println!("_");
                            app.quit();
                        }
                    }
                },
                Message::Add(num) => {
                    match num {
                        1 => {
                            println!("Adding");
                            let se: String =  input_eng.value().parse().unwrap();
                            let sf: String =  input_fr.value().parse().unwrap();
                            println!("English: {}, French: {}",se, sf);
                            
                            let fc = FlashCard::new(se.as_str(), sf.as_str());
                            
                            //TODO fix
                            fs::write(filename, fc.to_written_form().as_str())
                                .expect("Unable to write file");

                            toggle_display(true, btn_vec_main.borrow_mut());
                            btn_addnew.hide();
                            input_eng.hide();
                            input_fr.hide();
                        },
                        2 => {
                            let s: String =  input_eng.value().parse().unwrap();
                            println!("English: {}", s);
                        },
                        3 => {
                            let s: String =  input_fr.value().parse().unwrap();
                            println!("French: {}", s);
                        },
                        _ => {}
                    }
                }
                _ => println!("2")
            }
        }
    }

    app.run().unwrap(); //TODO fix this
}
