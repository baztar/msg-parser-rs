use msg_parser::OldOutlook;

fn main() {
    // Create Outlook object
    let outlook = OldOutlook::from_path("data/test_email_4.msg").unwrap();

    // Flush as json string
    let _json_string = outlook.to_json();

    println!("{:#?}", outlook);
}
