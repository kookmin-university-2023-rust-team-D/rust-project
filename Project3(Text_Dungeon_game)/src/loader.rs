use std::fs::File;
use std::io::{BufRead, BufReader, Result, Lines};
use rand::Rng;

pub fn open_file(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub struct Player {
    pub hp: i32,
    pub gold: i32,
    pub weapon: String,
    pub armor: String
}

impl From<&str> for Player {
    fn from(filename: &str) -> Self {
        let lines = open_file(filename);

        let mut lines = match  lines{
            Ok(lines) => lines,
            Err(err) => panic!("파일을 여는 도중에 문제가 생겼습니다! {:?}", err),
        };
        let line = lines.next().expect("파일을 읽을 수 없습니다!");
        let status_str = match line {
            Ok(val) => val,
            Err(_err) => panic!("파일을 읽을 수 없습니다!"),
            };
            
        let mut status = status_str.split_whitespace().map(|s| s);

        let hp = status.next().unwrap().parse::<i32>().unwrap();
        let gold = status.next().unwrap().parse::<i32>().unwrap();
        let weapon = status.next().unwrap();
        let armor = status.next().unwrap();

        Player {
            hp,
            gold,
            weapon: weapon.to_string(),
            armor: armor.to_string(),
        }
    }
}

pub struct Monster {
    pub name: String,
    pub hp: i32,
    pub damage: i32,
    pub gold: i32
}

impl From<&str> for Monster {
    fn from(filename: &str) -> Self {
        let mut randum = rand::thread_rng();
        let num_monster = randum.gen_range(0..=4);
        let lines = open_file(filename);

        // let lines = open_file(filename);

        let mut lines = match lines{
            Ok(lines) => lines,
            Err(err) => panic!("파일을 여는 도중에 문제가 생겼습니다! {:?}", err),
        };

        let line = lines.nth(num_monster).expect("파일을 읽을 수 없습니다!");
        let status_str = match line {
            Ok(val) => val,
            Err(_err) => panic!("파일을 읽을 수 없습니다!"),
            };
            
        let mut status = status_str.split_whitespace().map(|s| s);

        let name = status.next().unwrap();
        let hp = status.next().unwrap().parse().unwrap();
        let damage = status.next().unwrap().parse().unwrap();
        let gold = status.next().unwrap().parse().unwrap();

        Monster {
            name : name.to_string(),
            hp,
            damage,
            gold,
        }
    }
}