// color_test_string.mod.rs

use crate::html_encoded_push;
//use crate::web_sys_mod::debug_write;
use crate::web_sys_mod::HtmlEncoded;

use regex::Regex;

/// color the test string
pub fn color_test_string(regex_text: &str, test_string: &str) -> HtmlEncoded {
    let mut html = HtmlEncoded::new();
    // early return for errors
    let rgx = match Regex::new(regex_text) {
        Ok(r) => r,
        Err(_e) => {
            html_encoded_push!(html, "{}", test_string);
            return html;
        }
    };

    let colors = colors_with_order();
    /*
        extern crate rand;
    use rand::{Rng, StdRng};
    let mut rng = StdRng::new().unwrap();
    rng.shuffle(&mut x);
    */
    let mut cursor_color = 0;
    let mut caps = vec![];
    //first fill and then sort, start and end together
    for c in rgx.captures_iter(test_string) {
        for i in 0..c.len() {
            caps.push(('s', c.get(i).unwrap().start()));
            caps.push(('e', c.get(i).unwrap().end()));
        }
    }
    caps.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //debug_write(&format!("{:?}", caps));
    let mut cursor_pos = 0;
    for c in caps.iter() {
        html_encoded_push!(html, "{}", &test_string[cursor_pos..c.1]);
        if c.0 == 's' {
            html_encoded_push!(html, r#"<span style="background-color:{}">"#, colors[cursor_color].0);
        } else {
            // c.0== 'e'
            html_encoded_push!(html, r#"</span>"#);
        }
        cursor_pos = c.1;
        cursor_color += 1;
        if cursor_color >= colors.len() {
            cursor_color = 0;
        }
    }
    html_encoded_push!(html, "{}", &test_string[cursor_pos..]);
    // return
    html
}

/// dark colors shuffled
fn colors_with_order() -> Vec<(&'static str, &'static str)> {
    vec![
        ("#547053", "Evergreen"),
        ("#29304e", "Midnight Blue"),
        ("#80444c", "Deep Purple"),
        ("#004953", "Poker Green"),
        ("#11574a", "Field Maple"),
        ("#004400", "Aura Orange"),
        ("#5c5337", "Polished Brown"),
        ("#553f2d", "Blackened Brown"),
        ("#3b2820", "Tuk Tuk"),
        ("#985538", "Toffee"),
        ("#363737", "Dusty Olive"),
        ("#220a0a", "Rustic Red"),
        ("#742802", "Dark Brown"),
        ("#302621", "Black Chocolate"),
        ("#3b3c36", "Black Pearl"),
        ("#184343", "Very Blue"),
        ("#2a2b2d", "Tarmac"),
        ("#11887b", "Kabocha Green"),
        ("#820000", "Heavy Red"),
        ("#35654d", "Racing Green"),
        ("#214559", "Blue Charcoal"),
        ("#014600", "Smoke Pine"),
        ("#25342b", "Lunar Green"),
        ("#442200", "Brown Stone"),
        ("#ba160c", "Winter Sunset"),
        ("#900020", "Burnt Maroon"),
        ("#b0306a", "Alien Purple"),
        ("#af2f0d", "Celestial Pink"),
        ("#980036", "Red Bean"),
        ("#673a3f", "Purple Stone"),
        ("#34414e", "Night Blue"),
        ("#4a0100", "Pink Raspberry"),
        ("#5c4450", "Purple Brown"),
        ("#716e61", "Heavy Charcoal"),
        ("#36013f", "Medium Taupe"),
        ("#2a2a35", "Polished Steel"),
        ("#1e272c", "Coffee Bean"),
        ("#4e5541", "Night Sky"),
        ("#ca6636", "Capital Yellow"),
        ("#696006", "Green Ink"),
        ("#937a62", "Marsh"),
        ("#6f7755", "Elm Green"),
        ("#800000", "Old Mahagony"),
        ("#391285", "Reflecting Pond"),
        ("#062e03", "Woodland Grass"),
        ("#50574c", "Very Dark Brown"),
        ("#5a5348", "Thyme"),
        ("#565350", "Holly"),
        ("#420303", "Dark Red"),
        ("#d1001c", "Brick Orange"),
        ("#0a0502", "Pot Black"),
        ("#495e35", "Green Brown"),
        ("#c65102", "International Orange"),
        ("#333333", "Dark Grey"),
        ("#23191e", "Liquorice"),
        ("#7f7053", "Hardwood"),
        ("#35063e", "Dark Strawberry"),
        ("#280137", "Pickled Beet"),
        ("#490648", "Brownish Purple"),
        ("#0f3b57", "Dark Navy Blue"),
        ("#373e02", "Dill"),
        ("#3b302f", "Black"),
        ("#2b0202", "Void"),
        ("#7f4330", "Dark Orange"),
        ("#404854", "Sooty Willow Bamboo"),
        ("#840000", "Deep Maroon"),
        ("#2b6867", "Medieval Blue"),
        ("#00626f", "Blue Opal"),
        ("#203e4a", "Tiber"),
        ("#646356", "Flint"),
        ("#3d6c54", "Very Dark Green"),
        ("#040348", "Pixie Powder"),
        ("#4e5552", "Carbon"),
        ("#48412b", "Cape Cod"),
        ("#00022e", "Dark Water"),
        ("#d90166", "Dark Pink"),
        ("#c14a09", "Copper Orange"),
        ("#0000aa", "Very Dark Blue"),
        ("#9c004a", "Dark Hot Pink"),
        ("#044a05", "Midnight Green"),
        ("#5d5242", "Black Olive"),
        ("#9e1212", "Maroon"),
        ("#3e6257", "Toad King"),
        ("#3a181a", "Rusty Red"),
        ("#754600", "Grey Brown"),
        ("#674c47", "Midnight Purple"),
        ("#593c39", "Chestnut"),
        ("#76424e", "Dark Purple"),
        ("#cb416b", "Rich Maroon"),
        ("#033500", "Dark Olive"),
        ("#3d0c02", "Red Ink"),
        ("#573b2a", "Alien"),
        ("#4f1507", "Flat Brown"),
        ("#4d233d", "Purple Basil"),
        ("#410200", "Earth Brown"),
        ("#b4262a", "Blood Orange"),
        ("#2a293e", "Magnetic Blue"),
        ("#755139", "Tree House"),
        ("#605467", "Antique Brown"),
        ("#4d4b3a", "Space Grey"),
        ("#362d26", "Cynical Black"),
        ("#490206", "Deep Brown"),
        ("#80884e", "Garden Green"),
        ("#341c02", "Dark Chocolate"),
        ("#415764", "Brick Grey"),
    ]
}
