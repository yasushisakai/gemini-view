use nannou::io::walk_dir;
use nannou::prelude::*;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

struct Model {
    interval: Duration,
    filename: String,
    assets: PathBuf,
    index: usize,
    images: Vec<PathBuf>,
    texture: Option<wgpu::Texture>,
}

impl Model {
    pub fn inc_image(&mut self, app: &App) {
        self.index += 1;

        if self.images.len() == 0 || self.index >= self.images.len() {
            self.images = image_files(&app, &self.assets, &self.filename);
            self.index = 0;
             
            if let Some(img) = self.images.last() {
                println!("last image: {}", img.display());
            }

        }

        if !self.images.is_empty() {
            let img = &self.images[self.index];

            if let Ok(texture) = wgpu::Texture::from_path(app, img) {
                self.texture = Some(texture);
            }
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.interval += update.since_last;
    if model.interval.as_millis() > 100 {
        model.interval = Duration::from_secs(0);
        model.inc_image(app);
    }
}

fn image_files(app: &App, assets_path: &PathBuf, filename: &str) -> Vec<PathBuf> {
    let assets = app.assets_path().expect("can't find assets folder");

    let mut files: Vec<PathBuf> = walk_dir(assets.join("images"))
        .into_iter()
        .map(|dir| dir.unwrap().into_path())
        .filter(|p| p.to_string_lossy().ends_with(filename))
        .collect();

    files.sort();

    if files.len() > 100 {
        files[(files.len()-100)..files.len()].to_vec()
    } else {
        files
    }
}

fn model(app: &App) -> Model {
    app.new_window().fullscreen().view(view).build().unwrap();
    app.main_window().set_cursor_visible(false);
    app.set_loop_mode(LoopMode::rate_fps(60.0));

    let filename = match env::var("GEMINIFILENAME") {
        Ok(filename) => filename,
        Err(_) => "default.jpg".to_string(),
    };

    let assets = app.assets_path().expect("can't find assets");

    Model {
        filename,
        interval: Duration::from_secs(0),
        index: 0,
        assets,
        texture: None,
        images: Vec::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();

    if let Some(tex) = &model.texture {
        draw.texture(tex).wh(vec2(1980f32, 1080f32));
    }

    draw.to_frame(app, &frame).unwrap();
}
