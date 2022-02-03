use palette::encoding::TransferFn;

/// linear 0.0-1.0 floating point to srgb 0-255 integer conversion.
pub fn linear_to_srgb(value: f32) -> u32 {
    let v = f32::max(0., f32::min(1., value));
    (palette::encoding::Srgb::from_linear(v) * 255.) as u32
}

/// srgb 0-255 integer to linear 0.0-1.0 floating point conversion.
pub fn srgb_to_linear(value: u8) -> f32 {
    let v = value as f32 / 255.;
    palette::encoding::Srgb::into_linear(v)
}

fn sign(n: f32) -> f32 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

pub fn sign_pow(val: f32, exp: f32) -> f32 {
    sign(val) * f32::powf(val.abs(), exp)
}
