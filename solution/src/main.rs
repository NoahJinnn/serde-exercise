use std::num::NonZeroU64;

use message_program::{send_instruction, Message};

fn main() {
    let instruction_kinds: [u8; 3] = [3, 6, 9];

    for kind in instruction_kinds.iter() {
        let msg = Message {
            sender_id: [100; 32],
            size: Message::LEN as u64,
            priority_fee: Some(NonZeroU64::new(1).unwrap()),
            data: [0; 1024],
        };
        let mut instruction_bytes = [0u8; std::mem::size_of::<Message>() + 1];
        instruction_bytes[0] = *kind;

        let msg_bytes = bytemuck::bytes_of(&msg);
        instruction_bytes[1..].copy_from_slice(msg_bytes);
        send_instruction(&instruction_bytes).unwrap();
    }
}
