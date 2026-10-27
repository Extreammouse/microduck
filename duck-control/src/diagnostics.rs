//! Selective native operations for the exclusive raw-maintenance controller.
use crate::io::{IoError, Result};
use crate::model::{JOINT_IDS, JOINT_NAMES, homing_offset};
pub use duck_ipc_proto::diagnostics::{Motor, Pid};

pub fn index(id: u8) -> Result<usize> {
    JOINT_IDS
        .iter()
        .position(|v| *v == id)
        .ok_or_else(|| IoError::Bus("unknown motor ID".into()))
}
pub fn empty_motor(id: u8) -> Result<Motor> {
    let i = index(id)?;
    Ok(Motor {
        joint: JOINT_NAMES[i].into(),
        id,
        observed_at_ns: 0,
        model_number: None,
        firmware_version: None,
        hardware_error: None,
        voltage_v: None,
        temperature_c: None,
        current_limit: None,
        pwm_limit: None,
        velocity_limit: None,
        shutdown: None,
        torque: None,
        position_counts: None,
        goal_rad: None,
        pid: None,
        homing_offset_counts: None,
        expected_homing_offset_counts: homing_offset(id),
        before_startup_fix: None,
        operating_mode: None,
        drive_mode: None,
        min_position_rad: None,
        max_position_rad: None,
        profile_velocity: None,
        profile_acceleration: None,
        error: None,
    })
}
pub fn position_valid(q: f64) -> bool {
    q.is_finite() && q >= -std::f64::consts::PI && q <= std::f64::consts::PI * 4094.0 / 4096.0
}
pub fn pid_valid(p: Pid) -> bool {
    p.p <= 16383 && p.i <= 16383 && p.d <= 16383
}
