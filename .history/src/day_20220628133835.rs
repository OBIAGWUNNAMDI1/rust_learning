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
18
19 struct GroundStation {}
20
21 impl Mailbox {
22 fn post(&mut self, msg: Message) {
23 self.messages.push(msg);
24 }
25
26 fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
27 for i in 0..self.messages.len() {
28 if self.messages[i].to == recipient.id {
29 let msg = self.messages.remove(i);
30 return Some(msg);
31 }
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