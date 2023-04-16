use rand::Rng;
use std::io::{self, Write};

struct Player {
    hp: i32,
    gold: i32,
    weapon: String,
    armor: String,
}

struct Monster {
    hp: i32,
    gold: i32,
    damage: i32,
}

enum Event {
    Combat(Monster),
    Shop,
    Special,
    Rest,
}

fn main() {
    let mut rng = rand::thread_rng();

    let player = Player {
        hp: 100,
        gold: 0,
        weapon: String::from("검"),
        armor: String::from("갑옷"),
    };

    let mut events = vec![];
    let num_events = rng.gen_range(5..=10);

    for _ in 0..num_events {
        let event_type = rng.gen_range(0..4);
        let event = match event_type {
            0 => {
                let monster = Monster {
                    hp: rng.gen_range(50..=100),
                    gold: rng.gen_range(10..=30),
                    damage: rng.gen_range(5..=15),
                };
                Event::Combat(monster)
            }
            1 => Event::Shop,
            2 => Event::Special,
            3 => Event::Rest,
            _ => unreachable!(),
        };
        events.push(event);
    }
    events.push(Event::Combat(Monster {
        hp: 200,
        gold: 100,
        damage: 20,
    }));

    let mut current_event = 0;
    while current_event < events.len() {
        match &events[current_event] {
            Event::Combat(monster) => {
                println!("몬스터를 만났습니다!");
                loop {
                    println!("플레이어 HP: {}, 골드: {}", player.hp, player.gold);
                    println!("몬스터 HP: {}", monster.hp);
                    print!("어떤 행동을 하시겠습니까? (공격/도망) ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    if input.trim() == "공격" {
                        let damage = rng.gen_range(5..=15);
                        println!("플레이어가 {}의 피해를 입혔습니다!", damage);
                        monster.hp -= damage;

                        if monster.hp <= 0 {
                            println!("몬스터를 물리쳤습니다!");
                            player.gold += monster.gold;
                            break;
                        }

                        let damage = rng.gen_range(5..=15);
                        println!("몬스터가 {}의 피해를 입혔습니다!", damage);
                        player.hp -= damage;

                        if player.hp <= 0 {
                            println!("플레이어가 사망했습니다!");
                            return;
                        }
                    } else if input.trim() == "도망" {
                        println!("플레이어가 도망쳤습니다!");
                        break;
                    } else {
                        println!("잘못된 입력입니다. 다시 입력해주세요.");
                    }
                }
            }
            Event::Shop => {
                println!("상점을 발견했습니다!");
                println!("현재 골드: {}", player.gold);
                println!("무기: {}, 가격: 50", player.weapon);
                println!("방어구: {}, 가격: 50", player.armor);
                print!("구매할 아이템을 선택해주세요. (무기/방어구/나가기) ");
                io::stdout().flush().unwrap();
    
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
    
                if input.trim() == "무기" {
                    if player.gold >= 50 {
                        println!("무기를 구매했습니다!");
                        player.weapon = String::from("활");
                        player.gold -= 50;
                    } else {
                        println!("골드가 부족합니다!");
                    }
                } else if input.trim() == "방어구" {
                    if player.gold >= 50 {
                        println!("방어구를 구매했습니다!");
                        player.armor = String::from("갑옷");
                        player.gold -= 50;
                    } else {
                        println!("골드가 부족합니다!");
                    }
                } else if input.trim() == "나가기" {
                    println!("상점에서 나왔습니다.");
                } else {
                    println!("잘못된 입력입니다. 다시 입력해주세요.");
                }
            }
            Event::Special => {
                println!("특별 이벤트를 발견했습니다!");
                println!("특별 이벤트를 클리어했습니다.");
            }
            Event::Rest => {
                println!("휴식을 취할 수 있는 장소를 발견했습니다.");
                println!("플레이어 HP: {}", player.hp);
                player.hp += 20;
                println!("체력이 20 회복되었습니다. 현재 체력: {}", player.hp);
            }
        }
        current_event += 1;
    }
    println!("보스를 만났습니다!");
    let boss = Monster {
        hp: 300,
        gold: 200,
        damage: 30,
    };
    loop {
        println!("플레이어 HP: {}, 골드: {}", player.hp, player.gold);
        println!("보스 HP: {}", boss.hp);
        print!("어떤 행동을 하시겠습니까? (공격/도망) ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    
        if input.trim() == "공격" {
            let damage = rng.gen_range(5..=15);
            println!("플레이어가 {}의 피해를 입혔습니다!", damage);
            boss.hp -= damage;
    
            if boss.hp <= 0 {
                println!("보스를 물리쳤습니다! 플레이어의 승리!");
                return;
            }
    
            let damage = rng.gen_range(5..=15);
            println!("보스가 {}의 피해를 입혔습니다!", damage);
            player.hp -= damage;
    
            if player.hp <= {
                println!("플레이어가 사망했습니다. 게임 오버!");
                return;
            
        }
        } else if input.trim() == "도망" {
        let success = rng.gen_bool(0.5);
        if success {
            println!("플레이어가 도망쳤습니다.");
            return;
        } else {
            println!("도망에 실패했습니다. 보스의 공격을 받습니다.");
            let damage = rng.gen_range(5..=15);
            println!("보스가 {}의 피해를 입혔습니다!", damage);
            player.hp -= damage;

            if player.hp <= 0 {
                println!("플레이어가 사망했습니다. 게임 오버!");
                return;
            }
        }
    } else {
        println!("잘못된 입력입니다. 다시 입력해주세요.");
    }
}
