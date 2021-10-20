use ggez::*;
use oorandom::Rand32;
use rand::*;

struct State {
    shapes: Vec<Shape>,
}

enum Shape {
    Circle(mint::Point2<f32>, f32, graphics::Color),
    Rectangle(graphics::Rect, graphics::Color),
    Line(mint::Point2<f32>, mint::Point2<f32>, f32, graphics::Color),
    Triangle(
        mint::Point2<f32>,
        mint::Point2<f32>,
        mint::Point2<f32>,
        graphics::Color,
    ),
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for shape in &self.shapes {
            // Make the shape...
            let mesh = match shape {
                &Shape::Rectangle(rect, color) => {
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?
                }
                &Shape::Circle(origin, radius, color) => graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    origin,
                    radius,
                    0.1,
                    color,
                )?,
                &Shape::Line(p1, p2, width, color) => {
                    graphics::Mesh::new_line(ctx, &[p1, p2], width, color)?
                }
                &Shape::Triangle(p1, p2, p3, color) => graphics::Mesh::new_polygon(
                    ctx,
                    graphics::DrawMode::fill(),
                    &[p1, p2, p3],
                    color,
                )?,
            };
            // ...and then draw it.
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();

    let (ctx, event_loop) = ContextBuilder::new("ggez_game.", "fuck_you")
        .default_conf(c)
        .build()
        .unwrap();

    let mut shapes = Vec::new();
    for _ in 0..5 {
        let mut rng1 = rand::thread_rng();

        let mut rng = Rand32::new(rng1.gen_range(0..100)); // Random number chosen by fair die roll
        shapes.push(Shape::Rectangle(
            ggez::graphics::Rect::new(
                rng.rand_float() * 800.0,
                rng.rand_float() * 600.0,
                rng.rand_float() * 800.0,
                rng.rand_float() * 600.0,
            ),
            graphics::Color {
                r: rng1.gen(),
                g: rng1.gen(),
                b: rng1.gen(),
                a: rng1.gen(),
            },
        ));
        shapes.push(Shape::Triangle(
            mint::Point2 {
                x: rng.rand_float() * 800.0,
                y: rng.rand_float() * 600.0,
            },
            mint::Point2 {
                x: rng.rand_float() * 800.0,
                y: rng.rand_float() * 600.0,
            },
            mint::Point2 {
                x: rng.rand_float() * 800.0,
                y: rng.rand_float() * 600.0,
            },
            graphics::Color {
                r: rng1.gen(),
                g: rng1.gen(),
                b: rng1.gen(),
                a: rng1.gen(),
            },
        ));
        shapes.push(Shape::Line(
            mint::Point2 {
                x: rng.rand_float() * 800.0,
                y: rng.rand_float() * 600.0,
            },
            mint::Point2 {
                x: rng.rand_float() * 800.0,
                y: rng.rand_float() * 600.0,
            },
            rng.rand_float() * 20.0,
            graphics::Color {
                r: rng1.gen(),
                g: rng1.gen(),
                b: rng1.gen(),
                a: rng1.gen(),
            },
        ));
        shapes.push(Shape::Circle(
            mint::Point2 {
                x: rng.rand_float() * 800.0,
                y: rng.rand_float() * 600.0,
            },
            rng.rand_float() * 300.0,
            graphics::Color {
                r: rng1.gen(),
                g: rng1.gen(),
                b: rng1.gen(),
                a: rng1.gen(),
            },
        ));
    }
    println!("{}", shapes.len());

    let state = State { shapes };
    event::run(ctx, event_loop, state);
}
