use base64::{Engine as _, engine::general_purpose};
use fefix::definitions::*;
use fefix::tagvalue::*;
use rand::RngCore;
use std::time::{SystemTime, UNIX_EPOCH};

// Logon struct uwu
struct Logon {
    heart_bt_int: u32,
    username: String,
    password: String,
}

impl Logon {
    fn new(heart_bt_int: u32, username: &str, password: &str) -> Self {
        Self {
            heart_bt_int: heart_bt_int,
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    fn build_fix_message(&self, msg_seq_num: u32) -> Vec<u8> {
        let mut config = fefix::tagvalue::Config::default();
        config.set_separator(b'|');
        let mut encoder = Encoder::new(config);
        let mut buffer = Vec::new();
        // Tag 8 and 35 are handled by start_message
        let mut msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"A");

        msg.set(fix44::SENDER_COMP_ID, "logon");
        msg.set(fix44::TARGET_COMP_ID, "DERIBITSERVER");
        msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
        msg.set(fix44::HEART_BT_INT, self.heart_bt_int);

        // timestamp and nonce for raw data
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Build Nonce
        let mut nonce = vec![0u8; 32];
        rand::rng().fill_bytes(&mut nonce);
        let base64_nonce = general_purpose::STANDARD.encode(&nonce);
        let raw_data_str = format!("{}.{base64_nonce}", timestamp);
        let raw_data_len = raw_data_str.len();

        msg.set(fix44::RAW_DATA_LENGTH, raw_data_len);
        msg.set(fix44::RAW_DATA, raw_data_str.as_str());
        msg.set(fix44::USERNAME, self.username.as_str());
        msg.set(fix44::PASSWORD, self.password.as_str());

        drop(msg);
        buffer
    }
}
