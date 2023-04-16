//chatGPT를 활용해 코드를 만들었습니다 모듈화가 안되어있는 점은 죄송합니다.
// rand 라이브러리를 빌려옵니다. 랜덤함수를 사용할 수 있습니다.
use rand::Rng;
use std::io::{self, Write};
mod loader;
// 이벤트는 총 4가지로 설정하였습니다.
// combat은 몬스터와의 전투, shop은 상점, special은 특별한 랜덤 이벤트, rest는 휴식 장소로 플레이어의 체력을 회복할 수 있습니다.
enum Event {
    Combat {
        monster : loader::Monster
    },
    Shop,
    Special,
    Rest
}

//메인 함수 시작
fn main() {
    let mut rng = rand::thread_rng();
    let mut player = loader::Player::from("./src/player.txt");
    
    // 이벤트를 결정하기 위해 벡터를 생성하여, 랜덤으로 이벤트를 삽입합니다.
    let mut events = vec![];
    let num_events = rng.gen_range(5..=10);

    // for 반복문과 match 함수를 통해 각 이벤트를 랜덤으로 삽입합니다.
    for _ in 0..num_events {
        let event_type = rng.gen_range(0..4);
        let event = match event_type {
            0 => {
                let monster = loader::Monster::from("./src/monster.txt");
                Event::Combat {
                    monster : monster
                }
            }
            1 => Event::Shop,
            2 => Event::Special,
            3 => Event::Rest,
            _ => unreachable !()
        };
        events.push(event);
    }
    events.push(Event::Combat {
        monster : loader::Monster {
            name: "middle Boss".to_string(),
            hp : 200,
            damage: 15,
            gold: 100
        }
    });
    
    let mut current_event = 0;
    while current_event < events.len() {
        match & mut events[current_event]{
            Event::Combat {                         // 몬스터와 만났을 때
                monster
            } => {
                println !("{}를 만났습니다!", monster.name);
                loop {
                    println !("플레이어 HP: {}, 골드: {}", player.hp, player.gold);
                    println !("몬스터 HP: {}", monster.hp);
                    print !("어떤 행동을 하시겠습니까? (공격/도망) ");
                    io::stdout()
                        .flush()
                        .unwrap();

                    let mut input = String::new();
                    io::stdin()
                        .read_line(& mut input)
                        .unwrap();

                    if input.trim() == "공격" {
                        let damage = rng.gen_range(5..=15);
                        println !("플레이어가 {}의 피해를 입혔습니다!", damage);
                        monster.hp -= damage;
                        println !("몬스터의 체력: {}", monster.hp);

                        if monster.hp <= 0 {
                            println !("몬스터를 물리쳤습니다!");
                            player.gold += monster.gold;
                            break;
                        }

                        println !("몬스터가 {}의 피해를 입혔습니다!", monster.damage);
                        player.hp -= monster.damage;

                        if player.hp <= 0 {
                            println !("플레이어가 사망했습니다!");
                            return;
                        }
                    } else if input.trim() == "도망" {
                        println !("플레이어가 도망쳤습니다!");
                        break;
                    } else {
                        println !("잘못된 입력입니다. 다시 입력해주세요.");
                    }
                    println !("");
                }
            }
            Event::Shop => { //상점에서
                println !("상점을 발견했습니다!");
                println !("현재 골드: {}", player.gold);
                println !("무기: {}, 가격: 50", player.weapon);
                println !("방어구: {}, 가격: 50", player.armor);
                print !("구매할 아이템을 선택해주세요. (무기/방어구/나가기) ");
                io::stdout()
                    .flush()
                    .unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(& mut input)
                    .unwrap();

                if input.trim() == "무기" {
                    if player.gold >= 50 {
                        println !("무기를 구매했습니다!");
                        player.weapon = String::from("활");
                        player.gold -= 50;
                    } else {
                        println !("골드가 부족합니다!");
                    }
                } else if input.trim() == "방어구" {
                    if player.gold >= 50 {
                        println !("방어구를 구매했습니다!");
                        player.armor = String::from("갑옷");
                        player.gold -= 50;
                    } else {
                        println !("골드가 부족합니다!");
                    }
                } else if input.trim() == "나가기" {
                    println !("상점에서 나왔습니다.");
                } else {
                    println !("잘못된 입력입니다. 다시 입력해주세요.");
                }
                println !("");

            }
            // 특별 이벤트는 아직 생성 못함
            Event::Special => {
                println !("특별 이벤트를 발견했습니다!");
                println !("특별 이벤트를 클리어했습니다.");
                println !("");

            }
            Event::Rest => { // 휴식장소에서는 플레이어의 hp 회복
                println !("휴식을 취할 수 있는 장소를 발견했습니다.");
                println !("플레이어 HP: {}", player.hp);
                player.hp += 20;
                println !("체력이 20 회복되었습니다. 현재 체력: {}", player.hp);
                println !("");

            }
        }
        current_event += 1;
    }
    //마지막은 보스를 만난다.
    println !("보스를 만났습니다!");
    let mut boss = loader::Monster{
        name: "Boss".to_string(),
        hp : 300,
        damage: 15,
        gold: 200
    };
    loop {
        println !("플레이어 HP: {}, 골드: {}", player.hp, player.gold);
        println !("보스 HP: {}", boss.hp);
        print !("어떤 행동을 하시겠습니까? (공격/도망) ");
        io::stdout()
            .flush()
            .unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(& mut input)
            .unwrap();

        if input.trim() == "공격" {
            let damage = rng.gen_range(10..=20);
            println !("플레이어가 {}의 피해를 입혔습니다!", damage);
            boss.hp -= damage;

            if boss.hp <= 0 {
                println !("보스를 물리쳤습니다! 플레이어의 승리!");
                return;
            }

            println !("보스가 {}의 피해를 입혔습니다!", boss.damage);
            player.hp -= boss.damage;

            if player.hp <= 0 {
                println !("플레이어가 사망했습니다. 게임 오버!");
                return;
            }
            println !("");
        } else if input.trim() == "도망" {
            let success = rng.gen_bool(0.5);
            if success {
                println !("플레이어가 도망쳤습니다.");
                println !("");
                return;
            } else {
                println !("도망에 실패했습니다. 보스의 공격을 받습니다.");
                let damage = rng.gen_range(10..=20);
                println !("보스가 {}의 피해를 입혔습니다!", damage);
                player.hp -= damage;

                if player.hp <= 0 {
                    println !("플레이어가 사망했습니다. 게임 오버!");
                    return;
                }
            }
        } else {
            println !("잘못된 입력입니다. 다시 입력해주세요.");
        }
    }
}
