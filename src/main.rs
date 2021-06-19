use anyhow::Context;
use chrono::Local;
use log::{debug, warn};
use log::{error, info};
use nannou::prelude::*;
use nannou::ui::prelude::*;
use std::path::PathBuf;

fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{}", err)
    };

    nannou::app(model).update(update).run();
}

struct Model {
    // ui: Ui,
// ids: Ids,
// resolution: usize,
// scale: f32,
// rotation: f32,
// color: Rgb,
// position: Point2,
}

widget_ids! {
    struct Ids {
        resolution,
        scale,
        rotation,
        random_color,
        position,
    }
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);

    app.new_window()
        .size(1920, 1080)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .expect("couldn't create a window");

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    // Init our variables
    let resolution = 6;
    let scale = 200.0;
    let rotation = 0.0;
    let position = pt2(0.0, 0.0);
    let color = rgb(1.0, 0.0, 1.0);

    Model {
        // ui,
        // ids,
        // resolution,
        // scale,
        // rotation,
        // position,
        // color,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if let Err(err) = match key {
        Key::X => export_as_svg(model),
        _ => Ok(()),
    } {
        error!("{}", err);
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
}

fn update_ui(model: &mut Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    // let ui = &mut model.ui.set_widgets();

    // fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    //     widget::Slider::new(val, min, max)
    //         .w_h(200.0, 30.0)
    //         .label_font_size(15)
    //         .rgb(0.3, 0.3, 0.3)
    //         .label_rgb(1.0, 1.0, 1.0)
    //         .border(0.0)
    // }

    // for value in slider(model.resolution as f32, 3.0, 15.0)
    //     .top_left_with_margin(20.0)
    //     .label("Resolution")
    //     .set(model.ids.resolution, ui)
    // {
    //     model.resolution = value as usize;
    // }

    // for value in slider(model.scale, 10.0, 500.0)
    //     .down(10.0)
    //     .label("Scale")
    //     .set(model.ids.scale, ui)
    // {
    //     model.scale = value;
    // }

    // for value in slider(model.rotation, -PI, PI)
    //     .down(10.0)
    //     .label("Rotation")
    //     .set(model.ids.rotation, ui)
    // {
    //     model.rotation = value;
    // }

    // for _click in widget::Button::new()
    //     .down(10.0)
    //     .w_h(200.0, 60.0)
    //     .label("Random Color")
    //     .label_font_size(15)
    //     .rgb(0.3, 0.3, 0.3)
    //     .label_rgb(1.0, 1.0, 1.0)
    //     .border(0.0)
    //     .set(model.ids.random_color, ui)
    // {
    //     model.color = rgb(random(), random(), random());
    // }

    // for (x, y) in widget::XYPad::new(
    //     model.position.x,
    //     -200.0,
    //     200.0,
    //     model.position.y,
    //     -200.0,
    //     200.0,
    // )
    // .down(10.0)
    // .w_h(200.0, 200.0)
    // .label("Position")
    // .label_font_size(15)
    // .rgb(0.3, 0.3, 0.3)
    // .label_rgb(1.0, 1.0, 1.0)
    // .border(0.0)
    // .set(model.ids.position, ui)
    // {
    //     model.position = Point2::new(x, y);
    // }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    // draw.ellipse()
    //     .xy(model.position)
    //     .radius(model.scale)
    //     .resolution(model.resolution)
    //     .rotate(model.rotation)
    //     .color(model.color);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // // Draw the state of the `Ui` to the frame.
    // model.ui.draw_to_frame(app, &frame).unwrap();
}

fn build_svg_document_from_model(_model: &Model) -> svg::Document {
    use svg::node::element::path::Data;
    use svg::node::element::Path;
    use svg::Document;

    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    Document::new().set("viewBox", (0, 0, 70, 70)).add(path)
}

fn export_as_svg(model: &Model) -> Result<(), anyhow::Error> {
    info!("exporting image as SVG...");
    let document = build_svg_document_from_model(&model);
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
