use subxt::config::substrate::AccountId32;

pub fn get_child_bounty_id_from_storage_key(key: Vec<u8>) -> u32 {
    let s = &key[key.len() - 4..];
    let v: [u8; 4] = s.try_into().expect("slice with incorrect length");
    u32::from_le_bytes(v)
}

pub fn str(bytes: Vec<u8>) -> String {
    format!("{}", String::from_utf8(bytes).expect("Data not utf-8"))
}

pub fn compact(account: &AccountId32) -> String {
    let a = account.to_string();
    [&a[0..4], &a[a.len() - 4..a.len()]].join("...")
}
