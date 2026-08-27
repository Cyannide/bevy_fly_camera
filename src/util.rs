use bevy::prelude::*;

pub fn movement_axis(
	input: &Res<ButtonInput<KeyCode>>,
	plus: KeyCode,
	minus: KeyCode,
) -> f32 {
	let mut axis = 0.0;
	if input.pressed(plus) {
		axis += 1.0;
	}
	if input.pressed(minus) {
		axis -= 1.0;
	}
	axis
}

pub fn movement_axis_with_shift_check(
	input: &Res<ButtonInput<KeyCode>>,
	plus: KeyCode,
	minus: KeyCode,
	needs_shift: bool,
) -> f32 {
	let mut axis = 0.0;
	if input.pressed(plus) {
		if needs_shift && input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
			axis += 1.0;
		} else if !needs_shift && !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
			axis += 1.0;
		}
	}
	if input.pressed(minus) {
		if needs_shift && input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
			axis -= 1.0;
		} else if !needs_shift && !input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
			axis -= 1.0;
		}
	}
	axis
}

