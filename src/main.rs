use anyhow::Context;
use chrono::Local;
use log::{debug, error, info, trace, warn};
use nannou::{prelude::*, ui::prelude::*};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::path::PathBuf;
use svg::node::element::Polyline;

fn main() {
    let res = dotenv::dotenv();
    env_logger::init();
    if let Err(err) = res {
        warn!("{}", err)
    };

    nannou::app(model).update(update).run();
}

type PointColumns = Vec<Vec<Vector2>>;

struct Model {
    ui: Ui,
    ids: Ids,
    pub point_columns: PointColumns,
    pub point_column_params: PointColumnParams,
    pub show_viewbox: bool,
}

widget_ids! {
    struct Ids {
        column_alignment,
        column_spacing,
        export_svg,
        height,
        lines_per_column,
        noise_seed,
        number_of_columns,
        points_per_line,
        toggle_viewbox,
        vertical_jitter,
        width,
    }
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);

    let window_id = app
        .new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .expect("couldn't create a window");

    // Create the UI.
    let mut ui = app.new_ui().window(window_id).build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        ui,
        ids,
        point_columns: Vec::new(),
        point_column_params: Default::default(),
        show_viewbox: false,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);

    if model.point_columns.is_empty() {
        model.point_columns = generate_point_columns(&model.point_column_params);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PointColumnParams {
    pub column_spacing: f32,
    pub height: f32,
    pub lines_per_column: usize,
    pub noise_seed: u64,
    pub number_of_columns: usize,
    pub points_per_line: usize,
    pub vertical_jitter: f32,
    pub width: f32,
    pub column_alignment: f32,
}

impl Default for PointColumnParams {
    fn default() -> Self {
        Self {
            column_spacing: 80.0,
            height: 1000.0,
            lines_per_column: 16,
            noise_seed: 0,
            number_of_columns: 8,
            points_per_line: 16,
            vertical_jitter: 0.0,
            width: 1000.0,
            column_alignment: 0.0,
        }
    }
}

// fn generate_point_columns(params: &PointColumnParams) -> Vec<Vec<Vector2<f32>>> {
//     let PointColumnParams {
//         height,
//         lines_per_column,
//         noise_seed,
//         number_of_columns,
//         vertical_jitter,
//         points_per_line,
//         width,
//         column_spacing,
//     } = params;
//
//     let mut columns = Vec::with_capacity(*number_of_columns);
//     let number_of_spaces = (number_of_columns - 1) as f32;
//     let width_sans_spaces = width - (number_of_spaces * *column_spacing);
//     let column_width = width_sans_spaces / *number_of_columns as f32;
//     let horizontal_line_spacing = column_width / *points_per_line as f32;
//     let vertical_line_spacing = height / *points_per_line as f32;
//     let (half_width, half_height) = (width / 2.0, height / 2.0);
//
//     for column_index in 0..*number_of_columns {
//         let mut column = Vec::with_capacity(*lines_per_column);
//         let column_horizontal_origin = column_index as f32 * (column_width + column_spacing);
//         let mut spacing_scales = Vec::with_capacity(*points_per_line);
//
//         for _ in 0..*points_per_line {
//             let scale = horizontal_line_spacing * (random_f32() - 0.5) * 2.0;
//             spacing_scales.push(scale);
//         }
//
//         for line_index in 0..*lines_per_column {
//             let mut line = Vec::with_capacity(*points_per_line);
//             let line_horizontal_origin = line_index as f32 * horizontal_line_spacing;
//
//             for p in 0..*points_per_line {
//                 let point = Vector2::new(
//                     spacing_scales[p] + column_horizontal_origin + line_horizontal_origin
//                         - half_width,
//                     p as f32 * vertical_line_spacing - half_height,
//                 );
//
//                 line.push(point);
//             }
//
//             column.push(line);
//         }
//
//         columns.append(&mut column);
//     }
//
//     columns
// }

fn generate_point_columns(params: &PointColumnParams) -> Vec<Vec<Vector2<f32>>> {
    let mut lines = Vec::new();
    let mut rng: StdRng = SeedableRng::seed_from_u64(params.noise_seed);
    let (origin_x, origin_y) = (params.width * -0.5, params.height * -0.5);
    let count_of_spaces = params.number_of_columns.saturating_sub(1);
    let space_taken_up_by_spaces = count_of_spaces as f32 * params.column_spacing;
    let space_taken_up_by_columns = params.width - space_taken_up_by_spaces;
    let column_width = space_taken_up_by_columns / params.number_of_columns as f32;

    let width_of_a_column_and_a_space = column_width + params.column_spacing;
    for c in 0..params.number_of_columns {
        let origin_x = c as f32 * width_of_a_column_and_a_space + origin_x;
        lines.append(&mut generate_point_column(
            origin_x, origin_y, params, &mut rng,
        ));
    }

    lines
}

fn generate_point_column(
    origin_x: f32,
    origin_y: f32,
    params: &PointColumnParams,
    rng: &mut impl Rng,
) -> Vec<Vec<Vector2<f32>>> {
    let vertical_spacing = params.height / params.points_per_line as f32;
    let mut column_section_widths = Vec::new();

    for _ in 0..params.points_per_line {
        let column_section_width = 125.0 * rng.gen_range(0.3..1.2) as f32;

        column_section_widths.push(column_section_width);
    }

    let mut lines = Vec::new();

    for line_index in 0..params.lines_per_column {
        let mut line = Vec::new();

        for point_index in 0..params.points_per_line {
            let column_width = column_section_widths[point_index];
            let line_spacing = line_index as f32 * (column_width / params.lines_per_column as f32);
            let half_width = (column_width / 2.0) * (params.column_alignment - 1.0);
            let x = line_spacing + origin_x + half_width;
            let y = point_index as f32 * vertical_spacing + origin_y;

            line.push(Vector2::new(x, y));
        }

        lines.push(line);
    }

    lines
}

fn update_ui(model: &mut Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();
    let mut should_refresh_point_columns = false;

    fn dialer(val: f32, min: f32, max: f32) -> widget::NumberDialer<'static, f32> {
        widget::NumberDialer::new(val, min, max, 0)
            .w_h(300.0, 20.0)
            .label_font_size(12)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    if let Some(noise_seed) = dialer(model.point_column_params.noise_seed as f32, 0.0, 999_9999.0)
        .label("Noise Seed")
        .top_left_with_margin(20.0)
        .set(model.ids.noise_seed, ui)
    {
        model.point_column_params.noise_seed = noise_seed as u64;
        should_refresh_point_columns = true;
    }

    if let Some(lines_per_column) = dialer(
        model.point_column_params.lines_per_column as f32,
        1.0,
        100.0,
    )
    .down(10.0)
    .label("lines_per_column")
    .set(model.ids.lines_per_column, ui)
    {
        model.point_column_params.lines_per_column = lines_per_column as usize;
        should_refresh_point_columns = true;
    }

    if let Some(column_spacing) = dialer(model.point_column_params.column_spacing, 0.0, 10_000.0)
        .down(10.0)
        .label("column_spacing")
        .set(model.ids.column_spacing, ui)
    {
        model.point_column_params.column_spacing = column_spacing;
        should_refresh_point_columns = true;
    }

    if let Some(height) = dialer(model.point_column_params.height, 0.0, 10_000.0)
        .down(10.0)
        .label("height")
        .set(model.ids.height, ui)
    {
        model.point_column_params.height = height;
        should_refresh_point_columns = true;
    }

    if let Some(width) = dialer(model.point_column_params.width, 0.0, 10_000.0)
        .down(10.0)
        .label("width")
        .set(model.ids.width, ui)
    {
        model.point_column_params.width = width;
        should_refresh_point_columns = true;
    }

    if let Some(number_of_columns) = dialer(
        model.point_column_params.number_of_columns as f32,
        1.0,
        999.0,
    )
    .down(10.0)
    .label("number_of_columns")
    .set(model.ids.number_of_columns, ui)
    {
        model.point_column_params.number_of_columns = number_of_columns as usize;
        should_refresh_point_columns = true;
    }

    if let Some(points_per_line) =
        dialer(model.point_column_params.points_per_line as f32, 1.0, 999.0)
            .down(10.0)
            .label("points_per_line")
            .set(model.ids.points_per_line, ui)
    {
        model.point_column_params.points_per_line = points_per_line as usize;
        should_refresh_point_columns = true;
    }

    if let Some(column_alignment) =
        widget::NumberDialer::new(model.point_column_params.column_alignment, -1.0, 1.0, 3)
            .w_h(300.0, 20.0)
            .label_font_size(12)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
            .down(10.0)
            .label("column_alignment")
            .set(model.ids.column_alignment, ui)
    {
        model.point_column_params.column_alignment = column_alignment;
        should_refresh_point_columns = true;
    }

    if let Some(vertical_jitter) = dialer(model.point_column_params.vertical_jitter, 1.0, 100.0)
        .down(10.0)
        .label("vertical_jitter")
        .set(model.ids.vertical_jitter, ui)
    {
        model.point_column_params.vertical_jitter = vertical_jitter;
        should_refresh_point_columns = true;
    }

    for _click in widget::Button::new()
        .down(10.0)
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
        let point_columns = &model.point_columns;
        let point_column_params = &model.point_column_params;
        if let Err(err) = export_as_svg(point_columns, point_column_params) {
            error!("{}", err)
        }
    }

    if should_refresh_point_columns {
        model.point_columns = generate_point_columns(&model.point_column_params);
        trace!(
            "refresh point columns called, generated {} lines",
            model.point_columns.len()
        );
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    for column in &model.point_columns {
        draw.polyline()
            .weight(1.0)
            .color(BLACK)
            .join_round()
            // do I really have to clone here?
            .points(column.to_owned());
    }

    if model.show_viewbox {
        draw.rect()
            // we want the box centered on the screen, nannou places rectangle from their center
            .x_y(0.0, 0.0)
            .w_h(
                model.point_column_params.width,
                model.point_column_params.height,
            )
            .stroke(RED)
            .stroke_weight(2.0)
            .no_fill();
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn build_svg_document_from_model(
    point_columns: &PointColumns,
    point_column_params: &PointColumnParams,
) -> svg::Document {
    let doc = svg::Document::new().set(
        "viewBox",
        (0, 0, point_column_params.width, point_column_params.height),
    );

    let mut group = svg::node::element::Group::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    for line in point_columns.iter() {
        let data: Vec<_> = line
            .iter()
            .map(|p| format!("{:.2},{:.2}", p.x, p.y))
            .collect();

        let path = Polyline::new().set("points", data.join(" "));

        group = group.add(path);
    }

    let bounding_rect = svg::node::element::Rectangle::new()
        .set("width", point_column_params.width)
        .set("height", point_column_params.height)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    doc.add(group).add(bounding_rect)
}

fn export_as_svg(
    point_columns: &PointColumns,
    point_column_params: &PointColumnParams,
) -> Result<(), anyhow::Error> {
    info!("exporting image as SVG...");
    let document = build_svg_document_from_model(point_columns, point_column_params);
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
