use nannou::io::walk_dir;
use nannou::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

const FILENAME: &str = "scene_3.jpg";

struct Model {
    interval: Duration,
    assets: PathBuf,
    index: usize,
    images: Vec<String>,
    texture: wgpu::Texture,
}

impl Model {
    pub fn inc_image(&mut self, app: &App) {
        self.index += 1;
        if self.index >= self.images.len() {
            self.index = 0;
            let mut images: Vec<String> = walk_dir(self.assets.join("images")).into_iter().map(|dir| {
        let path = dir.unwrap().into_path();
        format!("{}", path.display())
    }).filter(|p| p.ends_with(FILENAME))
        .collect();
            images.sort();
            self.images = images;
        }
        let img = &self.images[self.index];
        self.texture = wgpu::Texture::from_path(app, img).unwrap();
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.interval += update.since_last;
    if model.interval.as_secs() > 2 {
        model.interval = Duration::from_secs(0);
        model.inc_image(app);
    }
}

fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();
    app.set_loop_mode(LoopMode::rate_fps(60.0));

    let assets = app.assets_path().unwrap();

    let mut images: Vec<String> = walk_dir(assets.join("images")).into_iter().map(|dir| {
        let path = dir.unwrap().into_path();
        format!("{}", path.display())
    }).filter(|p| p.ends_with(FILENAME))
        .collect();

    images.sort();

    let texture = wgpu::Texture::from_path(app, &images[1]).unwrap();
    let mut model = Model { interval: Duration::from_secs(0),
        index:0, assets, texture, images };
    model.inc_image(app);
    model
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    draw.texture(&model.texture).wh(vec2(1980f32, 1080f32));
    draw.to_frame(app, &frame).unwrap();
}
