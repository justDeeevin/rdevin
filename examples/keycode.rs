fn main() {
    let keycode = redev::keycodes::linux::code_from_key(redev::Key::Num1);
    dbg!(keycode);
}
