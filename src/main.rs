use std::thread::sleep;
use rand::Rng;
use std::process::Command;
use std::time::Duration;
use crossterm::event::{read,Event,KeyCode};
/// 定义墙的宽高
const WIDTH:usize = 50;
const HEIGHT:usize = 20;
// 地图
type Map = [[char; WIDTH]; HEIGHT];

fn main() {
    // 初始化地图
    let mut map: Map = create_map();
    // 初始化蛇
    let mut snake = create_snake();
    // 初始化食物
    let mut food = create_food();
    // 初始方向
    let mut current_direction = Direction::Right;
    loop {
        clear_screen();
        // 添加蛇到地图
        add_snake_to_map(&mut snake, &mut map);
        // 添加食物到地图
        add_food_to_map(&mut food, &mut map);
        // 随机生成食物
        random_new_food(&mut food, &snake);
        // 打印地图
        print_map(&map);
        current_direction = control_snake(&mut snake, current_direction, &mut food, &mut map);
        // 判断游戏是否结束
        if !is_game_over(&snake){
            println!("Game Over!");
            break;
        }
        sleep(Duration::from_millis(snake.speed as u64));

    }
    Command::new("cmd.exe").arg("/c").arg("pause").status().expect("clear error!");
}

/// 蛇
struct Snake {
    // 蛇头
    head: (usize, usize),
    // 蛇身
    body: Vec<(usize, usize)>,
    // 移动速度
    speed: usize,
}
/// 食物
struct Food {
    position: (usize, usize),
    eated: bool,
}
/// 创建食物
fn create_food() -> Food {
    Food {
        position: (7, 8),
        eated: false,
    }
}
/// 创建蛇
fn create_snake() -> Snake {
    Snake {
        head: (5, 6),
        body: vec![(4, 6), (3, 6)],
        speed: 300,
    }
}
/// 随机生成食物
fn random_new_food(food: &mut Food,snake: &Snake){
    // 判断食物是否被吃
    if !food.eated {
        return
    }
    let mut rng = rand::thread_rng();
    // 生成随机位置
    food.position = (rng.gen_range(1..WIDTH-1), rng.gen_range(1..HEIGHT-1));
    // 判断是否与蛇身重叠
    while is_food_collide_with_snake(food, snake) {
        random_new_food(food, snake);
    }
    food.eated = false;
}
/// 判断食物与蛇身是否重叠
fn is_food_collide_with_snake(food: &Food, snake: &Snake) -> bool {
    if food.position == snake.head || snake.body.contains(&food.position) {
        return true;
    }
    false
}
/// 添加食物到地图
fn add_food_to_map(food: &Food, map: &mut Map) {
    if food.eated {
        return
    }
    map[food.position.0][food.position.1] = '♥';
}
/// 添加蛇到地图
fn add_snake_to_map(snake: &Snake, map: &mut Map) {
    map[snake.head.0][snake.head.1] = '●';
    for body_part in &snake.body {
        map[body_part.0][body_part.1] = '▣';
    }
}
/// 创建地图
fn create_map() -> Map {
    let block = '■';
    let empty = ' ';
    let mut map: Map = [[empty; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 {
                map[i][j] = block;
            }
        }
    }
    map
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/// 移动蛇
fn move_snake(snake: &mut Snake, direction: Direction,food: &mut Food,map: &mut Map) {
    let before_head = snake.head;
    let mut eated = false;
    match direction {
        Direction::Up => {
            snake.head.0 -= 1;
            eated = snake.head == food.position;
            map[snake.head.0][snake.head.1] = '●';
            map[before_head.0][before_head.1] = '▣';
        },
        Direction::Left => {
            snake.head.1 -= 1;
            eated = snake.head == food.position;
            map[snake.head.0][snake.head.1] = '●';
            map[before_head.0][before_head.1] = '▣';
        },
        Direction::Right => {
            snake.head.1 += 1;
            eated = snake.head == food.position;
            map[snake.head.0][snake.head.1] = '●';
            map[before_head.0][before_head.1] = '▣';
        },
        Direction::Down => {
            snake.head.0 += 1;
            eated = snake.head == food.position;
            map[snake.head.0][snake.head.1] = '●';
            map[before_head.0][before_head.1] = '▣';
        }
    }
    // 没吃到，删掉最后一个
    if !eated {
        let tail = snake.body.remove(snake.body.len() - 1);
        map[tail.0][tail.1] = ' ';
    }
    // 吃到，不用删除
    food.eated = eated;
    snake.body.insert(0, before_head);
}

/// 清屏
fn clear_screen() {
    Command::new("cmd.exe").arg("/c").arg("cls").status().expect("clear error!");
}

/// 键盘控制
fn control_snake(snake: &mut Snake,dir:Direction,food: &mut Food,map: &mut Map) -> Direction {
    let mut direction = dir;
    if let Ok(Event::Key(event)) = read() {
        direction = match event.code {
            KeyCode::Char('w') => Direction::Up,
            KeyCode::Char('s') => Direction::Down,
            KeyCode::Char('a') => Direction::Left,
            KeyCode::Char('d') => Direction::Right,
            _ => dir
        };
    }
    move_snake(snake, direction, food, map);
    direction
}

/// 判断游戏结束
fn is_game_over(snake: &Snake) -> bool {
    for i in 0..snake.body.len() {
        if snake.body[i] == snake.head {
            return false
        }
    }
    true
}
/// 打印地图
fn print_map(map: &Map) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", map[i][j]);
        }
        println!();
    }
}