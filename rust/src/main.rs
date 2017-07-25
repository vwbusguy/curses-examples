extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho};
use std::collections::HashMap;
// Bring in lib.rs structs
mod lib;
use lib::{Point,WinRef};

fn main() {
    // Define screen boundaries
    let min_ref = Point{x:1,y:1};
    let max_ref = Point{x:17,y:19};

    // Define initial cursor position
    let cursor = Point{x:5,y:5};

    // Create initial HashMaps for tracking screen objects and user phrases
    let mut draw_list: HashMap<Point,WinRef> = HashMap::new();
    let mut phrase_list: HashMap<String,String> = HashMap::new();

    // Identify and populate person object points
    phrase_list.insert("Rick".to_string(),"Rick: Turn that banjo mularkey off!           ".to_string());
    phrase_list.insert("Nico".to_string(),"Nico: I think I've found the solution!        ".to_string());
    phrase_list.insert("Samuel".to_string(),"Samuel: I found a new zsh git tool!           ".to_string());
    phrase_list.insert("Mark".to_string(),"Mark: Ohayogazaimasu                          ".to_string());
    phrase_list.insert("Scott".to_string(),"Scott: Necesito mas burritos!!                ".to_string());
    draw_list.insert(Point{x:11,y:12},WinRef{ch:'o',rtype:"Scott".to_string()});
    draw_list.insert(Point{x:11,y:15},WinRef{ch:'o',rtype:"Rick".to_string()});
    draw_list.insert(Point{x:15,y:2},WinRef{ch:'o',rtype:"Mark".to_string()});
    draw_list.insert(Point{x:13,y:2},WinRef{ch:'o',rtype:"Samuel".to_string()});
    draw_list.insert(Point{x:9,y:2},WinRef{ch:'o',rtype:"Nico".to_string()});

    let window = initscr();
    
    // Populate lists of points for furniture character plots, grouped by character
    let furn_up_list = vec![
        Point{x:14,y:2},
        Point{x:14,y:3},
        Point{x:14,y:4},
        Point{x:14,y:2},
        Point{x:14,y:3},
        Point{x:14,y:4},
        Point{x:17,y:2},
        Point{x:17,y:3},
        Point{x:17,y:4},
        Point{x:17,y:5},
        Point{x:10,y:10},
        Point{x:10,y:11},
        Point{x:10,y:12},
        Point{x:14,y:10},
        Point{x:14,y:11},
        Point{x:14,y:12},
        Point{x:10,y:14},
        Point{x:10,y:15},
        Point{x:10,y:16},
    ];
    let furn_corn_list = vec![
        Point{x:14,y:1},
        Point{x:17,y:1},
        Point{x:14,y:5},
        Point{x:10,y:9},
        Point{x:10,y:13},
        Point{x:14,y:13},
        Point{x:10,y:17}
    ];
    let furn_hz_list = vec![
        Point{x:13,y:1},
        Point{x:12,y:1},
        Point{x:11,y:1},
        Point{x:10,y:1},
        Point{x:9,y:1},
        Point{x:8,y:1},
        Point{x:15,y:1},
        Point{x:16,y:1},
        Point{x:11,y:9},
        Point{x:9,y:9},
        Point{x:8,y:9},
        Point{x:7,y:9},
        Point{x:11,y:13},
        Point{x:12,y:13},
        Point{x:13,y:13},
        Point{x:9,y:17},
        Point{x:8,y:17},
        Point{x:7,y:17},
        Point{x:11,y:17},
        Point{x:12,y:17},
        Point{x:13,y:17},
    ];
    for fu in furn_up_list{
        draw_list.insert(fu,WinRef{ch:'|',rtype:"furn".to_string()});
    }
    for fc in furn_corn_list{
        draw_list.insert(fc,WinRef{ch:'+',rtype:"furn".to_string()});
    }
    for fz in furn_hz_list{
        draw_list.insert(fz,WinRef{ch:'-',rtype:"furn".to_string()});
    }
    for (pref,wref) in &draw_list{
        window.mvaddch(pref.y,pref.x,wref.ch);
    }

    // Give initial welcome message
    window.mvprintw(0,0,"Meet the SysDev team! (Type q or Del to exit)");

    // Put the cursor in the initial cursor position
    window.mv(cursor.y,cursor.x);

    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            // On Delete or 'q' key, break and end the program
            Some(Input::KeyDC) | Some( Input::Character('q')) => break,

            // For arrow keys, attempt to move the cursor or interact
            Some(Input::KeyUp) => {
                // Populate references for current position and where the cursor may end up
                let cur_ref = window.get_cur_yx();
                let cur_pt = Point{x:cur_ref.1,y:cur_ref.0};
                let ref_y = cur_pt.y - 1;
                let ref_pt = Point{x:cur_pt.x,y:ref_y};
                // If move isn't out of bounds or into an object, move the cursor
                if ref_y >= min_ref.y && !draw_list.contains_key(&ref_pt){
                    window.mv(ref_y,cur_pt.x);
                }

                // If the move would hit a person object, print the person's phrase and don't move
                if draw_list.contains_key(&ref_pt){
                    let ref dtype = draw_list.get(&ref_pt).unwrap().rtype;
                    if dtype != "furn" && phrase_list.contains_key(dtype){
                        window.mvprintw(0,0,phrase_list.get(dtype).unwrap());
                    }
                    window.mv(cur_pt.y,cur_pt.x);
                }                
            },
            Some(Input::KeyDown) => {
                let cur_ref = window.get_cur_yx();
                let cur_pt = Point{x:cur_ref.1,y:cur_ref.0};
                let ref_y = cur_pt.y + 1;
                let ref_pt = Point{x:cur_pt.x,y:ref_y};
                if ref_y <= max_ref.y && !draw_list.contains_key(&ref_pt){
                    window.mv(ref_y,cur_pt.x);
                }
                if draw_list.contains_key(&ref_pt){
                    let ref dtype = draw_list.get(&ref_pt).unwrap().rtype;
                    if dtype != "furn" && phrase_list.contains_key(dtype){
                        window.mvprintw(0,0,phrase_list.get(dtype).unwrap());
                    }
                    window.mv(cur_pt.y,cur_pt.x);
                }
            },
            Some(Input::KeyLeft) => {
                let cur_ref = window.get_cur_yx();
                let cur_pt = Point{x:cur_ref.1,y:cur_ref.0};
                let ref_x = cur_pt.x - 1;
                let ref_pt = Point{x:ref_x,y:cur_pt.y};
                if ref_x >= min_ref.x && !draw_list.contains_key(&ref_pt){
                    window.mv(cur_pt.y,ref_x);
                }
                if draw_list.contains_key(&ref_pt){
                    let ref dtype = draw_list.get(&ref_pt).unwrap().rtype;
                    if dtype != "furn" && phrase_list.contains_key(dtype){
                        window.mvprintw(0,0,phrase_list.get(dtype).unwrap());
                    }
                    window.mv(cur_pt.y,cur_pt.x);
                }
            },            
            Some(Input::KeyRight) => {
                let cur_ref = window.get_cur_yx();
                let cur_pt = Point{x:cur_ref.1,y:cur_ref.0};
                let ref_x = cur_pt.x + 1;
                let ref_pt = Point{x:ref_x,y:cur_pt.y};
                if ref_x <= max_ref.x && !draw_list.contains_key(&ref_pt){
                    window.mv(cur_pt.y,ref_x);
                }
                if draw_list.contains_key(&ref_pt){
                    let ref dtype = draw_list.get(&ref_pt).unwrap().rtype;
                    if dtype != "furn" && phrase_list.contains_key(dtype){
                        window.mvprintw(0,0,phrase_list.get(dtype).unwrap());
                    }
                    window.mv(cur_pt.y,cur_pt.x);
                }
            },
            Some(input) => {
                // If some other key is pressed, show the welcome message again
                let cur_ref = window.get_cur_yx();
                window.mvprintw(0,0,"Meet the SysDev team! (Type q or Del to exit)");
                window.mv(cur_ref.0,cur_ref.1);
            },
            None => ()
        }
        window.refresh();
    }
    endwin();
}
