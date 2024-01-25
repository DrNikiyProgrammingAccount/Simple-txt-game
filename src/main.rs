use std::io;

fn main() {
    let mut map: Vec<Vec<char>> = vec![
        vec![' ', ' ', ' ', ' ', '*', '*'],
        vec![' ', '0', '*', ' ', '$', '*'],
        vec![' ', '0', '0', '*', ' ', ' '],
        vec![' ', ' ', '$', '0', ' ', ' '],
        vec![' ', ' ', '*', ' ', '$', '0'],
        vec!['&', ' ', ' ', '*', '0', '0']
    ];
    let mut pos= (0,(map.len()-1) as i32);
    let mut lives = (3,0);
    let mut coins = (0,3);
    println!("Привео)))");
    println!("Передвижение: WASD, выйти на Q");
    println!("Задача: Собрать все монетки");
    println!("& - Персонаж");
    println!("* - Ловушка");
    println!("$ - Монетка");
    println!("0 - Стена");

    fn movement (map: &mut Vec<Vec<char>>, dir: (i32, i32), pos: &mut (i32, i32), lives: &mut (i32, i32), coins: &mut (i32, i32)) {
        if dir.1+pos.1 < 0 || dir.0+pos.0 < 0 || dir.1+pos.1 >= map.len() as i32 || dir.0+pos.0 >= map[0].len() as i32 {
            println!("Вы улетели за край карты, осторожнее! Попробуйте ещё раз");
            return;
        }
        match map[(dir.1+pos.1) as usize][(dir.0+pos.0) as usize] {
            ' ' => {change_pos(map, dir, pos)},
            '0' => println!("Тут стена, сюда нельзя! Попробуйте ещё раз"),
            '*' => {change_pos(map, dir, pos); lives.0 -= 1; lives.1 += 1},
            '$' => {change_pos(map, dir, pos); coins.0 += 1; coins.1 -= 1},
            _ => panic!("Проблема с картой")
        }
        fn change_pos (map: &mut Vec<Vec<char>>, dir: (i32, i32), pos: &mut (i32, i32)) {
            map[(pos.1) as usize][(pos.0) as usize] = ' ';
            map[(dir.1+pos.1) as usize][(dir.0+pos.0) as usize] = '&';
            pos.1 = dir.1+pos.1; pos.0 = dir.0+pos.0
        }
    }
    loop {
        for i in &map {
            println!("{:?}", i);
        }
        println!("Осталось жизней: {}", lives.0);
        println!("Монет собрано: {}", coins.0);
        println!("Монет осталось: {}", coins.1);
        if lives.0 == 0 {
            println!("Вы умерли :(");
            break;
        }
        if coins.1 == 0 {
            println!("Вы выиграли!");
            break;
        }
        loop {
            let mut input = String::new();
            break match io::stdin().read_line(&mut input) {
                Ok(_x) => match input.trim().to_ascii_uppercase().as_str() {
                    "W" => movement(&mut map, (0,-1), &mut pos, &mut lives, &mut coins),
                    "S" => movement(&mut map, (0,1), &mut pos, &mut lives, &mut coins),
                    "A" => movement(&mut map, (-1,0), &mut pos, &mut lives, &mut coins),
                    "D" => movement(&mut map, (1,0), &mut pos, &mut lives, &mut coins),
                    "Q" => std::process::exit(0),
                    _ => {println!("Введите корректный символ! {}", input.to_ascii_uppercase().as_str()); continue}
                },
                Err(x) => { println!("Ошибка: {}. Попробуйте повторить ввод", x); continue }
            }
        }
    }
}