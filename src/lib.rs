// Import pure and fast JSON library written in Rust
use regex::Regex;
use serde_json::json;
use serde_json::Value;
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn rust_filter(
    tag: *const c_char,
    tag_len: u32,
    time_sec: u32,
    time_nsec: u32,
    record: *const c_char,
    record_len: u32,
) -> *const u8 {
    // RAW DATA
    //let message_ptr = unsafe { slice::from_raw_parts(record as *const u8, record_len as usize) };
    //let message_str = str::from_utf8(message_ptr).expect("no record available");

    // JSON
    let slice_record: &[u8] =
        unsafe { slice::from_raw_parts(record as *const u8, record_len as usize) };
    let json_record: Value = serde_json::from_slice(slice_record).unwrap();

    //let re = Regex::new(r"calico-packet").unwrap();
    let re = Regex::new(r"calico-packet.*SRC=(?<SRC>[^\s]+)").unwrap();
    //let re = Regex::new(r"calico-packet.*SRC=(?<SRC>[^\s]+) DST=(?<DST>[^\s]+).*PROTO=(?<PROTO>\w+)SPT=(?<SPT>\d+) DPT=(?<DPT>\d+)").expect("regex build error");
    let iptable = json_record["iptable"].as_str().unwrap();
    let Some(fields) = re.captures(iptable) else {
        let message = json!({
            "parse iptable record": "error".to_string(),
            "original": json_record.to_string(),
        });
        return message.to_string().as_ptr();
    };

    let message = json!({
        "message": json_record["iptable"],
        "original2": json_record.to_string(),
        "parse iptable record": "ok".to_string(),
        "SRC": &fields["SRC"],
        //"DST": &fields["DST"],
        //"PROTO": &fields["PROTO"],
        //"SPT": &fields["SPT"],
        //"DPT": &fields["DPT"],
    });

    let buf: String = message.to_string();
    buf.as_ptr()
}
