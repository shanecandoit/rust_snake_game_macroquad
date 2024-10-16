use macroquad::prelude::*;

const SNAKE_SIZE: f32 = 20.0;
const MOVE_SPEED: f32 = 20.0;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<Vec2>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        Self {
            body: vec![vec2(100.0, 100.0)],
            direction: Direction::Right,
        }
    }

    fn update(&mut self) {
        let mut new_head = *self.body.first().unwrap();

        match self.direction {
            Direction::Up => new_head.y -= MOVE_SPEED,
            Direction::Down => new_head.y += MOVE_SPEED,
            Direction::Left => new_head.x -= MOVE_SPEED,
            Direction::Right => new_head.x += MOVE_SPEED,
        }

        self.body.insert(0, new_head);
        self.body.pop();
    }

    fn grow(&mut self) {
        let tail = *self.body.last().unwrap();
        self.body.push(tail);
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if (self.direction == Direction::Up && new_direction != Direction::Down)
            || (self.direction == Direction::Down && new_direction != Direction::Up)
            || (self.direction == Direction::Left && new_direction != Direction::Right)
            || (self.direction == Direction::Right && new_direction != Direction::Left)
        {
            self.direction = new_direction;
        }
    }

    fn draw(&self) {
        for segment in &self.body {
            draw_rectangle(segment.x, segment.y, SNAKE_SIZE, SNAKE_SIZE, GREEN);
        }
    }
}

struct Food {
    position: Vec2,
}

impl Food {
    fn new() -> Self {
        Self {
            position: vec2(
                (rand::gen_range(0, screen_width() as i32 / SNAKE_SIZE as i32) * SNAKE_SIZE as i32) as f32,
                (rand::gen_range(0, screen_height() as i32 / SNAKE_SIZE as i32) * SNAKE_SIZE as i32) as f32,
            ),
        }
    }

    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, SNAKE_SIZE, SNAKE_SIZE, RED);
    }
}

struct Game {
    snake: Snake,
    food: Vec<Food>,
}

impl Game {
    fn new() -> Self {
        
        let snake= Snake::new();
        let mut food= vec![Food::new()];
        food.push(Food::new());
        food.push(Food::new());
        Self {
            snake,
            food,
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.snake.change_direction(Direction::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            self.snake.change_direction(Direction::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            self.snake.change_direction(Direction::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            self.snake.change_direction(Direction::Right);
        }
    }

    fn update(&mut self) {
        self.snake.update();

        let mut new_food_positions = vec![];
        let mut eaten_food_indices = vec![];

        for (i, food) in self.food.iter().enumerate() {
            if self.snake.body.first().unwrap().distance(food.position) < SNAKE_SIZE {
                self.snake.grow();
                eaten_food_indices.push(i);
                new_food_positions.push(Food::new());
            }
        }

        // Remove eaten food
        for &index in eaten_food_indices.iter().rev() {
            self.food.remove(index);
        }

        // Add new food
        for new_food in new_food_positions {
            self.food.push(new_food);
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        self.snake.draw();
        for food in &self.food {
            food.draw();
        }
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game::new();

    loop {
        limit_fps(10);
        game.handle_input();
        game.update();
        game.draw();

        next_frame().await;
    }
}

fn limit_fps(fps: u32) {
    
    let minimum_frame_time = 1. / fps as f32; // 60 FPS
    let frame_time = get_frame_time();
    // println!("Frame time: {}ms", frame_time * 1000.);
    if frame_time < minimum_frame_time {
        let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        // println!("Sleep for {}ms", time_to_sleep);
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
}