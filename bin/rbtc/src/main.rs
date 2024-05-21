use rbtc_wire::msg::version::MsgVersion;

fn main() {
    let msg = MsgVersion::new();
    println!("msg version: {:?}", msg);
}
