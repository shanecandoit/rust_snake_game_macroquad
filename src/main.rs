use macroquad::prelude::*;

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
    speed: f32,
}

impl Snake {
    // Create a new snake with a single segment
    // at some position
    fn new(speed: f32) -> Self {
        let random_x = rand::gen_range(0., 20.);
        let random_y =
            rand::gen_range(0., (screen_height() as f32 / 20.) * 20.);
        let direction = Self::random_direction();
        Self {
            body: vec![vec2(random_x, random_y)],
            direction,
            speed,
        }
    }

    fn random_direction() -> Direction {
        match rand::gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::Right,
        }
    }

    // Update the snake's position based on the current direction
    fn update(&mut self) {
        let mut new_head = *self.body.first().unwrap();

        match self.direction {
            Direction::Up => new_head.y -= self.speed,
            Direction::Down => new_head.y += self.speed,
            Direction::Left => new_head.x -= self.speed,
            Direction::Right => new_head.x += self.speed,
        }

        self.body.insert(0, new_head);
        self.body.pop();

        if self.is_colliding_with_self() {
            // reset the snake
            self.body = vec![vec2(0., 0.)];
            self.direction = Self::random_direction();
            return;
        }
    }

    // check if the snake is colliding with itself
    fn is_colliding_with_self(&self) -> bool {
        let head = self.body.first().unwrap();
        for segment in self.body.iter().skip(1) {
            if head.distance(*segment) < 1.0 {
                return true;
            }
        }
        false
    }

    // Add a new segment to the snake's body
    fn grow(&mut self) {
        let tail = *self.body.last().unwrap();
        self.body.push(tail);
    }

    // Change the snake's direction, but only if the new direction
    // is not the opposite of the current direction
    fn change_direction(
        &mut self,
        new_direction: Direction,
    ) {
        match (self.direction, new_direction) {
            // invalid moves
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {
                // do not change direction
            }
            // valid moves, change direction
            _ => self.direction = new_direction,
        }
    }

    // Draw the snake's body
    fn draw(
        &self,
        snake_size: f32,
    ) {
        for segment in &self.body {
            draw_rectangle(segment.x, segment.y, snake_size, snake_size, GREEN);
        }
    }
}

struct Food {
    position: Vec2,
}

impl Food {
    // Create a new food at a random position
    fn new(snake_size: f32) -> Self {
        let x = rand::gen_range(0., screen_width() / snake_size) * snake_size;
        let y = rand::gen_range(0., screen_height() / snake_size) * snake_size;
        Self {
            position: vec2(x, y),
        }
    }

    // Draw the food
    fn draw(
        &self,
        snake_size: f32,
    ) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            snake_size,
            snake_size,
            RED,
        );
    }
}

struct Game {
    snake: Snake,
    food: Vec<Food>,

    snake_size: f32,
    // move_speed: f32,
    score: i32,
    // starting_food_count: i32,
}

impl Game {
    fn new(starting_food_count: Option<i32>) -> Self {
        let snake_size = 20.0;
        let score = 0;
        // let starting_food_count = 100;
        let starting_food_count = starting_food_count.unwrap_or(100);

        let snake = Snake::new(10.);

        // create food
        let mut food = vec![];
        for _ in 0..starting_food_count {
            food.push(Food::new(snake_size));
        }

        Self {
            snake,
            food,
            snake_size,
            score,
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
        if is_key_released(KeyCode::Escape) {
            //close the game
            // quit();
            // exit(0);
            std::process::exit(0);
        }
    }

    fn update(&mut self) {
        self.snake.update();

        let mut new_food_positions = vec![];
        let mut eaten_food_indices: Vec<usize> = vec![];

        for (i, food) in self.food.iter().enumerate() {
            // snake eat food?
            if self.snake.body.first().unwrap().distance(food.position)
                < self.snake_size
            {
                self.snake.grow();
                eaten_food_indices.push(i);
                new_food_positions.push(Food::new(self.snake_size));
                self.score += 1;
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

        // Check if snake is out of bounds
        let mut reset = false;
        let head = self.snake.body.first().unwrap();
        if head.x < 0.0
            || head.x > screen_width()
            || head.y < 0.0
            || head.y > screen_height()
        {
            reset = true;
        }

        // reset score
        if reset {
            let new_game = Game::new(None);
            self.snake = new_game.snake;
            self.food = new_game.food;
            self.score = new_game.score;
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        self.snake.draw(self.snake_size);
        for food in &self.food {
            food.draw(self.snake_size);
        }

        // Draw the score
        draw_text(&format!("Score: {}", self.score), 10.0, 20.0, 30.0, WHITE);
    }
}

#[macroquad::main("Snake")]
async fn main() {
    let mut game = Game::new(None);

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
        std::thread::sleep(std::time::Duration::from_millis(
            time_to_sleep as u64,
        ));
    }
}
