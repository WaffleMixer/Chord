extern crate chord;
use std::collections::HashMap;
use chord::parse_input;

#[test]
fn test_parse_1() {
    let data = "-cat category name -p -r admin, team member -ch channel-1 -r another role, third role | channel-2 | channel-3".to_string();

    let expected_data = HashMap::from([
        ("category", vec!["category name".to_string()]),
        ("private", vec!["true".to_string()]),
        ("roles", vec!["admin".to_string(), "team member".to_string()]),
        ("channels", vec!["channel-1 -r another role, third role".to_string(), "channel-2".to_string(), "channel-3".to_string()])
    ]);
    let parsed_data = parse_input(data).unwrap();

    assert_eq!(expected_data, parsed_data);

    for mut data in parsed_data["channels"].to_owned() {
        let split: Vec<&str> = data.split(' ').collect();
        let channel_name = split[0];
        data = data.replace(&format!("{channel_name} "), "");

        let parsed_data = parse_input(format!("{data}")).unwrap();

        let expected_data = HashMap::from([
            ("roles", vec!["another role".to_string(), "third role".to_string()])
        ]);
        assert_eq!(parsed_data, expected_data);
        break;
    }
}