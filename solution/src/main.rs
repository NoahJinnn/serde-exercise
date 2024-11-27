use std::num::NonZeroU64;

use message_program::{send_instruction, Message};

fn main() {
    let sender_id_types = [3, 6, 9];

    for sender_id_type in sender_id_types.iter() {
        let msg = Message {
            sender_id: [*sender_id_type; 32],
            size: Message::LEN as u64,
            priority_fee: Some(NonZeroU64::new(1).unwrap()),
            data: [0; 1024],
        };
        let data = bytemuck::bytes_of(&msg);
        send_instruction(data).unwrap();
    }
}
