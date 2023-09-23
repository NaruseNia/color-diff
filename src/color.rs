use std::intrinsics::powf64;

trait Color {

}

#[derive(Debug)]
pub struct ColorRGB {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(Debug)]
pub struct ColorLab {
    l: f64,
    a: f64,
    b: f64,
}

#[derive(Debug)]
pub struct ColorXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl ColorRGB {
    pub fn from_rgb(r: u32, g: u32, b: u32) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        } 
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xff) as f64 / 255.0,
            g: ((hex >> 8) & 0xff) as f64 / 255.0,
            b: (hex & 0xff) as f64 / 255.0,
        } 
    }

    pub fn liner(&self) -> ColorRGB {
        let r_lin = if self.r > 0.4045 { ((self.r + 0.055) / 1.055).powf(2.4) } else { self.r / 12.92 };
        let g_lin = if self.g > 0.4045 { ((self.g + 0.055) / 1.055).powf(2.4) } else { self.g / 12.92 };
        let b_lin = if self.b > 0.4045 { ((self.b + 0.055) / 1.055).powf(2.4) } else { self.b / 12.92 };

        ColorRGB { r: r_lin, g: g_lin, b: b_lin }
    }

    pub fn xyz(&self) -> ColorXYZ {
        let liner = self.liner();
        ColorXYZ {
            x: (liner.r * 0.4124) + (liner.g * 0.3576) + (liner.b * 0.1805),
            y: (liner.r * 0.2126) + (liner.g * 0.7152) + (liner.b * 0.0722),
            z: (liner.r * 0.0193) + (liner.g * 0.1192) + (liner.b * 0.9505),
        }
    }
}

