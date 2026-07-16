mod monster;
mod player;

use monster::Monster;
use player::Player;

use rand::{Rng, RngExt};

fn hit_logic(luck: i32) -> i32 {
    if luck > 85 {
        return 2;
    } else {
        return 1;
    }
}

fn main() {
    println!("Как зовут твоего героя?");
    let mut name = String::new();
    //let - штука чтобы объявлять переменную
    //mut - разрешает перезаписать значение переменной
    //String - тип того, что будет храниться внутри переменную
    //через дабл колон вызываем функцию из типа данных string(также можно из модулей вызывать так)
    //new - функция внутри типа данных стринг, которая создаёт и возвращает пустой текст
    std::io::stdin()
        .read_line(&mut name)
        .expect("Не удалось прочитать строку");
    //str:io это модуль раста который за ввод и вывод отвечает
    //stdin - фнукция даёт доступ к вводу с клавиатуры
    //read_line - метод, ждёт пока юзер что-то напишет и нажмёт энтер
    //внутрь реадлайна передаём ссылку на переменную
    //: используется для указания типов данных, а :: для разделения путей
    let name = name.trim();
    //важно понимать, что метод .trim сделал из String в&str
    //это ВРЕМЕНННАЯ ссылка, но нам для нашей структуры нужно для имени String
    println!("Привет, {name}! Игра Rust RPG приветствует тебя!");

    let mut rng = rand::rng();

    let mut player = Player {
        name: name.to_string(),
        hp: 100,
        damage: 15,
        luck: rng.random_range(0..=100),
    };
    //объявили мутабельную переменнную, туда мы кладём нового перса на основе структуры
    //имя из за того что писал выше мы преобразуем вString из &str
    //делаем мутабельной чтобы потом менять хп допустим или баффы давать ему
    println!(
        "Создали героя: {}, его здоровье: {}, а атака: {}",
        player.name, player.hp, player.damage
    );

    let mut monster = Monster {
        name: "Олег Швабров".to_string(),
        hp: 200,
        damage: 20,
        rarity: "ordinary".to_string(),
        luck: rng.random_range(0..=100),
    };

    let max_rand = monster.damage;
    let min_rand = 0;
    println!(
        "Вы встретили монстра, его величают {}, атака у него {}, а жизней {}...",
        monster.name, monster.damage, monster.hp
    );
    let mut turn = 0;
    let mut is_my_turn: bool = rng.random();
    println!("Бой начался!");
    loop {
        println!(
            "Ваша атака: {}, здоровье: {}, удача(макс 100): {}",
            player.damage, player.hp, player.luck
        );
        println!(
            "Атака {}: {}, здоровье: {}, удача(макс 100): {}",
            monster.name, monster.damage, monster.hp, monster.luck
        );
        turn = turn + 1;
        println!("--------------------");
        println!("{} ход", turn);

        if is_my_turn {
            println!("--------------------");
            println!("Ваш ход");
            println!("Что будешь делать?");
            println!("1. Со всей силы пробить в живот");
            if turn <= 1 {
                println!(
                    "2. Сдаться и начать использовать Текстовые квесты играй и пиши[доступно только в 1 ход]"
                );
            }

            println!("Введите цифру:");
            let mut choice = String::new();
            std::io::stdin()
                .read_line(&mut choice)
                .expect("Ошибка чтения строки");
            let choice = choice.trim();
            let crit = hit_logic(player.luck);
            match choice {
                "1" => {
                    if crit == 2 {
                        println!(
                            "Вот это удача!!! Критичиская атака даёт бонус x{} к атаке {}",
                            crit, player.damage
                        );
                    }

                    monster.hp = monster.hp - player.damage * crit;
                    println!(
                        "{} ОБРУШИЛ УДАР на {}, который стоил оставил ему {} жизней",
                        player.name, monster.name, monster.hp
                    );
                    if monster.hp <= 0 {
                        println!(
                            "{} повежен, вы герой, вы его сломали, молодец, вы король",
                            monster.name,
                        );
                        break;
                    }
                    is_my_turn = false;
                }
                "2" => {
                    if turn <= 1 {
                        player.hp = player.hp - monster.damage * hit_logic(monster.luck);
                        println!(
                            "{} трусливо побежал, но {} успел ударить вдогонку",
                            player.name, monster.name
                        );
                        break;
                    } else {
                        println!("Шалунишка, твой шанс сдаться уже был!")
                    }
                }
                _ => {
                    println!("Нужно либо 1 либо 2 ввести!!!!!!")
                }
            }
        } else {
            println!("--------------------");
            println!("Ход {}", monster.name);
            is_my_turn = true;
            let crit = hit_logic(monster.luck);
            if crit == 2 {
                println!("{} везёт и он выбивает критический удар", monster.name);
            }
            player.hp = player.hp - monster.damage * crit;
            println!(
                "{} бьёт {}, нанося урон и оставляя вам {} жизней",
                monster.name, player.name, player.hp
            );
            if player.hp <= 0 {
                println!("Вы умерли, {} оказался сильнее...", monster.name);
                break;
            }
        }
    }
    if player.hp <= 0 {
        println!("Попробуйте заново!")
    } else {
        println!("TODO: игра дальше")
    }
}

//i32 это 32 битное целое число
