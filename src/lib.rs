use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, BLACK, WHITE};
use ggez::{Context, GameResult};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const RECTANGLE_WIDTH: f32 = 96.0;

pub struct MainState {
    background_color: Color,
    heights: Vec<f32>,
    meshes: Vec<Mesh>,
    colors: Vec<Color>,
    rng: ThreadRng,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let mut rng = thread_rng();
        let background_color = BLACK;
        let heights = vec![0.0; 20];
        let meshes = vec![
            MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    Rect::new(0.0, 0.0, RECTANGLE_WIDTH, 10.0),
                    WHITE
                )
                .build(context)?;
            20
        ];
        let mut colors = vec![];

        for _ in 0..20 {
            let random_color = Color::from_rgb(
                rng.gen_range(200..255),
                rng.gen_range(200..255),
                rng.gen_range(200..255),
            );
            colors.push(random_color);
        }

        Ok(Self {
            background_color,
            heights,
            meshes,
            colors,
            rng,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let (_screen_width, screen_height) = graphics::drawable_size(context);

        let random_index = self.rng.gen_range(0..self.heights.len());
        self.heights[random_index] += 1.0;

        for index in 0..self.meshes.len() {
            let height = self.heights[index];
            let start_x = index as f32 * RECTANGLE_WIDTH;
            let start_y = screen_height - height;
            let rect = Rect::new(start_x, start_y, RECTANGLE_WIDTH, height);
            let color = self.colors[index];
            let mesh = MeshBuilder::new()
                .rectangle(DrawMode::fill(), rect, color)
                .rectangle(DrawMode::stroke(2.0), rect, BLACK)
                .build(context)?;
            self.meshes[index] = mesh;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        for mesh in &self.meshes {
            graphics::draw(context, mesh, DrawParam::new())?;
        }

        graphics::present(context)
    }
}
