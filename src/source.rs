use std::io::BufRead;

pub fn process_stdin(level_filter: String, strict: bool) {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("line: {}", line);
        // match serde_json::from_str::<LogRecord>(&line) {
        //     Ok(r) => {
        //         if r.level >= level_filter {
        //             print!("{}", r.format(format))
        //         }
        //     }
        //     Err(_) => {
        //         if !strict {
        //             println!("{}", line)
        //         }
        //     }
        // }
    }
}
