use std::io;

#[derive(Debug)]
struct GameConfiguration {
    min: u32,
    max: u32
}

struct GameState<'a> {
    config: & 'a GameConfiguration,
    numTries: u32,
    number: u32
}

impl GameConfiguration {
    
    fn read_from_stdin() -> GameConfiguration {
        let buf = &mut String::new();
        let mut reader = io::stdin();

        println!("min number: ");
        
        reader.read_line(buf).ok().expect("fail");
        println!("read {}", buf);

        let min_nr: u32 = buf.trim().parse::<u32>().ok().expect("fail");

        buf.truncate(0);

        println!("max number: ");
        reader.read_line(buf).ok().expect("fail");
        println!("read {}", buf);
        let max_nr: u32 = buf.trim().parse::<u32>().ok().expect("fail");

        return GameConfiguration { min: min_nr, max: max_nr };
    }
}

fn main() {

    let config = GameConfiguration::read_from_stdin();

    println!("{:?}", config);
     
}
