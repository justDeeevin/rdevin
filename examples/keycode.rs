fn main() {
    let keycode = redev::linux_keycode_from_key(redev::Key::Num1);
    dbg!(keycode);
}
