1 #![allow(unused_variables)]
2
3 #[derive(Debug)]
4 struct CubeSat {
5 id: u64,
6 }
7
8 #[derive(Debug)]
9 struct Mailbox {
10 messages: Vec<Message>,
11 }
12
13 #[derive(Debug)]
14 struct Message {
15 to: u64,
16 content: String,
17 }
struct GroundStation {}
 impl Mailbox {
    fn post(&mut self, msg: Message) {
     self.messages.push(msg);
}
fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
    for i in 0..self.messages.len() {
        if self.messages[i].to == recipient.id {
            let msg = self.messages.remove(i);
            return Some(msg);
}
}
None
}
}
impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
     CubeSat {
     id: sat_id,
} }
fn send(&self, mailbox: &mut Mailbox, msg: Message) {mailbox.post(msg);
} }
impl CubeSat {
fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
     mailbox.deliver(&self)
53 }
}
fn fetch_sat_ids() -> Vec<u64> {vec![1,2,3]
 }
fn main() {
    let mut mail = Mailbox { messages: vec![] };
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
let sat = base.connect(sat_id);
let msg = Message { to: sat_id, content: String::from("hello") };
base.send(&mut mail, msg);
 }
let sat_ids = fetch_sat_ids();
for sat_id in sat_ids {
 let sat = base.connect(sat_id);
 let msg = sat.recv(&mut mail);
 println!("{:?}: {:?}", sat, msg); } }