// 플레이어, 몬스터의 구조체를 정의합니다. hp가 0이 되면 죽습니다.
// 무기와 방어구를 만든다면 무기와 방어구의 구조체도 정의해야 할 수 있습니다.

use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Player {
    pub hp: i32,
    pub gold: i32,
    pub weapon: String,
    pub armor: String
}

impl From<&str> for Player {
    fn from(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines().map(|l| l.unwrap());

        let line = lines.next().unwrap();

        let mut status = line.split_whitespace().map(|s| s);

        let hp = status.next().unwrap().parse().unwrap();
        let gold = status.next().unwrap().parse().unwrap();
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

impl Player {
    pub fn attack(&self, monster: &mut Monster) {
        let mut rng = rand::thread_rng();
        let damage = rng.gen_range(5..=15);
        println !("플레이어가 {}의 피해를 입혔습니다!", damage);
        monster.hp -= damage;
        println !("몬스터의 체력: {}", monster.hp);
    }
    pub fn is_dead(&self) -> bool{
        return self.hp <= 0
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
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let line = reader.lines().nth(num_monster).expect("파일을 읽을 수 없습니다").unwrap();

        let mut status = line.split_whitespace().map(|s| s);

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

impl Monster {
    pub fn attack(&self, player: &mut Player) {
        println !("{}가 플레이어에게 {}의 피해를 입혔습니다!", self.name, self.damage);
        player.hp -= self.damage;
        println !("픓레이어의 체력: {}", player.hp);
    }
    pub fn is_dead(&self) -> bool{
        return self.hp <= 0
    }

}
