use redev::{simulate, EventType, Key, SimulateError};
use std::{thread, time};

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

#[cfg(target_os = "macos")]
fn test_macos_keys() {
    let virtual_input = redev::VirtualInput::new(
        redev::CGEventSourceStateID::Private,
        redev::CGEventTapLocation::Session,
    )
    .unwrap();

    let key_caps = redev::Key::RawKey(redev::RawKey::MacVirtualKeycode(redev::kVK_CapsLock));
    let key_ansi_a = redev::Key::RawKey(redev::RawKey::MacVirtualKeycode(redev::kVK_ANSI_A));
    {
        println!(
            "caps lock satte 1 {}",
            redev::VirtualInput::get_key_state(
                redev::CGEventSourceStateID::CombinedSessionState,
                redev::kVK_CapsLock
            )
        );

        virtual_input
            .simulate(&redev::EventType::KeyPress(key_caps))
            .unwrap();
        thread::sleep(time::Duration::from_millis(20));

        println!(
            "caps lock satte 2 {}",
            redev::VirtualInput::get_key_state(
                redev::CGEventSourceStateID::CombinedSessionState,
                redev::kVK_CapsLock
            )
        );

        virtual_input
            .simulate(&redev::EventType::KeyPress(key_ansi_a))
            .unwrap();
        thread::sleep(time::Duration::from_millis(20));
        virtual_input
            .simulate(&redev::EventType::KeyRelease(key_ansi_a))
            .unwrap();
        thread::sleep(time::Duration::from_millis(20));

        virtual_input
            .simulate(&redev::EventType::KeyRelease(key_caps))
            .unwrap();
        thread::sleep(time::Duration::from_millis(20));

        println!(
            "caps lock satte 3 {}",
            redev::VirtualInput::get_key_state(
                redev::CGEventSourceStateID::CombinedSessionState,
                redev::kVK_CapsLock
            )
        );
    }

    let command_tab = redev::Key::RawKey(redev::RawKey::MacVirtualKeycode(redev::kVK_Command));
    let key_tab = redev::Key::RawKey(redev::RawKey::MacVirtualKeycode(redev::kVK_CapsLock));
    {
        virtual_input
            .simulate(&redev::EventType::KeyPress(command_tab))
            .unwrap();
        thread::sleep(time::Duration::from_millis(200));

        virtual_input
            .simulate(&redev::EventType::KeyPress(key_tab))
            .unwrap();
        thread::sleep(time::Duration::from_millis(200));
        virtual_input
            .simulate(&redev::EventType::KeyRelease(key_tab))
            .unwrap();
        thread::sleep(time::Duration::from_millis(200));

        virtual_input
            .simulate(&redev::EventType::KeyPress(key_tab))
            .unwrap();
        thread::sleep(time::Duration::from_millis(200));
        virtual_input
            .simulate(&redev::EventType::KeyRelease(key_tab))
            .unwrap();
        thread::sleep(time::Duration::from_millis(200));

        virtual_input
            .simulate(&redev::EventType::KeyRelease(command_tab))
            .unwrap();
        thread::sleep(time::Duration::from_millis(200));
    }
}

#[cfg(windows)]
fn test_simulate_vk() {
    let _ = redev::simulate_code(Some(0xA2), None, true);
    let _ = redev::simulate_code(Some(0x4F), None, true);
    let _ = redev::simulate_code(Some(0x4F), None, false);
    let _ = redev::simulate_code(Some(0xA2), None, false);
}

#[cfg(windows)]
fn test_simulate_char() {
    println!("{:?}", redev::simulate_char('A', false));
    println!("{:?}", redev::simulate_char('€', false));
    println!("{:?}", redev::simulate_char('€', true));
}

#[cfg(target_os = "linux")]
fn simulate_combination() {
    send(&EventType::KeyPress(Key::ControlLeft));
    redev::linux::simulate_char('€', true);
    redev::linux::simulate_char('€', false);
    send(&EventType::KeyRelease(Key::ControlLeft));
}

fn test_simulate_dead() {
    send(&EventType::KeyPress(Key::AltGr));
    send(&EventType::KeyPress(Key::KeyE));
    send(&EventType::KeyRelease(Key::KeyE));
    send(&EventType::KeyRelease(Key::AltGr));
}

fn main() {
    // Windows: LeftBracket
    // scancode 26 => [
    // in us: [
    // in fr: ^(dead key)

    // send(&EventType::KeyPress(Key::Unknown(219)));
    // send(&EventType::KeyRelease(Key::Unknown(219)));

    // send(&EventType::KeyPress(Key::LeftBracket));
    // send(&EventType::KeyRelease(Key::LeftBracket));

    // #[cfg(target_os = "linux")]
    // simulate_combination();

    test_simulate_dead();

    // #[cfg(windows)]
    // test_simulate_vk();

    #[cfg(windows)]
    test_simulate_char();

    #[cfg(target_os = "macos")]
    test_macos_keys();
}
