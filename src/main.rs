fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() < 3 {
        eprintln!(
            "Usage: {} <traktor_nml_file> <output_rekordbox_file>",
            arguments[0]
        );
        std::process::exit(1);
    }

    let traktor_nml = std::fs::read_to_string(&arguments[1]).expect("Unable to read file");
    let traktor_data = traktor_n_rekords::parse_traktor_collection(&traktor_nml).unwrap();
    let rekordbox_data = traktor_n_rekords::traktor_to_rekordbox(traktor_data);
    let rekordbox_xml = serde_xml_rs::to_string(&rekordbox_data).unwrap();

    std::fs::write(&arguments[2], rekordbox_xml).expect("Unable to write file");
    println!("Rekordbox XML file has been written to {}", arguments[2]);
}
