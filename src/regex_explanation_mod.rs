use crate::html_encoded_push;
use crate::web_sys_mod::*;

use core::ops::Range;
use regex_syntax::ast::*;

#[derive(Debug, Default)]
pub struct Explanation {
    pub reg_str: String,
    pub html: HtmlEncoded,
    pub single_line: SingleLine,
    pub indent: usize,
    pub color_index: usize,
    /// ('s|e', pos, color_index)
    pub span_colors: Vec<(char, usize, usize)>,
}

/// builder elements for a single line
#[derive(Debug, Default)]
pub struct SingleLine {
    // print literals in one line for brevity
    pub fragment: String,
    pub symbol: String,
    pub explanation: String,
    pub range_start: usize,
    pub range_end: usize,
}

/// creates html encoded with the macro html_encoded_push!()
pub fn create_explanation_html(reg_str: String) -> HtmlEncoded {
    let mut exp = Explanation {
        reg_str,
        html: crate::web_sys_mod::HtmlEncoded::new(),
        ..Default::default()
    };

    exp.html.push_new_line();
    let mut ast = regex_syntax::ast::parse::Parser::new();
    let ast = match ast.parse(&exp.reg_str) {
        Ok(a) => a,
        Err(e) => {
            html_encoded_push!(exp.html, "Error: {}\n", &e.to_string());
            return exp.html;
        }
    };

    //dbg!(&ast);
    process_ast(&mut exp, &Box::new(ast));
    exp.print_literals();

    let html_2nd = exp.colorize_regex_text();
    exp.html.insert_html(0, &html_2nd);

    // return
    exp.html
}

impl Explanation {
    fn print_literals(&mut self) {
        if !self.single_line.fragment.is_empty() {
            self.span_colors.push(('s', self.single_line.range_start, self.color_index));
            self.span_colors.push(('e', self.single_line.range_end, self.color_index));
            self.print_fragment(&self.single_line.fragment.clone());
            html_encoded_push!(self.html, r#"{}"#, &self.single_line.explanation);
            self.html.push_new_line();

            self.color_index_increment();
            self.clear_single_line();
        }
    }
    fn clear_single_line(&mut self) {
        self.single_line.fragment.clear();
        self.single_line.explanation.clear();
        self.single_line.symbol.clear();
        self.single_line.range_start = 0;
        self.single_line.range_end = 0;
    }
    fn set_explanation(&mut self, symbol: &str, name: &str) {
        self.single_line.symbol.clear();
        self.single_line.symbol.push_str(&" ".repeat(self.indent * 2));
        self.single_line.symbol.push_str(symbol);
        self.single_line.symbol.push_str(if symbol.is_empty() { "" } else { " " });

        self.single_line.explanation.clear();
        self.single_line.explanation.push_str(name);
    }
    fn print_fragment(&mut self, fragment: &str) {
        // if is longer than 16, than new line and 16 spaces
        const COL_WIDTH: usize = 16;

        html_encoded_push!(
            self.html,
            r#"<span class="b_color_{}">{}</span>"#,
            &format!("{:02}", self.color_index),
            fragment
        );
        if fragment.len() < COL_WIDTH + self.indent {
            let rest = COL_WIDTH - fragment.len();
            html_encoded_push!(self.html, "{}", &" ".repeat(rest));
        } else {
            self.html.push_new_line();
            html_encoded_push!(self.html, "{}", &" ".repeat(COL_WIDTH));
        }
    }
    fn color_index_increment(&mut self) {
        self.color_index += 1;
    }
    /// return the regex with colors
    fn colorize_regex_text(&mut self) -> HtmlEncoded {
        let mut html_2nd = crate::web_sys_mod::HtmlEncoded::new();
        html_encoded_push!(html_2nd, r#"regex: "#);
        // sort the token events
        self.span_colors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        //debug_write(&format!("{:?}", self.span_colors));
        let mut cursor_pos = 0;
        for c in self.span_colors.iter() {
            html_encoded_push!(html_2nd, "{}", &self.reg_str[cursor_pos..c.1]);
            if c.0 == 's' {
                html_encoded_push!(html_2nd, r#"<span class="b_color_{}">"#, &format!("{:02}", c.2));
            } else {
                // c.0== 'e'
                html_encoded_push!(html_2nd, r#"</span>"#);
            }
            cursor_pos = c.1;
        }
        html_encoded_push!(html_2nd, "{}", &self.reg_str[cursor_pos..]);
        html_2nd.push_new_line();
        // return
        html_2nd
    }
}

/// print any token except literals
fn print(exp: &mut Explanation, symbol: &str, name: &str, range: Range<usize>) {
    exp.print_literals();
    exp.span_colors.push(('s', range.start, exp.color_index));
    exp.span_colors.push(('e', range.end, exp.color_index));
    exp.print_fragment(&exp.reg_str[range].to_string());
    exp.set_explanation(symbol, name);
    html_encoded_push!(
        exp.html,
        r#"<span class="hljs-keyword">{}</span>{}"#,
        &exp.single_line.symbol,
        &exp.single_line.explanation
    );
    exp.html.push_new_line();
    exp.color_index_increment();
}

fn greed(greedy: bool) -> &'static str {
    if greedy {
        "greedy"
    } else {
        "ungreedy"
    }
}
pub fn process_ast(exp: &mut Explanation, ast: &Box<Ast>) {
    exp.indent += 1;
    let ast = ast.as_ref();
    match ast {
        Ast::Dot(d) => print(
            exp,
            ".",
            "Matches any character (except for line terminators)",
            d.start.offset..d.end.offset,
        ),
        Ast::Group(g) => {
            let span = g.span.start.offset..g.span.end.offset;
            match &g.kind {
                GroupKind::CaptureIndex(i) => print(exp, "( )", &format!("Capture Group {}", i), span),
                GroupKind::CaptureName(n) => print(exp, "(?P<>", &format!("Named Capture Group {}", n.name), span),
                GroupKind::NonCapturing(f) => {
                    print(exp, "(?:", "Non-capturing group", span);
                    if !f.items.is_empty() {
                        print(exp, "(? )", &format!("Flags"), f.span.start.offset..f.span.end.offset);
                        flags(exp, &f);
                    }
                }
            }
            process_ast(exp, &g.ast);
        }
        Ast::Class(c) => match c {
            Class::Bracketed(b) => {
                print(
                    exp,
                    if b.negated { "[^ ]" } else { "[ ]" },
                    if b.negated { "Bracketed negated" } else { "Bracketed" },
                    b.span.start.offset..b.span.end.offset,
                );
                match &b.kind {
                    ClassSet::Item(i) => class_set_item(exp, i, b.negated),
                    _ => html_encoded_push!(exp.html, "unimplemented 1\n"),
                }
            }
            Class::Perl(p) => match p.kind {
                ClassPerlKind::Digit => print(exp, r"\d", r"Matches a digit (equal to [0-9])", p.span.start.offset..p.span.end.offset),
                ClassPerlKind::Space => print(
                    exp,
                    r"\s",
                    r"Matches any whitespace character (equal to [\r\n\t\f\v ])",
                    p.span.start.offset..p.span.end.offset,
                ),
                ClassPerlKind::Word => print(
                    exp,
                    r"\w",
                    r"Matches any word character (equal to [a-zA-Z0-9_])",
                    p.span.start.offset..p.span.end.offset,
                ),
            },
            _ => html_encoded_push!(exp.html, "unimplemented Class\n"),
        },
        Ast::Concat(c) => {
            for a in c.asts.iter() {
                process_ast(exp, &Box::new(a.clone()));
            }
        }
        Ast::Literal(l) => literal(exp, l, false),
        Ast::Assertion(a) => match a.kind {
            AssertionKind::EndLine => print(exp, "$", "Asserts position at the end of a line", a.span.start.offset..a.span.end.offset),
            _ => html_encoded_push!(exp.html, "unimplemented Assertion\n"),
        },
        Ast::Alternation(a) => {
            print(exp, "|", "Alternatives", a.span.start.offset..a.span.end.offset);
            exp.indent += 1;
            for (i, x) in a.asts.iter().enumerate() {
                print(exp, "|", &format!("Alternative {}", i), x.span().start.offset..x.span().end.offset);
                exp.indent += 1;
                process_ast(exp, &Box::new(x.clone()));
                exp.indent -= 1;
            }
            exp.indent -= 1;
        }
        Ast::Repetition(r) => {
            match &r.op.kind {
                RepetitionKind::ZeroOrOne => print(
                    exp,
                    "?",
                    &format!("Quantifier — Matches between zero and one times - {}", greed(r.greedy)),
                    r.span.start.offset..r.span.end.offset,
                ),
                RepetitionKind::ZeroOrMore => print(
                    exp,
                    if r.greedy { "*" } else { "*?" },
                    &format!("Quantifier — Matches between zero and unlimited times - {}", greed(r.greedy)),
                    r.span.start.offset..r.span.end.offset,
                ),
                RepetitionKind::OneOrMore => print(
                    exp,
                    if r.greedy { "+" } else { "+?" },
                    &format!("Quantifier — Matches between one and unlimited times - {}", greed(r.greedy)),
                    r.span.start.offset..r.span.end.offset,
                ),
                RepetitionKind::Range(rr) => match rr {
                    RepetitionRange::Exactly(e) => print(
                        exp,
                        "{n}",
                        &format!("Quantifier — Matches exactly {} times - {}", e, greed(r.greedy)),
                        r.span.start.offset..r.span.end.offset,
                    ),
                    RepetitionRange::Bounded(m, n) => print(
                        exp,
                        if r.greedy { "{m,n}" } else { "{m,n}?" },
                        &format!("Quantifier — Matches between {} and {} times - {}", m, n, greed(r.greedy)),
                        r.span.start.offset..r.span.end.offset,
                    ),
                    _ => html_encoded_push!(exp.html, "unimplemented RepetitionRange\n"),
                },
            }
            process_ast(exp, &r.ast);
            exp.print_literals();
        }
        Ast::Flags(f) => {
            print(exp, "(? )", &format!("Flags"), f.span.start.offset..f.span.end.offset);
            flags(exp, &f.flags);
        }
        _ => html_encoded_push!(exp.html, "unimplemented Ast"),
    }
    exp.indent -= 1;
}
pub fn flags(exp: &mut Explanation, f: &Flags) {
    for item in f.items.iter() {
        match item.kind {
            FlagsItemKind::Negation => print(exp, "-", &format!("Flag negation"), item.span.start.offset..item.span.end.offset),
            FlagsItemKind::Flag(flag) => match flag {
                Flag::CaseInsensitive => print(
                    exp,
                    "i modifier",
                    &format!("insensitive. Case insensitive match (ignores case of [a-zA-Z])"),
                    item.span.start.offset..item.span.end.offset,
                ),
                Flag::MultiLine => print(
                    exp,
                    "m modifier",
                    &format!(" multi line. Causes ^ and $ to match the begin/end of each line (not only begin/end of string)"),
                    item.span.start.offset..item.span.end.offset,
                ),
                Flag::DotMatchesNewLine => print(
                    exp,
                    "s modifier",
                    &format!(" single line. Dot matches also newline characters."),
                    item.span.start.offset..item.span.end.offset,
                ),
                Flag::SwapGreed => print(
                    exp,
                    "U modifier",
                    &format!("ungreedy. The match becomes lazy by default. Now a ? following a quantifier makes it greedy"),
                    item.span.start.offset..item.span.end.offset,
                ),
                Flag::Unicode => print(exp, "u modifier", &format!("unicode"), item.span.start.offset..item.span.end.offset),
                Flag::IgnoreWhitespace => print(
                    exp,
                    "x modifier",
                    &format!("ignore whitespace"),
                    item.span.start.offset..item.span.end.offset,
                ),
            },
        }
    }
}
pub fn class_set_item(exp: &mut Explanation, i: &ClassSetItem, negated: bool) {
    match i {
        ClassSetItem::Perl(p) => match p.kind {
            ClassPerlKind::Digit => print(
                exp,
                if p.negated { r"\D" } else { r"\d" },
                if p.negated {
                    r"matches any character that's not a digit (equal to [^0-9])"
                } else {
                    r"matches a digit (equal to [0-9])"
                },
                p.span.start.offset..p.span.end.offset,
            ),
            ClassPerlKind::Space => print(
                exp,
                if p.negated { r"\S" } else { r"\s" },
                if p.negated {
                    r"matches any non-whitespace character (equal to [^\r\n\t\f\v ])"
                } else {
                    r"matches any whitespace character (equal to [\r\n\t\f\v ])"
                },
                p.span.start.offset..p.span.end.offset,
            ),
            ClassPerlKind::Word => print(
                exp,
                if p.negated { r"\W" } else { r"\w" },
                if p.negated {
                    r"matches any non-word character (equal to [^a-zA-Z0-9_])"
                } else {
                    r"matches any word character (equal to [a-zA-Z0-9_])"
                },
                p.span.start.offset..p.span.end.offset,
            ),
        },
        ClassSetItem::Union(u) => {
            for x in u.items.iter() {
                class_set_item(exp, x, negated);
            }
        }
        ClassSetItem::Literal(l) => literal(exp, l, negated),
        ClassSetItem::Range(r) => print(
            exp,
            "",
            if negated {
                "Match a single character not present in the list below"
            } else {
                if &exp.reg_str[r.span.start.offset..r.span.end.offset] == "a-z" {
                    "Matches lower-case character"
                } else if &exp.reg_str[r.span.start.offset..r.span.end.offset] == "a-z" {
                    "Matches lower-case character"
                } else if &exp.reg_str[r.span.start.offset..r.span.end.offset] == "A-Z" {
                    "Matches uppercase-case character"
                } else if &exp.reg_str[r.span.start.offset..r.span.end.offset] == "0-9" {
                    r#"Matches a digit, same as \d"#
                } else {
                    "Match a single character present in the list"
                }
            },
            r.span.start.offset..r.span.end.offset,
        ),
        ClassSetItem::Ascii(a) => match a.kind {
            ClassAsciiKind::Alnum => print(
                exp,
                "[:alnum:]",
                "Matches an alphanumeric character [a-zA-Z0-9]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Alpha => print(
                exp,
                "[:alpha:]",
                "Matches a alphabetic character [a-zA-Z]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Ascii => print(
                exp,
                "[:ascii:]",
                "Matches a character with ASCII value 0 through 127",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Blank => print(
                exp,
                "[:blank:]",
                "Matches a whitespace character, including a line break [ \t]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Cntrl => print(
                exp,
                "[:cntrl:]",
                "Matches a control character [\x00-\x1F\x7F]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Digit => print(
                exp,
                "[:digit:]",
                r"Matches a digit [0-9] (also written as \d)",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Graph => print(
                exp,
                "[:graph:]",
                "Matches a visible character [\x21-\x7E] [!-~]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Lower => print(
                exp,
                "[:lower:]",
                "Matches a lowercase letter [a-z]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Print => print(
                exp,
                "[:print:]",
                "Matches a visible character or the space character [\x20-\x7E] [ -~]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Punct => print(
                exp,
                "[:punct:]",
                r##"Matches a punctuation character [!-/:-@\[-`{-~]"##,
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Space => print(
                exp,
                "[:space:]",
                r"Matches a whitespace character, including a line break [ \t\r\n\v\f] (also written as \s)",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Upper => print(
                exp,
                "[:upper:]",
                "Matches a uppercase letter [A-Z]",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Word => print(
                exp,
                "[:word:]",
                r"Matches a alphanumeric character or _ [A-Za-z0-9_] (also written as \w)",
                a.span.start.offset..a.span.end.offset,
            ),
            ClassAsciiKind::Xdigit => print(
                exp,
                "[:xdigit:]",
                "Matches a hexadecimal digit [A-Fa-f0-9]",
                a.span.start.offset..a.span.end.offset,
            ),
        },
        _ => html_encoded_push!(exp.html, "unimplemented class_set_item\n"),
    }
}
pub fn literal(exp: &mut Explanation, l: &Literal, negated: bool) {
    match l.kind {
        LiteralKind::Verbatim => {
            if exp.single_line.fragment.is_empty() {
                exp.single_line.fragment.clear();
                exp.single_line.explanation.clear();
                exp.single_line.symbol.clear();
                // start the line, but don't print, leave the possibility to add more literals
                // the actual print will happen when somebody else call print.
                if negated {
                    exp.set_explanation("", "Match all characters except this literally");
                } else {
                    exp.set_explanation("", "Matches characters literally");
                }
                exp.single_line.range_start = l.span.start.offset;
            }
            // add one literal to the same line
            exp.single_line.fragment.push_str(&exp.reg_str[l.span.start.offset..l.span.end.offset]);
            exp.single_line.range_end = l.span.end.offset;
        }
        LiteralKind::Punctuation => print(
            exp,
            "\\",
            "Matches the escaped character literally",
            l.span.start.offset..l.span.end.offset,
        ),
        _ => html_encoded_push!(exp.html, "unimplemented literal\n"),
    }
}
