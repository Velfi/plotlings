mod fixed_body;
mod massive;
mod orbiter;
mod params;
mod system;

use anyhow::Context;
use chrono::Local;
use lib_plotings::MouseButtonState;
use log::{debug, error, info, trace, warn};
use nannou::{prelude::*, ui::prelude::*};
use params::Params;
use rand::{prelude::StdRng, SeedableRng};
use std::path::PathBuf;
use std::{cell::RefCell, mem};
use system::System;

fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{}", err)
    };

    nannou::app(model).update(update).run();
}

pub struct Model {
    ui: Ui,
    ids: Ids,
    rng: RefCell<StdRng>,
    pub params: Params,
    pub system: System,
    pub show_viewbox: bool,
    pub mouse_xy: Point2,
    pub mouse_button_l: MouseButtonState,
    mouse_button_l_secret_state: bool,
}

widget_ids! {
    struct Ids {
        export_svg,
        noise_seed,
        toggle_viewbox,
        restart,
        starting_velocity,
    }
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(1920, 1080)
        .view(view)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .expect("couldn't create a window");

    // Create the UI.
    let mut ui = app.new_ui().window(window_id).build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());
    let params = Params::default();
    let mut rng: StdRng = SeedableRng::seed_from_u64(params.rng_seed);
    let system = System::new_from_params(&params, &mut rng);
    let rng = RefCell::new(rng);

    Model {
        ui,
        ids,
        rng,
        system,
        params,
        show_viewbox: false,
        mouse_xy: Default::default(),
        mouse_button_l: MouseButtonState::Released,
        mouse_button_l_secret_state: false,
    }
}

pub fn mouse_moved(_app: &App, model: &mut Model, position: Point2) {
    model.mouse_xy = position;
}

pub fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            model.mouse_button_l_secret_state = true;
        }
        _ => trace!("unhandled mouse press event: {:?}", button),
    }
}

pub fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            model.mouse_button_l_secret_state = false;
        }
        _ => trace!("unhandled mouse release event: {:?}", button),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);

    model
        .mouse_button_l
        .update(model.mouse_button_l_secret_state);

    model
        .system
        .update(&model.params, Some(model.mouse_xy), model.mouse_button_l)
}

fn update_ui(model: &mut Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let mut ui_cell = model.ui.set_widgets();
    let ui = &mut ui_cell;
    let mut should_refresh_system = false;

    // fn dialer(val: f32, min: f32, max: f32) -> widget::NumberDialer<'static, f32> {
    //     widget::NumberDialer::new(val, min, max, 0)
    //         .w_h(300.0, 20.0)
    //         .label_font_size(12)
    //         .rgb(0.3, 0.3, 0.3)
    //         .label_rgb(1.0, 1.0, 1.0)
    //         .border(0.0)
    // }

    // if let Some(noise_seed) = dialer(model.point_column_params.noise_seed as f32, 0.0, 999_9999.0)
    //     .label("Noise Seed")
    //     .top_left_with_margin(20.0)
    //     .set(model.ids.noise_seed, ui)
    // {
    //     model.point_column_params.noise_seed = noise_seed as u64;
    //     should_refresh_system = true;
    // }

    // if let Some(lines_per_column) = dialer(
    //     model.point_column_params.lines_per_column as f32,
    //     1.0,
    //     100.0,
    // )
    // .down(10.0)
    // .label("lines_per_column")
    // .set(model.ids.lines_per_column, ui)
    // {
    //     model.point_column_params.lines_per_column = lines_per_column as usize;
    //     should_refresh_system = true;
    // }

    // if let Some(column_spacing) = dialer(model.point_column_params.column_spacing, 0.0, 10_000.0)
    //     .down(10.0)
    //     .label("column_spacing")
    //     .set(model.ids.column_spacing, ui)
    // {
    //     model.point_column_params.column_spacing = column_spacing;
    //     should_refresh_system = true;
    // }

    // if let Some(height) = dialer(model.point_column_params.height, 0.0, 10_000.0)
    //     .down(10.0)
    //     .label("height")
    //     .set(model.ids.height, ui)
    // {
    //     model.point_column_params.height = height;
    //     should_refresh_system = true;
    // }

    // if let Some(width) = dialer(model.point_column_params.width, 0.0, 10_000.0)
    //     .down(10.0)
    //     .label("width")
    //     .set(model.ids.width, ui)
    // {
    //     model.point_column_params.width = width;
    //     should_refresh_system = true;
    // }

    // if let Some(column_width) = dialer(model.point_column_params.column_width, 0.0, 1000.0)
    //     .down(10.0)
    //     .label("column_column_width")
    //     .set(model.ids.column_width, ui)
    // {
    //     model.point_column_params.column_width = column_width;
    //     should_refresh_system = true;
    // }

    // if let Some(number_of_columns) = dialer(
    //     model.point_column_params.number_of_columns as f32,
    //     1.0,
    //     999.0,
    // )
    // .down(10.0)
    // .label("number_of_columns")
    // .set(model.ids.number_of_columns, ui)
    // {
    //     model.point_column_params.number_of_columns = number_of_columns as usize;
    //     should_refresh_system = true;
    // }

    // if let Some(points_per_line) =
    //     dialer(model.point_column_params.points_per_line as f32, 1.0, 999.0)
    //         .down(10.0)
    //         .label("points_per_line")
    //         .set(model.ids.points_per_line, ui)
    // {
    //     model.point_column_params.points_per_line = points_per_line as usize;
    //     should_refresh_system = true;
    // }

    // if let Some(column_alignment) =
    //     widget::NumberDialer::new(model.point_column_params.column_alignment, -1.0, 1.0, 3)
    //         .w_h(300.0, 20.0)
    //         .label_font_size(12)
    //         .rgb(0.3, 0.3, 0.3)
    //         .label_rgb(1.0, 1.0, 1.0)
    //         .border(0.0)
    //         .down(10.0)
    //         .label("column_alignment")
    //         .set(model.ids.column_alignment, ui)
    // {
    //     model.point_column_params.column_alignment = column_alignment;
    //     should_refresh_system = true;
    // }

    // if let Some(vertical_jitter) = dialer(model.point_column_params.vertical_jitter, 1.0, 100.0)
    //     .down(10.0)
    //     .label("vertical_jitter")
    //     .set(model.ids.vertical_jitter, ui)
    // {
    //     model.point_column_params.vertical_jitter = vertical_jitter;
    //     should_refresh_system = true;
    // }

    for _click in widget::Button::new()
        // .down(10.0)
        .top_left_with_margin(20.0)
        .w_h(300.0, 20.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .label("Restart")
        .set(model.ids.restart, ui)
    {
        should_refresh_system = true;
    }

    for _click in widget::Button::new()
        // .down(10.0)
        .top_left_with_margin(20.0)
        .w_h(300.0, 20.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .label("Toggle Viewbox")
        .set(model.ids.toggle_viewbox, ui)
    {
        model.show_viewbox = !model.show_viewbox;
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(300.0, 20.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .label("Export SVG")
        .set(model.ids.export_svg, ui)
    {
        if let Err(err) = export_as_svg(&model.params, &model.system) {
            error!("{}", err)
        }
    }

    mem::drop(ui_cell);

    if should_refresh_system {
        let rng = &mut *model.rng.borrow_mut();
        let system = System::new_from_params(&model.params, rng);
        // TODO do I need to manually drop here or is the compiler smart enough?
        mem::drop(rng);

        model.system = system;
        trace!(
            "should_refresh_system=true, creating {} orbiter(s)",
            model.system.orbiters().len()
        );
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    model.system.draw(&draw, &model.params);

    if model.show_viewbox {
        draw.rect()
            // we want the box centered on the screen, nannou places rectangle from their center
            .x_y(0.0, 0.0)
            .w_h(model.params.width, model.params.height)
            .stroke(RED)
            .stroke_weight(2.0)
            .no_fill()
            .finish();
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn build_svg_document_from_model(params: &Params, system: &System) -> svg::Document {
    let doc = svg::Document::new().set("viewBox", (0, 0, params.width, params.height));
    let system = system.svg(params);
    let bounding_rect = svg::node::element::Rectangle::new()
        .set("width", params.width)
        .set("height", params.height)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", "2mm");

    doc.add(system).add(bounding_rect)
}

fn export_as_svg(params: &Params, system: &System) -> Result<(), anyhow::Error> {
    info!("exporting image as SVG...");
    let document = build_svg_document_from_model(params, system);
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
