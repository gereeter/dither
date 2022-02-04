use crate::geom::Affine3;

use image::Pixel;

//////// Color spaces ////////

pub type Srgb8 = image::Rgb<u8>;

#[derive(Copy, Clone)]
pub struct LinearRgb {
    pub data: [f64; 3]
}

impl Affine3 for LinearRgb {
    fn into_coords(self) -> [f64; 3] { self.data }
    fn from_coords(coords: [f64; 3]) -> Self { LinearRgb { data: coords } }
}

impl LinearRgb {
    pub fn clamp(mut self) -> Self {
        for coord in &mut self.data {
            if *coord < 0.0 {
                *coord = 0.0;
            } else if *coord > 1.0 {
                *coord = 1.0;
            }
        }
        self
    }
}

// CIEXYZ: we only use this as an intermediate conversion step
// TODO: inline this?
#[derive(Copy, Clone)]
struct Xyz {
    data: [f64; 3]
}

// TODO: make this unnecessary
// A linear color space designed to approximate the CIELAB color space so that distances are a
// tolerable approximation to color difference
#[derive(Copy, Clone)]
pub struct PseudoLab {
    pub data: [f64; 3]
}

impl Affine3 for PseudoLab {
    fn into_coords(self) -> [f64; 3] { self.data }
    fn from_coords(coords: [f64; 3]) -> Self { PseudoLab { data: coords } }
}

// CIELAB
#[derive(Copy, Clone)]
pub struct Lab {
    pub l: f64,
    a: f64,
    b: f64,
    c: f64 // For optimized color comparisons, we cache this: c = sqrt(a^2 + b^2)
}

//////// Conversions ////////

pub fn srgb_decode_channel(value: u8) -> f64 {
    let normalized = value as f64 / 255.0;
    if normalized < 0.04045 {
        normalized / 12.92
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4)
    }
}

pub fn srgb_encode_channel(value: f64) -> u8 {
    let normalized = if value <= 0.04045 / 12.92 {
        value * 12.92
    } else {
        value.powf(1.0 / 2.4) * 1.055 - 0.055
    };
    (normalized * 255.0).round() as u8
}

impl From<Srgb8> for LinearRgb {
    fn from(srgb: Srgb8) -> LinearRgb {
        LinearRgb { data: [
            srgb_decode_channel(srgb.channels()[0]),
            srgb_decode_channel(srgb.channels()[1]),
            srgb_decode_channel(srgb.channels()[2]),
        ] }
    }
}

impl From<LinearRgb> for Srgb8 {
    fn from(rgb: LinearRgb) -> Srgb8 {
        image::Rgb([
            srgb_encode_channel(rgb.data[0]),
            srgb_encode_channel(rgb.data[1]),
            srgb_encode_channel(rgb.data[2]),
        ])
    }
}

impl From<LinearRgb> for Xyz {
    fn from(rgb: LinearRgb) -> Xyz {
        Xyz { data: [
            0.4124*rgb.data[0] + 0.3576*rgb.data[1] + 0.1805*rgb.data[2],
            0.2126*rgb.data[0] + 0.7152*rgb.data[1] + 0.0722*rgb.data[2],
            0.0193*rgb.data[0] + 0.1192*rgb.data[1] + 0.9505*rgb.data[2]
        ] }
    }
}

impl From<Xyz> for LinearRgb {
    fn from(xyz: Xyz) -> LinearRgb {
        LinearRgb { data: [
             3.2406 * xyz.data[0] - 1.5372 * xyz.data[1] - 0.4986 * xyz.data[2],
            -0.9689 * xyz.data[0] + 1.8758 * xyz.data[1] + 0.0415 * xyz.data[2],
             0.0557 * xyz.data[0] - 0.2040 * xyz.data[1] + 1.0570 * xyz.data[2]
        ] }
    }
}

impl From<Xyz> for Lab {
    fn from(xyz: Xyz) -> Lab {
        fn f(value: f64) -> f64 {
            let delta: f64 = 6.0/29.0;
            if value > delta.powi(3) {
                value.powf(1.0/3.0)
            } else {
                value / (3.0 * delta.powi(2)) + 4.0/29.0
            }
        }

        let fx = f(xyz.data[0] / 0.9505);
        let fy = f(xyz.data[1]);
        let fz = f(xyz.data[2] / 1.089);

        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (fx - fy);
        let b = 200.0 * (fy - fz);

        Lab { l, a, b, c: a.hypot(b) }
    }
}

impl From<Xyz> for PseudoLab {
    fn from(xyz: Xyz) -> PseudoLab {
        fn f(value: f64) -> f64 {
            // linear approximation to x^(1/3)
            value * 0.78 + 0.325
        }
        
        let fx = f(xyz.data[0] / 0.9505);
        let fy = f(xyz.data[1]);
        let fz = f(xyz.data[2] / 1.089);

        let l = 1.16 * fy - 0.16;
        let a = 5.0 * (fx - fy);
        let b = 2.0 * (fy - fz);

        PseudoLab { data: [l, a, b] }
    }
}

impl From<PseudoLab> for Xyz {
    fn from(plab: PseudoLab) -> Xyz {
        fn invf(value: f64) -> f64 {
            (value - 0.325) / 0.78
        }

        let fy = (plab.data[0] + 0.16) / 1.16;
        let fx = plab.data[1] / 5.0 + fy;
        let fz = fy - plab.data[2] / 2.0;

        Xyz { data: [invf(fx) * 0.9505, invf(fy), invf(fz) * 1.089] }
    }
}

impl From<PseudoLab> for Lab {
    fn from(plab: PseudoLab) -> Lab {
        Lab::from(Xyz::from(plab))
    }
}

impl From<LinearRgb> for PseudoLab {
    fn from(rgb: LinearRgb) -> PseudoLab {
        PseudoLab::from(Xyz::from(rgb))
    }
}

impl From<LinearRgb> for Lab {
    fn from(rgb: LinearRgb) -> Lab {
        Lab::from(Xyz::from(rgb))
    }
}

impl From<Srgb8> for Lab {
    fn from(srgb: Srgb8) -> Lab {
        Lab::from(Xyz::from(LinearRgb::from(srgb)))
    }
}

//////// Color difference ////////

impl Lab {
    /// Using the "graphic arts" constants
    pub fn cie1994_distance2(lab1: Lab, lab2: Lab) -> f64 {
        let diff2_l = (lab1.l - lab2.l).powi(2);
        let diff2_chroma = (lab1.c - lab2.c).powi(2);
        let diff2_hue = (lab1.a - lab2.a).powi(2) + (lab1.b - lab2.b).powi(2) - diff2_chroma;

        let sc = 1.0 + 0.045 * lab1.c;
        let sh = 1.0 + 0.015 * lab1.c;

        diff2_l + diff2_chroma / sc.powi(2) + diff2_hue / sh.powi(2)
    }

    // From Warren D. Smith's Color-Packings and Color-Distances, 2018 (https://rangevoting.org/ColorPack.html)
    pub fn sym_cie1994_distance2(lab1: Lab, lab2: Lab) -> f64 {
        let diff2_l = (lab1.l - lab2.l).powi(2);
        let diff2_chroma = (lab1.c - lab2.c).powi(2);
        let diff2_hue = (lab1.a - lab2.a).powi(2) + (lab1.b - lab2.b).powi(2) - diff2_chroma;

        let avg_chroma = (lab1.c + lab2.c) / 2.0;
        let sc = 1.0 + 0.045 * avg_chroma;
        let sh = 1.0 + 0.015 * avg_chroma;

        diff2_l + diff2_chroma / sc.powi(2) + diff2_hue / sh.powi(2)
    }

    // From Warren D. Smith's Color-Packings and Color-Distances, 2018 (https://rangevoting.org/ColorPack.html)
    pub fn wds_cie1994_distance2(lab1: Lab, lab2: Lab) -> f64 {
        let base_dist = Lab::sym_cie1994_distance2(lab1, lab2);
        205.85012080886 * base_dist / (100.0 + base_dist.powf(82.0 / 81.0))
    }

    pub fn ciede2000_distance2(lab1: Lab, lab2: Lab) -> f64 {
        use core::f64::consts::PI;
        const TAU: f64 = PI * 2.0;

        // This is a fairly direct translation from Wikipedia's description, with a few optimizations
        let l1 = lab1.l;
        let l2 = lab2.l;
        let a1 = lab1.a;
        let a2 = lab2.a;
        let b1 = lab1.b;
        let b2 = lab2.b;
    
        let c1 = lab1.c;
        let c2 = lab2.c;

        let diff_l_adj = l2 - l1;
        let avg_l = (l1 + l2) / 2.0;
        let avg_c = (c1 + c2) / 2.0;
        let adj_coeff = 1.5 - 0.5 * (avg_c.powi(7) / (avg_c.powi(7) + 25.0_f64.powi(7))).sqrt();
        let a1_adj = a1 * adj_coeff;
        let a2_adj = a2 * adj_coeff;
        let c1_adj = (a1_adj.powi(2) + b1.powi(2)).sqrt(); // This is faster than .hypot and the inaccuracy is slight enough to be ignored
        let c2_adj = (a2_adj.powi(2) + b2.powi(2)).sqrt();
        let avg_c_adj = (c1_adj + c2_adj) / 2.0;
        let diff_c_adj = c2_adj - c1_adj;
        let h1_adj = b1.atan2(a1_adj);
        let h2_adj = b2.atan2(a2_adj);
        let diff_h_adj = if (h2_adj - h1_adj).abs() <= PI {
            h2_adj - h1_adj
        } else if h2_adj <= h1_adj {
            h2_adj - h1_adj + TAU
        } else {
            h2_adj - h1_adj - TAU
        };
        let diff_H_adj = 2.0 * (c1_adj * c2_adj).sqrt() * (diff_h_adj / 2.0).sin();
        let avg_h_adj = if (h2_adj - h1_adj).abs() <= PI {
            (h1_adj + h2_adj) / 2.0
        } else if h1_adj + h2_adj < TAU {
            (h1_adj + h2_adj) / 2.0 + PI
        } else {
            (h1_adj + h2_adj) / 2.0 - PI
        };
        // We use multi-angle formulas to only compute sin/cos once
        //
        //let t = 1.0 - 0.17 * (avg_h_adj - TAU / 12.0).cos()
        //            + 0.24 * (2.0 * avg_h_adj).cos()
        //            + 0.32 * (3.0 * avg_h_adj + TAU / 60.0).cos()
        //            - 0.2 * (4.0 * avg_h_adj - TAU * 63.0 / 360.0).cos();
        let (sin_avg_h_adj, cos_avg_h_adj) = avg_h_adj.sin_cos();
        let cos_twice_avg_h_adj = cos_avg_h_adj.powi(2) - sin_avg_h_adj.powi(2);
        let sin_twice_avg_h_adj = cos_avg_h_adj * sin_avg_h_adj * 2.0;
        let t = 1.0 - cos_avg_h_adj * 1.1019653381968969733956172062093467174501
                    - sin_avg_h_adj * 0.1853473247369473325438407886103981943174
                    + cos_twice_avg_h_adj * 0.24
                    + cos_avg_h_adj.powi(3) * 1.2729880260713898712610456895751300883466
                    + sin_avg_h_adj.powi(3) * 0.1337964329825964433917877181471975924232
                    - (cos_twice_avg_h_adj.powi(2) - sin_twice_avg_h_adj.powi(2)) * 0.0907980999479093583120816732715742397966
                    - cos_twice_avg_h_adj * sin_twice_avg_h_adj * 0.3564026096753471449438838285654505251082;

        let sl = 1.0 + 0.015 * (avg_l - 50.0).powi(2) / (20.0 + (avg_l - 50.0).powi(2)).sqrt();
        let sc = 1.0 + 0.045 * avg_c_adj;
        let sh = 1.0 + 0.015 * avg_c_adj * t;
        let rt = -2.0 * (avg_c_adj.powi(7) / (avg_c_adj.powi(7) + 25.0_f64.powi(7))).sqrt() * (TAU / 6.0 * (-(avg_h_adj / 25.0f64.to_radians() - 11.0).powi(2)).exp()).sin();

        (diff_l_adj / sl).powi(2) + (diff_c_adj / sc).powi(2) + (diff_H_adj / sh).powi(2) + rt * (diff_c_adj / sc) * (diff_H_adj / sh)
    }

    // From Warren D. Smith's Color-Packings and Color-Distances, 2018 (https://rangevoting.org/ColorPack.html)
    pub fn cont_ciede2000_distance2(lab1: Lab, lab2: Lab) -> f64 {
        use core::f64::consts::PI;
        const TAU: f64 = PI * 2.0;

        // This is a fairly direct translation from Wikipedia's description, with a few optimizations
        let l1 = lab1.l;
        let l2 = lab2.l;
        let a1 = lab1.a;
        let a2 = lab2.a;
        let b1 = lab1.b;
        let b2 = lab2.b;

        let c1 = lab1.c;
        let c2 = lab2.c;

        let diff_l_adj = l2 - l1;
        let avg_l = (l1 + l2) / 2.0;
        let avg_c = (c1 + c2) / 2.0;
        let adj_coeff = 1.5 - 0.5 * (avg_c.powi(7) / (avg_c.powi(7) + 25.0_f64.powi(7))).sqrt();
        let a1_adj = a1 * adj_coeff;
        let a2_adj = a2 * adj_coeff;
        let c1_adj = (a1_adj.powi(2) + b1.powi(2)).sqrt(); // This is faster than .hypot and the inaccuracy is slight enough to be ignored
        let c2_adj = (a2_adj.powi(2) + b2.powi(2)).sqrt();
        let avg_c_adj = (c1_adj + c2_adj) / 2.0;
        let diff_c_adj = c2_adj - c1_adj;
        let h1_adj = b1.atan2(a1_adj);
        let h2_adj = b2.atan2(a2_adj);
        let diff_h_adj = if (h2_adj - h1_adj).abs() <= PI {
            h2_adj - h1_adj
        } else if h2_adj <= h1_adj {
            h2_adj - h1_adj + TAU
        } else {
            h2_adj - h1_adj - TAU
        };
        let diff_H_adj = 2.0 * (c1_adj * c2_adj).sqrt() * (diff_h_adj / 2.0).sin();
        let avg_h_adj = if (h2_adj - h1_adj).abs() <= PI {
            (h1_adj + h2_adj) / 2.0
        } else if h1_adj + h2_adj < TAU {
            (h1_adj + h2_adj) / 2.0 + PI
        } else {
            (h1_adj + h2_adj) / 2.0 - PI
        };

        // TODO: Use multi-angle formulas to only compute sin/cos once
        let t = 1.0 + 0.24 * (2.0 * avg_h_adj).cos()
                    - 0.2 * (4.0 * avg_h_adj - TAU * 63.0 / 360.0).cos()
                    + (0.32 * (3.0 * avg_h_adj + TAU / 60.0).cos() - 0.17 * (avg_h_adj - TAU / 12.0).cos()) * (diff_h_adj * 0.5).cos();
        let sin_dro = if (avg_h_adj - 275.0f64.to_radians()).abs() >= 85.0f64.to_radians() {
            0.0
        } else {
            let exponent = (avg_h_adj / 25.0f64.to_radians() - 11.0).powi(2);
            let mul = 1.0 - exponent * (25.0 / 289.0);
            (TAU / 6.0 * (-exponent).exp() * mul).sin()
        };

        let sl = 1.0 + 0.015 * (avg_l - 50.0).powi(2) / (20.0 + (avg_l - 50.0).powi(2)).sqrt();
        let sc = 1.0 + 0.045 * avg_c_adj;
        let sh = 1.0 + 0.015 * avg_c_adj * t;
        let rt = -2.0 * (avg_c_adj.powi(7) / (avg_c_adj.powi(7) + 25.0_f64.powi(7))).sqrt() * sin_dro;
        let rt_repair = if diff_h_adj.abs() > 140.0f64.to_radians() {
            rt * (4.5 - diff_h_adj.abs() / 40.0f64.to_radians())
        } else {
            rt
        };

        (diff_l_adj / sl).powi(2) + (diff_c_adj / sc).powi(2) + (diff_H_adj / sh).powi(2) + rt_repair * (diff_c_adj / sc) * (diff_H_adj / sh)
    }
}
