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
            html_encoded_push!(
                html,
                r#"<span style="font-weight: bold;background-color:{}">"#,
                colors[cursor_color].0
            );
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

fn colors_with_order() -> Vec<(&'static str, &'static str)> {
    vec![
        ("#214559", "Blue Charcoal"),
        ("#014600", "Smoke Pine"),
        ("#754600", "Grey Brown"),
        ("#00022e", "Dark Water"),
        ("#2b6867", "Medieval Blue"),
        ("#29304e", "Midnight Blue"),
        ("#302621", "Black Chocolate"),
        ("#34414e", "Night Blue"),
        ("#040348", "Pixie Powder"),
        ("#391285", "Reflecting Pond"),
        ("#203e4a", "Tiber"),
        ("#184343", "Very Blue"),
        ("#0000aa", "Very Dark Blue"),
        ("#000133", "Dark Green"),
        ("#033500", "Dark Olive"),
        ("#373e02", "Dill"),
        ("#6f7755", "Elm Green"),
        ("#547053", "Evergreen"),
        ("#2a293e", "Magnetic Blue"),
        ("#11574a", "Field Maple"),
        ("#495e35", "Green Brown"),
        ("#696006", "Green Ink"),
        ("#11887b", "Kabocha Green"),
        ("#044a05", "Midnight Green"),
        ("#004953", "Poker Green"),
        ("#35654d", "Racing Green"),
        ("#3e6257", "Toad King"),
        ("#3d6c54", "Very Dark Green"),
        ("#062e03", "Woodland Grass"),
        ("#004400", "Aura Orange"),
        ("#b4262a", "Blood Orange"),
        ("#00626f", "Blue Opal"),
        ("#d1001c", "Brick Orange"),
        ("#80884e", "Garden Green"),
        ("#c14a09", "Copper Orange"),
        ("#7f4330", "Dark Orange"),
        ("#c65102", "International Orange"),
        ("#ba160c", "Winter Sunset"),
        ("#ca6636", "Capital Yellow"),
        ("#d5b60a", "Burgundy"),
        ("#900020", "Burnt Maroon"),
        ("#420303", "Dark Red"),
        ("#840000", "Deep Maroon"),
        ("#820000", "Heavy Red"),
        ("#9e1212", "Maroon"),
        ("#800000", "Old Mahagony"),
        ("#4a0100", "Pink Raspberry"),
        ("#980036", "Red Bean"),
        ("#3d0c02", "Red Ink"),
        ("#220a0a", "Rustic Red"),
        ("#3a181a", "Rusty Red"),
        ("#af2f0d", "Celestial Pink"),
        ("#9c004a", "Dark Hot Pink"),
        ("#d90166", "Dark Pink"),
        ("#cb416b", "Rich Maroon"),
        ("#b0306a", "Alien Purple"),
        ("#490648", "Brownish Purple"),
        ("#76424e", "Dark Purple"),
        ("#35063e", "Dark Strawberry"),
        ("#80444c", "Deep Purple"),
        ("#36013f", "Medium Taupe"),
        ("#674c47", "Midnight Purple"),
        ("#280137", "Pickled Beet"),
        ("#4d233d", "Purple Basil"),
        ("#5c4450", "Purple Brown"),
        ("#673a3f", "Purple Stone"),
        ("#605467", "Antique Brown"),
        ("#553f2d", "Blackened Brown"),
        ("#442200", "Brown Stone"),
        ("#593c39", "Chestnut"),
        ("#742802", "Dark Brown"),
        ("#341c02", "Dark Chocolate"),
        ("#490206", "Deep Brown"),
        ("#410200", "Earth Brown"),
        ("#4f1507", "Flat Brown"),
        ("#7f7053", "Hardwood"),
        ("#937a62", "Marsh"),
        ("#5c5337", "Polished Brown"),
        ("#985538", "Toffee"),
        ("#755139", "Tree House"),
        ("#3b2820", "Tuk Tuk"),
        ("#573b2a", "Alien"),
        ("#415764", "Brick Grey"),
        ("#48412b", "Cape Cod"),
        ("#4e5552", "Carbon"),
        ("#333333", "Dark Grey"),
        ("#363737", "Dusty Olive"),
        ("#646356", "Flint"),
        ("#716e61", "Heavy Charcoal"),
        ("#565350", "Holly"),
        ("#25342b", "Lunar Green"),
        ("#4e5541", "Night Sky"),
        ("#2a2a35", "Polished Steel"),
        ("#6f828a", "Soft Steel"),
        ("#404854", "Sooty Willow Bamboo"),
        ("#4d4b3a", "Space Grey"),
        ("#110022", "Tap Shoe"),
        ("#2a2b2d", "Tarmac"),
        ("#5a5348", "Thyme"),
        ("#50574c", "Very Dark Brown"),
        ("#1d0200", "Walnut Hull"),
        ("#5d5242", "Black Olive"),
        ("#3b3c36", "Black Pearl"),
        ("#1e272c", "Coffee Bean"),
        ("#362d26", "Cynical Black"),
        ("#171717", "Kurobeni"),
        ("#23191e", "Liquorice"),
        ("#0a0502", "Pot Black"),
        ("#161616", "Reversed Grey"),
        ("#080808", "Sepia Black"),
        ("#2b0202", "Void"),
        ("#0f3b57", "Dark Navy Blue"),
        ("#050d25", "Wood Bark"),
        ("#1b1811", "Black Coffee"),
        ("#3b302f", "Black"),
    ]
}
