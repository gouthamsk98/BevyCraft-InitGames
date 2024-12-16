//! A freecam-style camera controller plugin.
//! To use in your own application:
//! - Copy the code for the [`CameraControllerPlugin`] and add the plugin to your App.
//! - Attach the [`CameraController`] component to an entity with a [`Camera3dBundle`].

use bevy::input::mouse::{ MouseMotion, MouseScrollUnit, MouseWheel };
use bevy::{ prelude::*, render::camera::{ ScalingMode } };
use bevy::window::CursorGrabMode;
use std::{ f32::consts::*, fmt };
use crate::models::{ CameraController, ToolType };
use crate::web::get_tool_type;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
    }
}

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly 1.0 / 180.0,
/// but I'm guessing it is a misunderstanding between degrees/radians and then sticking with
/// it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut left_mouse_cursor_grab: Local<bool>,
    mut right_mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController, &mut Projection), With<Camera>>
) {
    let tool_type: ToolType = get_tool_type().into();
    if tool_type != ToolType::Default {
        return;
    }
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut controller, mut projection)) = query.get_single_mut() {
        if !controller.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
            controller.initialized = true;
        }
        if !controller.enabled {
            mouse_events.clear();
            return;
        }

        let mut scroll = 0.0;
        for scroll_event in scroll_events.read() {
            let amount = match scroll_event.unit {
                MouseScrollUnit::Line => scroll_event.y,
                MouseScrollUnit::Pixel => scroll_event.y / 16.0,
            };
            scroll += amount;
        }
        controller.scroll += scroll * controller.scroll_factor;
        transform.scale = Vec3::ONE * controller.scroll;

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(controller.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(controller.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(controller.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(controller.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(controller.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(controller.key_down) {
            axis_input.y -= 1.0;
        }

        let mut left_cursor_grab_change = false;
        let mut right_cursor_grab_change = false;
        if key_input.just_pressed(controller.keyboard_key_toggle_cursor_grab) {
            *toggle_cursor_grab = !*toggle_cursor_grab;
            left_cursor_grab_change = true;
        }
        if mouse_button_input.just_pressed(controller.mouse_left_key_cursor_grab) {
            *left_mouse_cursor_grab = true;
            left_cursor_grab_change = true;
        }
        if mouse_button_input.just_released(controller.mouse_left_key_cursor_grab) {
            *left_mouse_cursor_grab = false;
            left_cursor_grab_change = true;
        }
        if mouse_button_input.just_pressed(controller.mouse_right_key_cursor_grab) {
            right_cursor_grab_change = true;
            *right_mouse_cursor_grab = true;
        }
        if mouse_button_input.just_released(controller.mouse_right_key_cursor_grab) {
            right_cursor_grab_change = true;
            *right_mouse_cursor_grab = false;
        }
        let left_cursor_grab = *left_mouse_cursor_grab || *toggle_cursor_grab;

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(controller.key_run) {
                controller.run_speed
            } else {
                controller.walk_speed
            };
            controller.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = controller.friction.clamp(0.0, 1.0);
            controller.velocity *= 1.0 - friction;
            if controller.velocity.length_squared() < 1e-6 {
                controller.velocity = Vec3::ZERO;
            }
        }
        let forward = *transform.forward();
        let right = *transform.right();
        transform.translation +=
            controller.velocity.x * dt * right +
            controller.velocity.y * dt * Vec3::Y +
            controller.velocity.z * dt * forward;

        // Handle cursor grab
        if left_cursor_grab_change || right_cursor_grab_change {
            if left_cursor_grab || *right_mouse_cursor_grab {
                for mut window in &mut windows {
                    if !window.focused {
                        continue;
                    }

                    window.cursor.grab_mode = CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }
            } else {
                for mut window in &mut windows {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                }
            }
        }
        info!("cursor_grab: {} left cursor grad {}", left_cursor_grab, left_cursor_grab_change);
        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if left_cursor_grab || *right_mouse_cursor_grab {
            for mouse_event in mouse_events.read() {
                mouse_delta += mouse_event.delta;
            }
        } else {
            mouse_events.clear();
        }

        //check if tool_type is None
        if mouse_delta != Vec2::ZERO {
            // Apply look update
            controller.pitch = (
                controller.pitch -
                mouse_delta.y * RADIANS_PER_DOT * controller.sensitivity
            ).clamp(-PI / 2.0, PI / 2.0);
            controller.yaw -= mouse_delta.x * RADIANS_PER_DOT * controller.sensitivity;
            if left_cursor_grab {
                transform.rotation = Quat::from_euler(
                    EulerRot::ZYX,
                    0.0,
                    controller.yaw,
                    controller.pitch
                );
            }
            if *right_mouse_cursor_grab {
                transform.translation.x -= mouse_delta.x * 0.01;
                transform.translation.z -= mouse_delta.y * 0.01;
            }
        }
    }
}
