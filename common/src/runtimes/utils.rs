use num_format::{Locale, ToFormattedString};
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
    [&a[..4], &a[a.len() - 4..a.len()]].join("...")
}

pub fn amount_human(value: u128, decimals: u32) -> String {
    let base: u128 = 10;
    let n = value / base.pow(decimals) as u128;
    let r = (value % base.pow(decimals) as u128) / base.pow((decimals - 2).into()) as u128;
    let s = n.to_formatted_string(&Locale::en);
    format!("{s}.{r}")
}
