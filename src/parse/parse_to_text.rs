use crate::parse::parse_input;
use std::collections::HashMap;

const SENSITIVE_STRING: [&str; 5] = ["-ch", "-cat", "-r", "-p", "-t"];

pub fn parse_to_text(mut unparsed: String) -> String {
    let mut full_text = String::from("These data were detected\n\n");
    unparsed = unparsed.trim().replace('\n', " ");

    let splitted_data: Vec<String> = unparsed.split("-cat ").map(|s| s.to_string()).collect();

    for split in splitted_data {
        if split.is_empty() {
            continue;
        }

        let parsed_data = parse_input(format!("-cat {split}"));

        match parsed_data {
            Ok(data) => {
                full_text.push_str(&main_text(data));
            }
            Err(data) => {
                full_text.push_str(&main_text(data));
                full_text.push_str("\nError acquired in the upper section")
            }
        }

        full_text.push('\n')
    }
    full_text
}

fn channel_text(channel_data: HashMap<&str, Vec<String>>) -> String {
    let mut full_text = String::new();

    if channel_data.contains_key("roles") {
        full_text.push_str("Roles:");
        for role in &channel_data["roles"] {
            full_text.push_str(&format!(" {},", role))
        }
        full_text.pop().unwrap();
        full_text.push(' ');
    }

    if channel_data.contains_key("private") {
        full_text.push_str("Private: Yes ")
    }
    if channel_data.contains_key("channel_type") {
        match channel_data["channel_type"][0].as_str() {
            "text" => full_text.push_str(&format!("Channel Type: Text")),
            "voice" => full_text.push_str(&format!("Channel Type: voice")),
            "ann" => full_text.push_str(&format!("Channel Type: Announcement")),
            _ => {}
        }
    }

    full_text
}

fn main_text(data: HashMap<&str, Vec<String>>) -> String {
    let mut full_text = String::new();

    if data.contains_key("category") && !data["category"][0].is_empty() {
        full_text.push_str(&format!("Category: {}\n", data["category"][0]))
    }

    if data.contains_key("roles") {
        full_text.push_str("Roles:");
        for role in &data["roles"] {
            full_text.push_str(&format!(" {},", role))
        }
        full_text.pop().unwrap();
        full_text.push('\n')
    }

    if data.contains_key("private") {
        full_text.push_str("Private: Yes\n")
    } else {
        full_text.push_str("Private: No\n")
    }

    if data.contains_key("channels") {
        full_text.push_str("Channels:\n");
        for channel in &data["channels"] {
            let mut channel_name_unparsed: String = String::new();

            for word in channel.split(' ').collect::<Vec<&str>>() {
                if SENSITIVE_STRING.contains(&word) {
                    break;
                }
                if word.starts_with("|") {
                    break;
                }

                channel_name_unparsed.push_str(&format!(" {word}"));
            }
            println!("{}", channel_name_unparsed);
            channel_name_unparsed = channel_name_unparsed.trim().to_string();

            let channel_name = channel_name_unparsed.replace(" ", "-");

            full_text.push_str(&format!("    {channel_name}: "));
            let channel = channel
                .replace(&channel_name_unparsed, "")
                .trim()
                .to_string();

            if !channel.is_empty() {
                let parsed_channel = parse_input(channel.to_string());
                match parsed_channel {
                    Ok(data) => full_text.push_str(&channel_text(data)),
                    Err(data) => {
                        full_text.push_str(&channel_text(data));
                        full_text.push_str(". Error acquired here")
                    }
                }
            }
            full_text.push('\n')
        }
        full_text.push('\n')
    }

    full_text
}
