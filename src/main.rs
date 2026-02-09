#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use macroquad::prelude::*;

struct MetaBall {
    pos: Vec2,
    dir: Vec2,
    radius: f32,
}

struct UiState {
    speed: f32,
    ball_radius: f32,
    show_settings: bool,
    color: [f32; 3]
}

const MAX_BALLS: usize = 100;

fn window_conf() -> Conf {
    Conf {
        window_title: "Lava Lamp".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut meta_balls: Vec<MetaBall> = vec![];

    let mut ui_state = UiState {
        speed: 0.05,
        ball_radius: 0.05,
        show_settings: true,
        color: [1.0, 0.0, 0.7]
    };

    let material = load_material(

        ShaderSource::Glsl {
            vertex: include_str!("../shaders/metaballs.vert"),
            fragment: include_str!("../shaders/metaballs.frag"),
        },

        MaterialParams {

            uniforms: vec![
                UniformDesc::array(UniformDesc::new("metaballs", UniformType::Float3), MAX_BALLS),
                UniformDesc::new("count", UniformType::Int1),
                UniformDesc::new("threshold", UniformType::Float1),
                UniformDesc::new("resolution", UniformType::Float2),
                UniformDesc::new("base_color", UniformType::Float3)
            ],

            ..Default::default()

        },

    ).expect("shader compile/link failed");


    loop {

        clear_background(BLACK);

        draw_ui(&mut ui_state, &mut meta_balls);

        if is_key_pressed(KeyCode::Tab) {
            ui_state.show_settings = !ui_state.show_settings;
        }

        if is_key_pressed(KeyCode::Space) && meta_balls.len() < MAX_BALLS {

            let new_ball = MetaBall {
                pos: vec2(rand::gen_range(0.1, 0.9), rand::gen_range(0.1, 0.9)),
                dir: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
                radius: ui_state.ball_radius,
            };

            meta_balls.push(new_ball);

        }

        move_metaballs(&mut meta_balls, ui_state.speed, screen_width(), screen_height());

        draw_metaballs_shader(&material, &meta_balls, ui_state.color);

        egui_macroquad::draw();

        next_frame().await;

    }

}

fn draw_ui(state: &mut UiState, meta_balls: &mut Vec<MetaBall>) {

    egui_macroquad::ui(|egui_ctx| {

        if state.show_settings {

            egui::Window::new(egui::RichText::new("Settings").color(egui::Color32::WHITE))
                .anchor(egui::Align2::LEFT_TOP, [20.0, 20.0])
                .collapsible(false)
                .resizable(true)
                .default_width(250.0)
                .open(&mut state.show_settings) 

                .show(egui_ctx, |ui| {

                    ui.set_max_width(250.0);
                    ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                    ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::WHITE;
                    ui.visuals_mut().widgets.active.bg_fill = egui::Color32::WHITE;
                    ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::WHITE;

                    ui.vertical_centered_justified(|ui| {
                        ui.add(egui::Slider::new(&mut state.speed, 0.01..=1.0).text("Speed"));
                        ui.add(egui::Slider::new(&mut state.ball_radius, 0.0..=0.5).text("Ball Radius").step_by(0.001));
                    });

                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        
                        ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                        ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_gray(60);
                        ui.visuals_mut().widgets.active.bg_fill = egui::Color32::from_gray(40);
                        ui.color_edit_button_rgb(&mut state.color);
                        
                        ui.add_space(8.0);

                        ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 50, 50);
                        let clear_btn = egui::Button::new(
                            egui::RichText::new("Clear Balls")
                                .color(egui::Color32::WHITE)
                                .size(14.0)
                        ).min_size(egui::vec2(100.0, 24.0));

                        if ui.add(clear_btn).clicked() {
                            meta_balls.clear();
                        }

                    });

                }
            );

        }

    });

}

fn move_metaballs(meta_balls: &mut Vec<MetaBall>, speed: f32, screen_width: f32, screen_height: f32) {

    for meta_ball in meta_balls {

        meta_ball.pos += meta_ball.dir * speed * get_frame_time();

        let avg_screen = (screen_width + screen_height) / 2.0;
        let radius_x = meta_ball.radius * avg_screen / screen_width;
        let radius_y = meta_ball.radius * avg_screen / screen_height;

        if meta_ball.pos.x - radius_x < 0.0 {
            meta_ball.pos.x = radius_x;
            meta_ball.dir.x *= -1.0;
        }

        if meta_ball.pos.x + radius_x > 1.0 {
            meta_ball.pos.x = 1.0 - radius_x;
            meta_ball.dir.x *= -1.0;
        }

        if meta_ball.pos.y - radius_y < 0.0 {
            meta_ball.pos.y = radius_y;
            meta_ball.dir.y *= -1.0;
        }

        if meta_ball.pos.y + radius_y > 1.0 {
            meta_ball.pos.y = 1.0 - radius_y;
            meta_ball.dir.y *= -1.0;
        }

    }

}

fn draw_metaballs_shader(material: &Material, meta_balls: &[MetaBall], color: [f32; 3]) {

    let mut data = vec![Vec3::ZERO; MAX_BALLS];

    for (i, ball) in meta_balls.iter().enumerate().take(MAX_BALLS) {
        data[i] = Vec3::new(ball.pos.x, ball.pos.y, ball.radius);
    }

    material.set_uniform_array("metaballs", &data[..]);
    material.set_uniform("count", meta_balls.len() as i32);
    material.set_uniform("threshold", 1.0f32);
    material.set_uniform("resolution", vec2(screen_width(), screen_height()));
    material.set_uniform("base_color", vec3(color[0], color[1], color[2]));

    gl_use_material(material);
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
    gl_use_default_material();

}
