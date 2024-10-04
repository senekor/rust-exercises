//! Two Japanese people are on the phone. One of them, the speaker, tells a
//! story. The other one, the listener, is practicing [aizuchi], a japanese
//! cultural norm where the listener interjects in the conversation with little
//! acknowledgments.
//!
//! However, there is a problem. The phone connection is synchronous, so all
//! the acknowledgments from the listener arrive only at the very end of the
//! conversation! What the speaker and listener say should be interleaved.
//!
//! Make the conversation async to prevent any cultural misunderstandings!
//!
//! Nevermind the fact that they're actually speaking Englich, they are
//! language students practicing among themselves.
//!
//! [aizuchi]: https://en.wikipedia.org/wiki/Aizuchi

use std::time::Duration;

async fn speaker(phone: Phone) {
    phone.say("So I was going to the mall...");
    std::thread::sleep(Duration::from_millis(2));
    phone.say("...where I met Susan by coincidence...");
    std::thread::sleep(Duration::from_millis(2));
    phone.say("...and she was wearing a purple hat!");
}

async fn listener(phone: Phone) {
    std::thread::sleep(Duration::from_millis(1));
    phone.say("u-hu.");
    std::thread::sleep(Duration::from_millis(2));
    phone.say("oh really?");
    std::thread::sleep(Duration::from_millis(2));
    phone.say("no way!");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (phone, wire_tap) = Phone::new();

    let listener_handle = tokio::spawn(listener(phone.clone()));
    speaker(phone).await;

    // make sure all messages get sent
    listener_handle.await.unwrap();

    for message in wire_tap {
        println!("{message}");
    }
}

/// This phone is wire-tapped for testing purposes.
#[derive(Clone)]
struct Phone {
    sender: std::sync::mpsc::Sender<&'static str>,
}
impl Phone {
    fn new() -> (Phone, std::sync::mpsc::Receiver<&'static str>) {
        let (sender, wire_tap) = std::sync::mpsc::channel();
        (Self { sender }, wire_tap)
    }
    fn say(&self, thing: &'static str) {
        self.sender.send(thing).unwrap();
    }
}

#[tokio::test]
async fn messages_are_interleaved() {
    let (phone, wire_tap) = Phone::new();

    let listener_handle = tokio::spawn(listener(phone.clone()));
    speaker(phone).await;

    // make sure all messages get sent
    listener_handle.await.unwrap();

    let messages = wire_tap.into_iter().collect::<Vec<_>>();

    assert_eq!(messages[0], "So I was going to the mall...");
    assert_eq!(messages[1], "u-hu.");
    assert_eq!(messages[2], "...where I met Susan by coincidence...");
    assert_eq!(messages[3], "oh really?");
    assert_eq!(messages[4], "...and she was wearing a purple hat!");
    assert_eq!(messages[5], "no way!");
}
