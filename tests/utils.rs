
use color_print::cprintln;
use handoff_register::handoff::Handoff;

pub fn show_blue(oper: &str, h: &Handoff<i32>) {
    cprintln!("<blue,bold>[{}]</> {}", oper, h);
}

pub fn show_red(oper: &str, h: &Handoff<i32>) {
    cprintln!("<red,bold>[{}]</> {}", oper, h);
}