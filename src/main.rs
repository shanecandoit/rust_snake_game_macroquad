use macroquad::prelude::*;
// use macroquad::window::quit;

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
    // Create a new snake with a single segment
    // at some position
    fn new() -> Self {
        let random_x = rand::gen_range(
            0,
            (screen_width() / 20f32)
                as i32
                * 20,
        ) as f32;
        let random_y = rand::gen_range(
            0,
            (screen_height() / 20f32)
                as i32
                * 20,
        ) as f32;
        let random_dir =
            rand::gen_range(0, 4);
        let direction = match random_dir
        {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::Right,
        };
        Self {
            body: vec![vec2(
                random_x, random_y,
            )],
            direction,
        }
    }

    // Update the snake's position based on the current direction
    fn update(
        &mut self,
        move_speed: f32,
    ) {
        let mut new_head =
            *self.body.first().unwrap();

        match self.direction {
            Direction::Up => {
                new_head.y -= move_speed
            }
            Direction::Down => {
                new_head.y += move_speed
            }
            Direction::Left => {
                new_head.x -= move_speed
            }
            Direction::Right => {
                new_head.x += move_speed
            }
        }

        self.body.insert(0, new_head);
        self.body.pop();
    }

    // Add a new segment to the snake's body
    fn grow(&mut self) {
        let tail =
            *self.body.last().unwrap();
        self.body.push(tail);
    }

    // Change the snake's direction, but only if the new direction
    // is not the opposite of the current direction
    fn change_direction(
        &mut self,
        new_direction: Direction,
    ) {
        if (self.direction
            == Direction::Up
            && new_direction
                != Direction::Down)
            || (self.direction
                == Direction::Down
                && new_direction
                    != Direction::Up)
            || (self.direction
                == Direction::Left
                && new_direction
                    != Direction::Right)
            || (self.direction
                == Direction::Right
                && new_direction
                    != Direction::Left)
        {
            self.direction =
                new_direction;
        }
    }

    // Draw the snake's body
    fn draw(
        &self,
        snake_size: f32,
    ) {
        for segment in &self.body {
            draw_rectangle(
                segment.x, segment.y,
                snake_size, snake_size,
                GREEN,
            );
        }
    }
}

struct Food {
    position: Vec2,
}

impl Food {
    // Create a new food at a random position
    fn new(snake_size: f32) -> Self {
        Self {
            position: vec2(
                (rand::gen_range(
                    0,
                    screen_width()
                        as i32
                        / snake_size
                            as i32,
                ) * snake_size as i32)
                    as f32,
                (rand::gen_range(
                    0,
                    screen_height()
                        as i32
                        / snake_size
                            as i32,
                ) * snake_size as i32)
                    as f32,
            ),
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
    move_speed: f32,
    score: i32,
    starting_food_count: i32,
}

impl Game {
    fn new() -> Self {
        let snake_size = 20.0;
        let move_speed = 10.0;
        let score = 0;
        let starting_food_count = 100;

        let snake = Snake::new();

        // create food
        let mut food = vec![];
        for _ in 0..starting_food_count
        {
            food.push(Food::new(
                snake_size,
            ));
        }

        Self {
            snake,
            food,
            snake_size,
            move_speed,
            score,
            starting_food_count,
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.snake
                .change_direction(
                    Direction::Up,
                );
        }
        if is_key_pressed(KeyCode::Down)
        {
            self.snake
                .change_direction(
                    Direction::Down,
                );
        }
        if is_key_pressed(KeyCode::Left)
        {
            self.snake
                .change_direction(
                    Direction::Left,
                );
        }
        if is_key_pressed(
            KeyCode::Right,
        ) {
            self.snake
                .change_direction(
                    Direction::Right,
                );
        }
        if is_key_released(
            KeyCode::Escape,
        ) {
            //close the game
            // quit();
            // exit(0);
            std::process::exit(0);
        }
    }

    fn update(&mut self) {
        self.snake
            .update(self.move_speed);

        let mut new_food_positions =
            vec![];
        let mut eaten_food_indices: Vec<usize> = vec![];

        for (i, food) in
            self.food.iter().enumerate()
        {
            // snake eat food?
            if self
                .snake
                .body
                .first()
                .unwrap()
                .distance(food.position)
                < self.snake_size
            {
                self.snake.grow();
                eaten_food_indices
                    .push(i);
                new_food_positions
                    .push(Food::new(
                        self.snake_size,
                    ));
                self.score += 1;
            }
        }

        // Remove eaten food
        for &index in eaten_food_indices
            .iter()
            .rev()
        {
            self.food.remove(index);
        }

        // Add new food
        for new_food in
            new_food_positions
        {
            self.food.push(new_food);
        }

        // Check if snake is out of bounds
        let mut reset = false;
        let head = self
            .snake
            .body
            .first()
            .unwrap();
        if head.x < 0.0
            || head.x > screen_width()
            || head.y < 0.0
            || head.y > screen_height()
        {
            reset = true;
        }

        // reset score
        if reset {
            let new_game = Game::new();
            self.snake = new_game.snake;
            self.food = new_game.food;
            self.score = new_game.score;
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        self.snake
            .draw(self.snake_size);
        for food in &self.food {
            food.draw(self.snake_size);
        }

        // Draw the score
        draw_text(
            &format!(
                "Score: {}",
                self.score
            ),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
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
    let minimum_frame_time =
        1. / fps as f32; // 60 FPS
    let frame_time = get_frame_time();
    // println!("Frame time: {}ms", frame_time * 1000.);
    if frame_time < minimum_frame_time {
        let time_to_sleep =
            (minimum_frame_time
                - frame_time)
                * 1000.;
        // println!("Sleep for {}ms", time_to_sleep);
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
}
