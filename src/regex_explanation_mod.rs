use core::ops::Range;
use regex_syntax::ast::*;

pub fn lib_main(reg_str: String) -> String {
    let mut exp = Explanation {
        reg_str,
        ..Default::default()
    };
    exp.explanation_all.push_str("regex explanation start\n");
    exp.explanation_all
        .push_str(&format!("regex:{}\n", &exp.reg_str));
    let mut ast = regex_syntax::ast::parse::Parser::new();
    let ast = ast.parse(&exp.reg_str).unwrap();

    //dbg!(&ast);
    process_ast(&mut exp, &Box::new(ast));
    exp.print_literals();
    exp.explanation_all.push_str("regex explanation end\n");
    // return
    exp.explanation_all.to_string()
}

#[derive(Debug, Default)]
pub struct Explanation {
    pub reg_str: String,
    pub indent: usize,
    pub explanation_all: String,
    // print literals in one line for brevity
    pub fragment: String,
    pub symbol: String,
    pub explanation: String,
}

impl Explanation {
    fn print_literals(&mut self) {
        if !self.fragment.is_empty() {
            self.print_fragment(&self.fragment.clone());
            self.explanation_all
                .push_str(&format!("{}\n", self.explanation));
            self.fragment.clear();
            self.explanation.clear();
        }
    }
    fn set_explanation(&mut self, symbol: &str, name: &str) {
        self.symbol.clear();
        self.symbol.push_str(&" ".repeat(self.indent * 2));
        self.symbol.push_str(symbol);
        self.symbol
            .push_str(if symbol.is_empty() { "" } else { " " });

        self.explanation.clear();
        self.explanation.push_str(name);
    }
    fn print_fragment(&mut self, fragment: &str) {
        // the left 16 col are for the regex fragment
        const COL_WIDTH: usize = 16;
        let mut pos_start = 0;
        loop {
            let pos_end = fragment.len().min(pos_start + COL_WIDTH);
            self.explanation_all
                .push_str(&format!("{}", &fragment[pos_start..pos_end]));
            if pos_end == fragment.len() {
                let rest = COL_WIDTH - (pos_end % COL_WIDTH);
                self.explanation_all
                    .push_str(&format!("{}", " ".repeat(rest)));
                break;
            }
            pos_start = pos_end;
            self.explanation_all.push_str(&format!("\n"));
        }
    }
}

/// print any token except literals
fn print(exp: &mut Explanation, symbol: &str, name: &str, range: Range<usize>) {
    exp.print_literals();
    exp.print_fragment(&exp.reg_str[range].to_string());
    exp.set_explanation(symbol, name);
    exp.explanation_all
        .push_str(&format!("{}{}\n", &exp.symbol, &exp.explanation));
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
                GroupKind::CaptureIndex(i) => {
                    print(exp, "( )", &format!("Capture Group {}", i), span)
                }
                GroupKind::CaptureName(n) => print(
                    exp,
                    "(?P<>",
                    &format!("Named Capture Group {}", n.name),
                    span,
                ),
                GroupKind::NonCapturing(f) => {
                    print(exp, "(?:", "Non-capturing group", span);
                    if !f.items.is_empty() {
                        print(
                            exp,
                            "(? )",
                            &format!("Flags"),
                            f.span.start.offset..f.span.end.offset,
                        );
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
                    if b.negated {
                        "Bracketed negated"
                    } else {
                        "Bracketed"
                    },
                    b.span.start.offset..b.span.end.offset,
                );
                match &b.kind {
                    ClassSet::Item(i) => class_set_item(exp, i, b.negated),
                    _ => exp.explanation_all.push_str(&format!("unimplemented 1\n")),
                }
            }
            Class::Perl(p) => match p.kind {
                ClassPerlKind::Digit => print(
                    exp,
                    r"\d",
                    r"Matches a digit (equal to [0-9])",
                    p.span.start.offset..p.span.end.offset,
                ),
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
            _ => exp
                .explanation_all
                .push_str(&format!("unimplemented Class\n")),
        },
        Ast::Concat(c) => {
            for a in c.asts.iter() {
                process_ast(exp, &Box::new(a.clone()));
            }
        }
        Ast::Literal(l) => literal(exp, l, false),
        Ast::Assertion(a) => match a.kind {
            AssertionKind::EndLine => print(
                exp,
                "$",
                "Asserts position at the end of a line",
                a.span.start.offset..a.span.end.offset,
            ),
            _ => exp
                .explanation_all
                .push_str(&format!("unimplemented Assertion\n")),
        },
        Ast::Alternation(a) => {
            print(
                exp,
                "|",
                "Alternatives",
                a.span.start.offset..a.span.end.offset,
            );
            exp.indent += 1;
            for (i, x) in a.asts.iter().enumerate() {
                print(
                    exp,
                    "|",
                    &format!("Alternative {}", i),
                    x.span().start.offset..x.span().end.offset,
                );
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
                    &format!(
                        "Quantifier — Matches between zero and one times - {}",
                        greed(r.greedy)
                    ),
                    r.span.start.offset..r.span.end.offset,
                ),
                RepetitionKind::ZeroOrMore => print(
                    exp,
                    if r.greedy { "*" } else { "*?" },
                    &format!(
                        "Quantifier — Matches between zero and unlimited times - {}",
                        greed(r.greedy)
                    ),
                    r.span.start.offset..r.span.end.offset,
                ),
                RepetitionKind::OneOrMore => print(
                    exp,
                    if r.greedy { "+" } else { "+?" },
                    &format!(
                        "Quantifier — Matches between one and unlimited times - {}",
                        greed(r.greedy)
                    ),
                    r.span.start.offset..r.span.end.offset,
                ),
                RepetitionKind::Range(rr) => match rr {
                    RepetitionRange::Exactly(e) => print(
                        exp,
                        "{n}",
                        &format!(
                            "Quantifier — Matches exactly {} times - {}",
                            e,
                            greed(r.greedy)
                        ),
                        r.span.start.offset..r.span.end.offset,
                    ),
                    RepetitionRange::Bounded(m, n) => print(
                        exp,
                        if r.greedy { "{m,n}" } else { "{m,n}?" },
                        &format!(
                            "Quantifier — Matches between {} and {} times - {}",
                            m,
                            n,
                            greed(r.greedy)
                        ),
                        r.span.start.offset..r.span.end.offset,
                    ),
                    _ => exp
                        .explanation_all
                        .push_str(&format!("unimplemented RepetitionRange\n")),
                },
            }
            process_ast(exp, &r.ast);
            exp.print_literals();
        }
        Ast::Flags(f) => {
            print(
                exp,
                "(? )",
                &format!("Flags"),
                f.span.start.offset..f.span.end.offset,
            );
            flags(exp, &f.flags);
        }
        _ => exp.explanation.push_str(&format!("unimplemented Ast")),
    }
    exp.indent -= 1;
}
pub fn flags(exp: &mut Explanation, f: &Flags) {
    for item in f.items.iter() {
        match item.kind{
            FlagsItemKind::Negation => print(
                exp,
                "-",
                &format!("Flag negation"),
                item.span.start.offset..item.span.end.offset
            ),
            FlagsItemKind::Flag(flag)=>match flag{
                Flag::CaseInsensitive=>print(
                    exp,
                    "i modifier",
                    &format!("insensitive. Case insensitive match (ignores case of [a-zA-Z])"),
                    item.span.start.offset..item.span.end.offset
                ),
                    Flag::MultiLine=>print(
                    exp,
                    "m modifier",
                    &format!(" multi line. Causes ^ and $ to match the begin/end of each line (not only begin/end of string)"),
                    item.span.start.offset..item.span.end.offset
                ),
                    Flag::DotMatchesNewLine=>print(
                    exp,
                    "s modifier",
                    &format!(" single line. Dot matches also newline characters."),
                    item.span.start.offset..item.span.end.offset
                ),
                Flag::SwapGreed=>print(
                    exp,
                    "U modifier",
                    &format!("ungreedy. The match becomes lazy by default. Now a ? following a quantifier makes it greedy"),
                    item.span.start.offset..item.span.end.offset
                ),
                Flag::Unicode=>print(
                    exp,
                    "u modifier",
                    &format!("unicode"),
                    item.span.start.offset..item.span.end.offset
                ),
                Flag::IgnoreWhitespace=>print(
                    exp,
                    "x modifier",
                    &format!("ignore whotespace"),
                    item.span.start.offset..item.span.end.offset
                ),
            }
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
                "Match a single character present in the list"
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
        _ => exp
            .explanation_all
            .push_str(&format!("unimplemented class_set_item\n")),
    }
}
pub fn literal(exp: &mut Explanation, l: &Literal, negated: bool) {
    match l.kind {
        LiteralKind::Verbatim => {
            if exp.fragment.is_empty() {
                // start the line, but don't print, leave the possibility to add more literals
                // the actual print will happen when somebody else call print.
                if negated {
                    exp.set_explanation("", "Match all characters except this literally");
                } else {
                    exp.set_explanation("", "Matches characters literally");
                }
                exp.fragment.clear();
            }
            // add one literal to the same line
            exp.fragment
                .push_str(&exp.reg_str[l.span.start.offset..l.span.end.offset]);
        }
        LiteralKind::Punctuation => print(
            exp,
            "\\",
            "Matches the escaped character literally",
            l.span.start.offset..l.span.end.offset,
        ),
        _ => exp
            .explanation_all
            .push_str(&format!("unimplemented literal\n")),
    }
}
