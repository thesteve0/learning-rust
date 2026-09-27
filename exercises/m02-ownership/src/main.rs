use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AgentMessage {
    role: String,
    content: String,
}

impl Drop for AgentMessage {
    fn drop(&mut self) {
        println!("Dropping message from {:?}", self.role);
    }
}

fn describe(message: &str) {
    println!("Borrowed view: {message}");
}

fn append_period(message: &mut String) {
    if !message.ends_with('.') {
        message.push('.');
    }
}

fn acknowledgement(message: &AgentMessage) -> String {
    format!("Received {} message: {}", message.role, message.content)
}

#[tokio::main]
async fn main() -> Result<(), serde_json::Error> {
    let original = AgentMessage {
        role: String::from("user"),
        content: String::from("Explain Rust ownership"),
    };

    // `owner` now owns the value. Uncommenting `println!("{:?}", original)` here
    // produces the "borrow of moved value" error discussed in the lesson.
    let owner = original;

    let mut text = owner.content.clone();
    describe(&text);
    append_period(&mut text);
    println!("Mutated owned String: {text}");

    let json = serde_json::to_string(&owner)?;
    println!("Serialized JSON: {json}");

    let round_tripped: AgentMessage = serde_json::from_str(&json)?;
    println!("{}", acknowledgement(&round_tripped));

    drop(round_tripped);
    println!("The explicit drop above ran before the end of main.");

    Ok(())
}
