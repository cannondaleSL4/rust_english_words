use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::collections::HashMap;

#[derive(Debug)]
struct Translate {
    value:Vec<String>
}

impl Translate {
    fn translate(&self) -> String {
        // let vv: String = self.value.concat();
        // return vv;
        return self.value.concat();
    }
}

pub fn read_file() -> Result<(), Error> {
    let file = File::open("words")?;
    let reader = BufReader::new(file);

    let mut map: HashMap<String, Translate> = HashMap::new();

    for line in reader.lines() {
        let linez = line?;
        map = read_words(linez, map);
    }

    Ok(())
}

fn read_words(line: String, map: HashMap<String, Translate>) -> HashMap<String, Translate> {
    println!("{}", line);
    return map;
    // map.insert("test", "test")
}