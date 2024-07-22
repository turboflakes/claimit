pub fn get_child_bounty_id_from_storage_key(key: Vec<u8>) -> u32 {
    let s = &key[key.len() - 4..];
    let v: [u8; 4] = s.try_into().expect("slice with incorrect length");
    u32::from_le_bytes(v)
}

pub fn str(bytes: Vec<u8>) -> String {
    format!("{}", String::from_utf8(bytes).expect("Data not utf-8"))
}
