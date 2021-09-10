mod pasta;

use anyhow::Context;
use chrono::Local;
use log::{debug, error, info, trace, warn};
use nannou::{geom::Rect, prelude::*, ui::prelude::*};
use pasta::{new_lines_from_noise, Line};
use std::path::PathBuf;

fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{}", err)
    };

    nannou::app(model).update(update).run();
}

#[derive(Default)]
pub struct State {
    pub height: f32,
    pub lines: Vec<Line>,
    pub noise_seed: u64,
    pub noise_scale: f64,
    pub line_count: usize,
    pub should_rebuild: bool,
    pub show_viewbox: bool,
    pub stroke_weight: f32,
    pub vertical_jitter: f32,
    pub width: f32,
    pub min_line_length: f32,
}

impl State {
    fn new(width: f32, height: f32) -> Self {
        Self {
            height,
            width,
            show_viewbox: false,
            should_rebuild: true,
            stroke_weight: 3.0,
            lines: Vec::new(),
            line_count: 200,
            min_line_length: 10.0,
            vertical_jitter: 0.001,
            noise_seed: 0,
            noise_scale: 0.01,
        }
    }

    fn update(&mut self) {
        if self.should_rebuild {
            self.lines = new_lines_from_noise(&self);
            self.should_rebuild = false;
        }
    }
}

struct Model {
    ui: Ui,
    ids: Ids,
    pub state: State,
}

widget_ids! {
    struct Ids {
        export_svg,
        height,
        line_count,
        min_line_length,
        noise_scale,
        noise_seed,
        param_title_text,
        stroke_weight,
        toggle_viewbox,
        vertical_jitter,
        width,
    }
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);
    let (width, height) = (640, 480);

    let window_id = app
        .new_window()
        .size(width, height)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .expect("couldn't create a window");

    // Create the UI.
    let mut ui = app.new_ui().window(window_id).build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());
    let state = State::new(width as f32, height as f32);

    Model { ui, ids, state }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    model.state.update();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::T => {
            if model.state.min_line_length < 100.0 {
                model.state.min_line_length += 5.0;
                model.state.should_rebuild = true;
            }
        }
        Key::H => {
            if model.state.min_line_length > 0.0 {
                model.state.min_line_length -= 5.0;
                model.state.should_rebuild = true;
            }
        }
        _ => (),
    }
}

const SETTING_WIDTH: f64 = 300.0;
const SETTING_HEIGHT: f64 = 30.0;
const SETTING_MARGIN: f64 = 10.0;
const LABEL_COLOR: nannou::ui::color::Color = nannou::ui::color::WHITE;
const FILL: nannou::ui::color::Color = nannou::ui::color::DARK_BLUE;
const FONT_SIZE: u32 = 12;

fn dialer(val: f32, min: f32, max: f32) -> widget::NumberDialer<'static, f32> {
    use nannou::ui::color::*;
    widget::NumberDialer::new(val, min, max, 0)
        .w_h(SETTING_WIDTH, SETTING_HEIGHT)
        .label_font_size(FONT_SIZE)
        .color(FILL)
        .label_color(LABEL_COLOR)
        .border(0.0)
}

fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    use nannou::ui::color::*;
    widget::Slider::new(val, min, max)
        .w_h(SETTING_WIDTH, SETTING_HEIGHT)
        .label_font_size(FONT_SIZE)
        .color(FILL)
        .label_color(LABEL_COLOR)
        .border(0.0)
}

fn update_ui(model: &mut Model) {
    let ui = &mut model.ui.set_widgets();
    let mut should_refresh_model = false;

    widget::Text::new("Parameters")
        .top_left_with_margin(20.0)
        .color(LABEL_COLOR)
        .font_size(20)
        .set(model.ids.param_title_text, ui);

    for count in dialer(model.state.line_count as f32, 1.0, 512.0)
        .down(SETTING_MARGIN)
        .label("Count")
        .set(model.ids.line_count, ui)
    {
        model.state.line_count = count.floor() as usize;
        should_refresh_model = true;
    }

    for height in dialer(model.state.height as f32, 10.0, 2160.0)
        .down(SETTING_MARGIN)
        .label("Height")
        .set(model.ids.height, ui)
    {
        model.state.height = height.floor();
        should_refresh_model = true;
    }

    for width in dialer(model.state.width as f32, 10.0, 3840.0)
        .down(SETTING_MARGIN)
        .label("Width")
        .set(model.ids.width, ui)
    {
        model.state.width = width.floor();
        should_refresh_model = true;
    }

    for vertical_jitter in slider(model.state.vertical_jitter, 0.001, 100.0)
        .down(SETTING_MARGIN)
        .label("Jitter")
        .set(model.ids.vertical_jitter, ui)
    {
        model.state.vertical_jitter = vertical_jitter;
        should_refresh_model = true;
    }

    for min_line_length in slider(model.state.min_line_length, 1.0, 100.0)
        .down(SETTING_MARGIN)
        .label("Min Line Length")
        .set(model.ids.min_line_length, ui)
    {
        model.state.min_line_length = min_line_length;
        should_refresh_model = true;
    }

    for noise_seed in slider(model.state.noise_seed as f32, 0.0, 100.0)
        .down(SETTING_MARGIN)
        .label("Noise")
        .set(model.ids.noise_seed, ui)
    {
        model.state.noise_seed = noise_seed as u64;
        should_refresh_model = true;
    }

    for noise_scale in slider(model.state.noise_scale as f32, 0.001, 0.01)
        .down(SETTING_MARGIN)
        .label("Noise Scale")
        .set(model.ids.noise_scale, ui)
    {
        model.state.noise_scale = noise_scale as f64;
        should_refresh_model = true;
    }

    for stroke_weight in slider(model.state.stroke_weight, 1.0, 12.0)
        .down(SETTING_MARGIN)
        .label("Stroke Weight (px)")
        .set(model.ids.stroke_weight, ui)
    {
        model.state.stroke_weight = stroke_weight;
        should_refresh_model = true;
    }

    for _click in widget::Button::new()
        .down(SETTING_MARGIN)
        .w_h(SETTING_WIDTH, SETTING_HEIGHT)
        .label_font_size(FONT_SIZE)
        .color(FILL)
        .label_color(LABEL_COLOR)
        .border(0.0)
        .label("Toggle Viewbox")
        .set(model.ids.toggle_viewbox, ui)
    {
        model.state.show_viewbox = !model.state.show_viewbox;
        should_refresh_model = true;
    }

    for _click in widget::Button::new()
        .down(SETTING_MARGIN)
        .w_h(SETTING_WIDTH, SETTING_HEIGHT)
        .label_font_size(FONT_SIZE)
        .color(FILL)
        .label_color(LABEL_COLOR)
        .border(0.0)
        .label("Export SVG")
        .set(model.ids.export_svg, ui)
    {
        if let Err(err) = export_as_svg(&model.state) {
            error!("{}", err)
        }
    }

    if should_refresh_model {
        model.state.should_rebuild = true;
        trace!("refresh model called");
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let bounding_rect = Rect::from_w_h(model.state.width, model.state.height)
        .align_middle_x_of(app.window_rect())
        .align_middle_y_of(app.window_rect());

    if model.state.show_viewbox {
        draw.rect()
            .xy(bounding_rect.xy())
            .wh(bounding_rect.wh())
            .stroke(RED)
            .stroke_weight(2.0)
            .no_fill();
    }

    let line_draw = draw.translate(bounding_rect.bottom_left().extend(0.0));

    model
        .state
        .lines
        .iter()
        .for_each(|line| line.draw(&line_draw, model.state.stroke_weight));

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn build_svg_document_from_state(state: &State) -> svg::Document {
    let doc = svg::Document::new().set("viewBox", (0, 0, state.width, state.height));

    let mut group = svg::node::element::Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", state.stroke_weight);

    for line in state.lines.iter() {
        let path = line.as_svg();

        group = group.add(path);
    }

    let bounding_rect = svg::node::element::Rectangle::new()
        .set("width", state.width)
        .set("height", state.height)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    doc.add(group).add(bounding_rect)
}

fn export_as_svg(state: &State) -> Result<(), anyhow::Error> {
    info!("exporting image as SVG...");
    let document = build_svg_document_from_state(state);
    let base_path = std::env::var("SVG_EXPORT_DIRECTORY").context("setting 'SVG_EXPORT_DIRECTORY' is required, please set it to the directory you wish to export SVGs to")?;
    let current_date = Local::today().format("%Y-%m-%d");
    let svg_filename = format!("{}-plotling.svg", &current_date);
    let mut svg_filepath: PathBuf = [base_path, svg_filename].iter().collect();

    // I don't want to silently overwrite anything so I look for an unused filename,
    // incrementing the counter until I find an unused number
    // I could have also used a random string/number, I just like this better
    if svg_filepath.exists() {
        let mut counter = 1;

        while svg_filepath.exists() {
            if counter > 100 {
                debug!(
                    "export_as_svg counter has reached {}, you're not in an infinite loop are you?",
                    counter
                );
            }

            let _ = svg_filepath.pop();
            let svg_filename = format!("{}-plotling-{}.svg", &current_date, &counter);
            svg_filepath.push(svg_filename);
            counter += 1;
        }
    }

    svg::save(&svg_filepath, &document)?;
    info!(
        "SVG successfully exported to {}",
        &svg_filepath.to_string_lossy()
    );

    Ok(())
}
