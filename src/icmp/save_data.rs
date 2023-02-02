use std::{fs::OpenOptions, io::Write};
use csv::Reader;

pub fn init_file(file_name: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    let header = "block,count\n";
    file.write_all(header.as_bytes()).unwrap();
}

pub fn write_data(file_name: String, block: String) {
    let mut rdr = Reader::from_path(&file_name).unwrap();
    let mut found = false;

    for result in rdr.records() {
        let record = result.unwrap();
        let current_block = record.get(0).unwrap();
        let current_count: u32 = record.get(1).unwrap().parse().unwrap();

        if current_block == block {
            found = true;
            let new_count = current_count + 1;

            // Open the file in write mode
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&file_name)
                .unwrap();

            // Replace the current name and score with the updated values
            let data = format!("{},{}\n", block, new_count);
            let bytes = data.as_bytes();
            file.write_all(bytes).unwrap();
            break;
        }
    }

    if !found {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_name)
            .unwrap();

        let data = format!("{},1\n", block);
        let bytes = data.as_bytes();
        file.write_all(bytes).unwrap();
    }
}