use std::char;
use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

use grid::Grid;
use rustty::{
    Terminal, 
    Event,
    HasSize,
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult
};

use rustty::ui::{
    Dialog,
    Label,
    StdButton
};

struct Preset {
    name: String,
    path: String,
}

pub fn load(grid: &mut Grid, term: &mut Terminal) {
    let (t_width, t_height) = term.size();
    let mut presets: Vec<Preset> = Vec::new();
    let mut ui = create_load_ui(50, t_height - t_height/3 + 2, &mut presets);
    ui.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));

    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ui.result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                Some(ButtonResult::Custom(i)) => {
                    load_preset(&presets[i as usize -1], grid);
                    break 'main;
                },
                _                      => {}
            }
        }

        ui.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn load_preset(p: &Preset, grid: &mut Grid) {
    let mut file = match File::open(Path::new(&p.path)) {
        Err(why) => panic!("Error loading preset {}: {}", &p.path,
                           Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Error reading {}: {}", &p.path,
                           Error::description(&why)),
        Ok(_) => panic!("{}", s)
    }
}

pub fn save(grid: &mut Grid, term: &mut Terminal) {
    let mut ui = create_save_ui(40, 10);
}

fn create_load_ui(width: usize, height: usize, presets: &mut Vec<Preset>) 
    -> Dialog {

    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let paths = fs::read_dir(Path::new("./presets")).unwrap();

    let mut i: u32 = 1;
    for path in paths {
        if i as usize >= height-1 || i > 9 {
            panic!(format!("Sorry! Only maximum 10 presets allowed. Either \
                            increase height of terminal or delete some presets
                            within the 'preset' folder"));
        }
        let path = path.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let mut btn = StdButton::new(&format!("{}: {}", i, name), 
                                         (i + 48) as u8 as char, 
                                         ButtonResult::Custom(i as i32));
            btn.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top,
                     (2,i as usize));
            dlg.add_button(btn);
            presets.push( Preset {
                name: name.to_string(),
                path: path.to_str().unwrap().to_string()});
            i += 1;
        }
    }
    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (1,1));
    dlg.add_button(quit);
    dlg
}

fn create_save_ui(width: usize, height: usize) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    dlg
}
