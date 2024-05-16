#[derive(Debug)]
pub enum Message {
    NewPosition { id: u32, dx: i32, dy: i32 },
}