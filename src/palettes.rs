use crate::color::{self, Srgb8, LinearRgb, Lab};
use crate::geom::{determinant, subtract, midpoint};

use image::Pixel;

pub fn grid(r_levels: usize, g_levels: usize, b_levels: usize) -> Vec<Srgb8> {
    let mut palette = Vec::with_capacity(r_levels * g_levels * b_levels);
    for r_idx in 0..r_levels {
        let r = (r_idx * 255 + (r_levels - 1) / 2) / (r_levels - 1);
        for g_idx in 0..g_levels {
            let g = (g_idx * 255 + (g_levels - 1) / 2) / (g_levels - 1);
            for b_idx in 0..b_levels {
                let b = (b_idx * 255 + (b_levels - 1) / 2) / (b_levels - 1);
                palette.push(image::Rgb([r as u8, g as u8, b as u8]));
            }
        }
    }
    palette
}

pub const RGBI: [Srgb8; 16] = [
    image::Rgb([0x00,0x00,0x00]),
    image::Rgb([0xff,0x00,0x00]),
    image::Rgb([0x00,0xff,0x00]),
    image::Rgb([0xff,0xff,0x00]),
    image::Rgb([0x00,0x00,0xff]),
    image::Rgb([0xff,0x00,0xff]),
    image::Rgb([0x00,0xff,0xff]),
    image::Rgb([0xff,0xff,0xff]),
    image::Rgb([0x55,0x55,0x55]),
    image::Rgb([0xaa,0x55,0x55]),
    image::Rgb([0x55,0xaa,0x55]),
    image::Rgb([0xaa,0xaa,0x55]),
    image::Rgb([0x55,0x55,0xaa]),
    image::Rgb([0xaa,0x55,0xaa]),
    image::Rgb([0x55,0xaa,0xaa]),
    image::Rgb([0xaa,0xaa,0xaa]),
];

pub const MICROSOFT16: [Srgb8; 16] = [
    image::Rgb([0x00,0x00,0x00]),
    image::Rgb([0x80,0x00,0x00]),
    image::Rgb([0x00,0x80,0x00]),
    image::Rgb([0x80,0x80,0x00]),
    image::Rgb([0x00,0x00,0x80]),
    image::Rgb([0x80,0x00,0x80]),
    image::Rgb([0x00,0x80,0x80]),
    image::Rgb([0xc0,0xc0,0xc0]),
    image::Rgb([0x80,0x80,0x80]),
    image::Rgb([0xff,0x00,0x00]),
    image::Rgb([0x00,0xff,0x00]),
    image::Rgb([0xff,0xff,0x00]),
    image::Rgb([0x00,0x00,0xff]),
    image::Rgb([0xff,0x00,0xff]),
    image::Rgb([0x00,0xff,0xff]),
    image::Rgb([0xff,0xff,0xff]),
];

pub const MACINTOSH16: [Srgb8; 16] = [
    image::Rgb([0xff,0xff,0xff]),
    image::Rgb([0xfb,0xf3,0x05]),
    image::Rgb([0xff,0x64,0x03]),
    image::Rgb([0xdd,0x09,0x07]),
    image::Rgb([0xf2,0x08,0x84]),
    image::Rgb([0x47,0x00,0xa5]),
    image::Rgb([0x00,0x00,0xd3]),
    image::Rgb([0x02,0xab,0xea]),
    image::Rgb([0x1f,0xb7,0x14]),
    image::Rgb([0x00,0x64,0x12]),
    image::Rgb([0x56,0x2c,0x05]),
    image::Rgb([0x90,0x71,0x3a]),
    image::Rgb([0xc0,0xc0,0xc0]),
    image::Rgb([0x80,0x80,0x80]),
    image::Rgb([0x40,0x40,0x40]),
    image::Rgb([0x00,0x00,0x00]),
];

// Lehn-Stern reallysafe subset of the websafe palette
pub const REALLYSAFE: [Srgb8; 22] = [
    image::Rgb([0x00,0x00,0x00]),
    image::Rgb([0xFF,0x00,0x00]),
    image::Rgb([0x00,0x00,0x33]),
    image::Rgb([0xFF,0x00,0x33]),
    image::Rgb([0x00,0x00,0xFF]),
    image::Rgb([0xFF,0x00,0xFF]),
    image::Rgb([0x00,0xFF,0x00]),
    image::Rgb([0x66,0xFF,0x00]),
    image::Rgb([0xFF,0xFF,0x00]),
    image::Rgb([0x33,0xFF,0x33]),
    image::Rgb([0x66,0xFF,0x33]),
    image::Rgb([0xFF,0xFF,0x33]),
    image::Rgb([0x00,0xFF,0x66]),
    image::Rgb([0x33,0xFF,0x66]),
    image::Rgb([0xCC,0xFF,0x66]),
    image::Rgb([0xFF,0xFF,0x66]),
    image::Rgb([0x00,0xFF,0xCC]),
    image::Rgb([0x33,0xFF,0xCC]),
    image::Rgb([0x00,0xFF,0xFF]),
    image::Rgb([0x33,0xFF,0xFF]),
    image::Rgb([0x66,0xFF,0xFF]),
    image::Rgb([0xFF,0xFF,0xFF]),
];

// Taken from https://bisqwit.iki.fi/story/howto/dither/jy/
// Used to demonstrate the Yliluoma dithering algorithms
pub const YLILUOMA_EXAMPLE: [Srgb8; 16] = [
    image::Rgb([0x08,0x00,0x00]),
    image::Rgb([0x23,0x43,0x09]),
    image::Rgb([0x2b,0x34,0x7c]),
    image::Rgb([0x6a,0x94,0xab]),
    image::Rgb([0x20,0x1a,0x0b]),
    image::Rgb([0x5d,0x4f,0x1e]),
    image::Rgb([0x2b,0x74,0x09]),
    image::Rgb([0xd5,0xc4,0xb3]),
    image::Rgb([0x43,0x28,0x17]),
    image::Rgb([0x9c,0x6b,0x20]),
    image::Rgb([0xd0,0xca,0x40]),
    image::Rgb([0xfc,0xe7,0x6e]),
    image::Rgb([0x49,0x29,0x10]),
    image::Rgb([0xa9,0x22,0x0f]),
    image::Rgb([0xe8,0xa0,0x77]),
    image::Rgb([0xfc,0xfa,0xe2]),
];

// A palette selected to work well for the example used in the
// description of Yliluoma's dithering algorithms, making sure to bound
// all the colors.
pub const YLILUOMA_EXAMPLE_ALTERNATE: [Srgb8; 16] = [
    image::Rgb([0x00,0x00,0x00]),
    image::Rgb([0xff,0x00,0x00]),
    image::Rgb([0x00,0xff,0x00]),
    image::Rgb([0xff,0xff,0x00]),
    image::Rgb([0x00,0x00,0xff]),
    image::Rgb([0xff,0x00,0xff]),
    image::Rgb([0x00,0xff,0xff]),
    image::Rgb([0xff,0xff,0xff]),
    image::Rgb([0x22,0x1d,0x12]),
    image::Rgb([0xd4,0xe7,0xdc]),
    image::Rgb([0x53,0x63,0x57]),
    image::Rgb([0x6d,0x40,0x1a]),
    image::Rgb([0x96,0x76,0x32]),
    image::Rgb([0xaf,0xb8,0x6c]),
    image::Rgb([0x74,0xa5,0xbc]),
    image::Rgb([0x93,0x88,0x5f]),
];

// Taken from https://petzforum.proboards.com/thread/36497/petz-color-hex-codes/
pub const PETZ_SOURCE: [Srgb8; 256] = [
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0x80, 0x00, 0x00]),
    image::Rgb([0x00, 0x80, 0x00]),
    image::Rgb([0x80, 0x80, 0x00]),
    image::Rgb([0x00, 0x00, 0x80]),
    image::Rgb([0x80, 0x00, 0x80]),
    image::Rgb([0x00, 0x80, 0x80]),
    image::Rgb([0xC0, 0xC0, 0xC0]),
    image::Rgb([0xD4, 0xD0, 0xC8]),
    image::Rgb([0x40, 0x80, 0x80]),
    image::Rgb([0xE7, 0xE2, 0xDD]),
    image::Rgb([0xE3, 0xDE, 0xD8]),
    image::Rgb([0xDF, 0xDA, 0xD4]),
    image::Rgb([0xDB, 0xD6, 0xD0]),
    image::Rgb([0xD7, 0xD2, 0xCC]),
    image::Rgb([0xD3, 0xCE, 0xC7]),
    image::Rgb([0xCF, 0xCA, 0xC3]),
    image::Rgb([0xCB, 0xC6, 0xBF]),
    image::Rgb([0xC7, 0xC2, 0xBB]),
    image::Rgb([0xC3, 0xBE, 0xB6]),
    image::Rgb([0x75, 0x75, 0x75]),
    image::Rgb([0x6F, 0x6F, 0x6F]),
    image::Rgb([0x6A, 0x6A, 0x6A]),
    image::Rgb([0x65, 0x65, 0x65]),
    image::Rgb([0x60, 0x60, 0x60]),
    image::Rgb([0x5B, 0x5B, 0x5B]),
    image::Rgb([0x56, 0x56, 0x56]),
    image::Rgb([0x51, 0x51, 0x51]),
    image::Rgb([0x4C, 0x4C, 0x4C]),
    image::Rgb([0x46, 0x46, 0x46]),
    image::Rgb([0x42, 0x42, 0x42]),
    image::Rgb([0x3A, 0x3A, 0x3A]),
    image::Rgb([0x33, 0x33, 0x33]),
    image::Rgb([0x2C, 0x2C, 0x2C]),
    image::Rgb([0x24, 0x24, 0x24]),
    image::Rgb([0x1D, 0x1D, 0x1D]),
    image::Rgb([0x16, 0x16, 0x16]),
    image::Rgb([0x0E, 0x0E, 0x0E]),
    image::Rgb([0x07, 0x07, 0x07]),
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0xDC, 0xC2, 0x96]),
    image::Rgb([0xD5, 0xBB, 0x90]),
    image::Rgb([0xCF, 0xB5, 0x8A]),
    image::Rgb([0xCF, 0xB5, 0x8A]),
    image::Rgb([0xC2, 0xA9, 0x7F]),
    image::Rgb([0xBB, 0xA2, 0x7A]),
    image::Rgb([0xB5, 0x9C, 0x74]),
    image::Rgb([0xB5, 0x9C, 0x74]),
    image::Rgb([0xA8, 0x90, 0x69]),
    image::Rgb([0xA2, 0x89, 0x63]),
    image::Rgb([0x87, 0x41, 0x22]),
    image::Rgb([0x7F, 0x3D, 0x20]),
    image::Rgb([0x77, 0x39, 0x1E]),
    image::Rgb([0x70, 0x35, 0x1C]),
    image::Rgb([0x68, 0x31, 0x1A]),
    image::Rgb([0x61, 0x2D, 0x18]),
    image::Rgb([0x59, 0x29, 0x16]),
    image::Rgb([0x52, 0x25, 0x14]),
    image::Rgb([0x4A, 0x21, 0x12]),
    image::Rgb([0x42, 0x1D, 0x10]),
    image::Rgb([0xB4, 0x73, 0x16]),
    image::Rgb([0xAF, 0x6D, 0x13]),
    image::Rgb([0xAA, 0x68, 0x11]),
    image::Rgb([0xA5, 0x63, 0x0E]),
    image::Rgb([0xA1, 0x5E, 0x0C]),
    image::Rgb([0x9C, 0x58, 0x09]),
    image::Rgb([0x97, 0x53, 0x07]),
    image::Rgb([0x93, 0x4E, 0x04]),
    image::Rgb([0x8E, 0x49, 0x02]),
    image::Rgb([0x89, 0x44, 0x00]),
    image::Rgb([0xF0, 0x9E, 0xB7]),
    image::Rgb([0xE9, 0x99, 0xB2]),
    image::Rgb([0xE3, 0x95, 0xAD]),
    image::Rgb([0xDD, 0x91, 0xA8]),
    image::Rgb([0xD6, 0x8D, 0xA3]),
    image::Rgb([0xD0, 0x88, 0x9E]),
    image::Rgb([0xCA, 0x84, 0x99]),
    image::Rgb([0xC3, 0x80, 0x94]),
    image::Rgb([0xBD, 0x7C, 0x8F]),
    image::Rgb([0xB7, 0x77, 0x8B]),
    image::Rgb([0xA8, 0x29, 0x01]),
    image::Rgb([0xA4, 0x28, 0x01]),
    image::Rgb([0x9F, 0x27, 0x01]),
    image::Rgb([0x9B, 0x26, 0x01]),
    image::Rgb([0x97, 0x25, 0x01]),
    image::Rgb([0x92, 0x24, 0x01]),
    image::Rgb([0x8E, 0x23, 0x01]),
    image::Rgb([0x8A, 0x22, 0x01]),
    image::Rgb([0x85, 0x21, 0x01]),
    image::Rgb([0x81, 0x20, 0x01]),
    image::Rgb([0x6B, 0x4A, 0x0C]),
    image::Rgb([0x65, 0x44, 0x0B]),
    image::Rgb([0x60, 0x3E, 0x0B]),
    image::Rgb([0x5B, 0x39, 0x0B]),
    image::Rgb([0x56, 0x33, 0x0B]),
    image::Rgb([0x51, 0x2D, 0x0A]),
    image::Rgb([0x4C, 0x27, 0x0A]),
    image::Rgb([0x47, 0x22, 0x0A]),
    image::Rgb([0x42, 0x1C, 0x0A]),
    image::Rgb([0x3C, 0x16, 0x09]),
    image::Rgb([0xA6, 0x8A, 0x38]),
    image::Rgb([0xA2, 0x85, 0x37]),
    image::Rgb([0x9E, 0x81, 0x37]),
    image::Rgb([0x9A, 0x7D, 0x37]),
    image::Rgb([0x96, 0x78, 0x36]),
    image::Rgb([0x93, 0x74, 0x36]),
    image::Rgb([0x8F, 0x70, 0x36]),
    image::Rgb([0x8B, 0x6B, 0x35]),
    image::Rgb([0x87, 0x67, 0x35]),
    image::Rgb([0x84, 0x62, 0x35]),
    image::Rgb([0x62, 0x70, 0x7D]),
    image::Rgb([0x5D, 0x69, 0x76]),
    image::Rgb([0x58, 0x63, 0x70]),
    image::Rgb([0x53, 0x5D, 0x69]),
    image::Rgb([0x4E, 0x57, 0x63]),
    image::Rgb([0x49, 0x50, 0x5D]),
    image::Rgb([0x44, 0x4A, 0x56]),
    image::Rgb([0x3F, 0x44, 0x50]),
    image::Rgb([0x3A, 0x3E, 0x4A]),
    image::Rgb([0x36, 0x38, 0x43]),
    image::Rgb([0x9A, 0x8E, 0x73]),
    image::Rgb([0x96, 0x8A, 0x70]),
    image::Rgb([0x93, 0x87, 0x6D]),
    image::Rgb([0x90, 0x84, 0x6B]),
    image::Rgb([0x8C, 0x81, 0x68]),
    image::Rgb([0x89, 0x7E, 0x66]),
    image::Rgb([0x86, 0x7B, 0x63]),
    image::Rgb([0x82, 0x78, 0x61]),
    image::Rgb([0x7F, 0x75, 0x5E]),
    image::Rgb([0x7C, 0x71, 0x5B]),
    image::Rgb([0x55, 0xAB, 0x57]),
    image::Rgb([0x3C, 0xA1, 0x47]),
    image::Rgb([0x15, 0x99, 0x17]),
    image::Rgb([0x35, 0x83, 0x36]),
    image::Rgb([0x30, 0x7B, 0x1C]),
    image::Rgb([0x10, 0x79, 0x19]),
    image::Rgb([0x27, 0x62, 0x17]),
    image::Rgb([0x2F, 0x5E, 0x2B]),
    image::Rgb([0x13, 0x5C, 0x14]),
    image::Rgb([0x10, 0x41, 0x11]),
    image::Rgb([0x2B, 0x61, 0xC3]),
    image::Rgb([0x38, 0x46, 0xE3]),
    image::Rgb([0x33, 0x3B, 0xFF]),
    image::Rgb([0x33, 0x43, 0xCE]),
    image::Rgb([0x16, 0x1A, 0xD7]),
    image::Rgb([0x2E, 0x3C, 0xB6]),
    image::Rgb([0x16, 0x1C, 0xA9]),
    image::Rgb([0x2A, 0x42, 0x90]),
    image::Rgb([0x19, 0x22, 0x77]),
    image::Rgb([0x11, 0x19, 0x53]),
    image::Rgb([0xD8, 0xF0, 0xFF]),
    image::Rgb([0xAC, 0xE0, 0xFF]),
    image::Rgb([0x99, 0xD6, 0xFF]),
    image::Rgb([0x82, 0xCA, 0xFF]),
    image::Rgb([0x75, 0xB6, 0xE8]),
    image::Rgb([0x68, 0xC0, 0xFF]),
    image::Rgb([0x51, 0x96, 0xDC]),
    image::Rgb([0x18, 0x9C, 0xCE]),
    image::Rgb([0x56, 0x8B, 0xB9]),
    image::Rgb([0x1E, 0x89, 0xA9]),
    image::Rgb([0xEB, 0xED, 0xA7]),
    image::Rgb([0xEA, 0xEB, 0x90]),
    image::Rgb([0xD1, 0xCC, 0x77]),
    image::Rgb([0xF7, 0xF4, 0x00]),
    image::Rgb([0xED, 0xE8, 0x32]),
    image::Rgb([0xC3, 0xC4, 0x0B]),
    image::Rgb([0xC3, 0xC3, 0x34]),
    image::Rgb([0x9F, 0xA2, 0x17]),
    image::Rgb([0x9F, 0x9F, 0x43]),
    image::Rgb([0x6E, 0x7A, 0x2E]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0xC0, 0xE4, 0xE7]),
    image::Rgb([0xAC, 0xC6, 0xD5]),
    image::Rgb([0xA7, 0xA7, 0xB1]),
    image::Rgb([0xA0, 0xA0, 0xA8]),
    image::Rgb([0x74, 0xA0, 0xB6]),
    image::Rgb([0x83, 0x99, 0xB1]),
    image::Rgb([0x83, 0x99, 0xB1]),
    image::Rgb([0x80, 0x98, 0xB0]),
    image::Rgb([0x80, 0x98, 0xB0]),
    image::Rgb([0xE2, 0xBE, 0xAC]),
    image::Rgb([0xD5, 0x93, 0x90]),
    image::Rgb([0xD7, 0x76, 0x6E]),
    image::Rgb([0xB8, 0x73, 0x67]),
    image::Rgb([0x9F, 0x77, 0x73]),
    image::Rgb([0xA2, 0x6A, 0x5D]),
    image::Rgb([0x89, 0x64, 0x58]),
    image::Rgb([0x98, 0x57, 0x4D]),
    image::Rgb([0x6A, 0x45, 0x42]),
    image::Rgb([0x5A, 0x3C, 0x31]),
    image::Rgb([0x72, 0x9E, 0x8C]),
    image::Rgb([0x00, 0x80, 0x80]),
    image::Rgb([0x42, 0x7A, 0x75]),
    image::Rgb([0x00, 0x80, 0x80]),
    image::Rgb([0x39, 0x78, 0x60]),
    image::Rgb([0x3D, 0x5A, 0x63]),
    image::Rgb([0x26, 0x58, 0x47]),
    image::Rgb([0x21, 0x41, 0x2B]),
    image::Rgb([0x12, 0x39, 0x30]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0xF4, 0xF6, 0xD8]),
    image::Rgb([0xE9, 0xD8, 0xC2]),
    image::Rgb([0x2C, 0x5F, 0x59]),
    image::Rgb([0xD3, 0xF4, 0xC5]),
    image::Rgb([0xC4, 0xD3, 0x9F]),
    image::Rgb([0xFF, 0xC7, 0x1A]),
    image::Rgb([0xB0, 0xB8, 0x9B]),
    image::Rgb([0xAF, 0xB1, 0x77]),
    image::Rgb([0xA5, 0x94, 0x8E]),
    image::Rgb([0xAC, 0xA9, 0x8F]),
    image::Rgb([0xD7, 0xA0, 0x14]),
    image::Rgb([0xC6, 0x7F, 0x08]),
    image::Rgb([0xCA, 0x6E, 0x46]),
    image::Rgb([0x79, 0x8E, 0x61]),
    image::Rgb([0x99, 0x7E, 0x4D]),
    image::Rgb([0x80, 0x80, 0x80]),
    image::Rgb([0x65, 0x9B, 0x2A]),
    image::Rgb([0x00, 0xCB, 0x16]),
    image::Rgb([0xA7, 0x6C, 0x39]),
    image::Rgb([0xFF, 0x42, 0x00]),
    image::Rgb([0x97, 0x64, 0x42]),
    image::Rgb([0x99, 0x65, 0x2A]),
    image::Rgb([0xDD, 0x34, 0x6A]),
    image::Rgb([0x29, 0x44, 0x75]),
    image::Rgb([0x58, 0x69, 0xB5]),
    image::Rgb([0x42, 0x6B, 0x84]),
    image::Rgb([0x50, 0x64, 0x80]),
    image::Rgb([0x75, 0x59, 0x4A]),
    image::Rgb([0x0A, 0x24, 0x6A]),
    image::Rgb([0x80, 0x80, 0x80]),
    image::Rgb([0xD4, 0xD0, 0xC8]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0xD4, 0xD0, 0xC8]),
    image::Rgb([0xD4, 0xD0, 0xC8]),
    image::Rgb([0x80, 0x80, 0x80]),
    image::Rgb([0x0A, 0x24, 0x6A]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0xD4, 0xD0, 0xC8]),
    image::Rgb([0x0A, 0x24, 0x6A]),
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0xFF, 0xFF, 0xFF]),
    image::Rgb([0x80, 0x80, 0x80]),
    image::Rgb([0x00, 0x00, 0x00]),
    image::Rgb([0x80, 0x80, 0x80]),
    image::Rgb([0xFF, 0x00, 0x00]),
    image::Rgb([0x00, 0xFF, 0x00]),
    image::Rgb([0xFF, 0xFF, 0x00]),
    image::Rgb([0x00, 0x00, 0xFF]),
    image::Rgb([0xFF, 0x00, 0xFF]),
    image::Rgb([0x00, 0xFF, 0xFF]),
    image::Rgb([0xD4, 0xD0, 0xC8]),
];

// Simplex dithering wants small simplices containing every target color, so we attempt to build a palette explicitly for
// that purpose. We start with the RGB cube, divided into 6 simplices based on hue (all sharing white and black). Then, since
// that palette only contains 8 colors, we repeatedly split simplices in half by introducing a new color on their longest
// edge, up until we have the desired number of palette colors. Currently, simplices are chosen for splitting based on a
// heuristic squaring the maximal perceptual distance withing the simplex (since large simplices benefit from splitting more)
// and multiplying by the number of target points inside that simplex (since those splits will help a larger fraction of the
// image).
//
// As a final post-processing pass, each simplex is shrunk to fit the colors it contains to attempt to reduce the error.
pub fn make_simplex_palette(palette_size: usize, pixels: impl Iterator<Item=image::Rgb<u8>>, distance2: fn(Lab, Lab) -> f64) -> Vec<image::Rgb<u8>> {
    struct SimplexCut {
        vertices_rgb: [Srgb8; 4],
        vertices_lin: [LinearRgb; 4],
        vertices_lab: [Lab; 4],
        diameter2: f64,
        diameter_edge: [usize; 2],
        points: Vec<[f64; 4]>
    }

    impl SimplexCut {
        // The heuristic used to select which simplex to cut
        fn weight(&self) -> f64 {
            self.diameter2 * self.points.len() as f64
        }

        // Shrink to fit contained points
        fn optimize(&mut self, referenced_points: &mut std::collections::HashMap<image::Rgb<u8>, usize>, distance2: fn(Lab, Lab) -> f64) {
            let mut changed = false;
            // Loop through all edges and maximally shrink that edge
            for opt_vertex in 0..4 {
                for other_vertex in (0..4).filter(|&v| v != opt_vertex) {
                    let mut max_ratio = 0.0;
                    for coords in &self.points {
                        let ratio = coords[opt_vertex] / coords[other_vertex];
                        if ratio > max_ratio {
                            max_ratio = ratio;
                        }
                    }
                    if max_ratio < 1e15 {
                        let factor = 1.0 - 1.0 / (max_ratio + 1.0);
                        //eprintln!("  Optimization factor {}", factor);
                        let new_lin = subtract(self.vertices_lin[opt_vertex], self.vertices_lin[other_vertex]) * factor + self.vertices_lin[other_vertex];
                        let new_rgb = Srgb8::from(new_lin);
                        if new_rgb != self.vertices_rgb[opt_vertex] &&
                           (referenced_points.get(&self.vertices_rgb[opt_vertex]) == Some(&1) || referenced_points.contains_key(&new_rgb)) {
                            changed = true;
                            let new_lin = LinearRgb::from(new_rgb);
                            let new_lab = Lab::from(new_lin);
                            match referenced_points.entry(self.vertices_rgb[opt_vertex]) {
                                std::collections::hash_map::Entry::Occupied(mut occ_entry) => {
                                    *occ_entry.get_mut() -= 1;
                                    if *occ_entry.get() == 0 {
                                        occ_entry.remove();
                                    }
                                },
                                _ => unreachable!()
                            }
                            self.vertices_rgb[opt_vertex] = new_rgb;
                            self.vertices_lin[opt_vertex] = new_lin;
                            self.vertices_lab[opt_vertex] = new_lab;
                            *referenced_points.entry(new_rgb).or_insert(0) += 1;
                            for coords in &mut self.points {
                                if factor < 1e-15 {
                                    coords[opt_vertex] += coords[other_vertex];
                                    coords[other_vertex] = 0.0;
                                } else {
                                    coords[opt_vertex] /= factor;
                                    coords[other_vertex] -= coords[opt_vertex] * (1.0 - factor);
                                }
                            }
                        }
                    }
                }
            }
            if changed {
                let mut max_distance_seen = 0.0;
                let mut max_distance_edge = [0, 1];
                for &edge in &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]] {
                    let dist2 = distance2(self.vertices_lab[edge[0]], self.vertices_lab[edge[1]]);
                    if dist2 > max_distance_seen {
                        max_distance_seen = dist2;
                        max_distance_edge = edge;
                    }
                }
                self.diameter2 = max_distance_seen;
                self.diameter_edge = max_distance_edge;
            }
        }
    }

    impl std::cmp::PartialEq for SimplexCut {
        fn eq(&self, other: &SimplexCut) -> bool {
            self.weight() == other.weight()
        }
    }
    impl std::cmp::Eq for SimplexCut { }
    impl std::cmp::PartialOrd for SimplexCut {
        fn partial_cmp(&self, other: &SimplexCut) -> Option<std::cmp::Ordering> {
            self.weight().partial_cmp(&other.weight())
        }
    }
    impl std::cmp::Ord for SimplexCut {
        fn cmp(&self, other: &SimplexCut) -> std::cmp::Ordering {
            self.weight().partial_cmp(&other.weight()).unwrap()
        }
    }

    let mut palette = Vec::with_capacity(palette_size);

    // Build the initial simplices by subdividing the RGB cube
    let mut hue_split_points = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for pixel in pixels {
        if pixel.channels()[0] < pixel.channels()[1] {
            if pixel.channels()[1] < pixel.channels()[2] {
                // Cyan-Blue
                hue_split_points[3].push(pixel);
            } else if pixel.channels()[2] < pixel.channels()[0] {
                // Yellow-Green
                hue_split_points[1].push(pixel);
            } else {
                // Cyan-Green
                hue_split_points[2].push(pixel);
            }
        } else {
            if pixel.channels()[0] < pixel.channels()[2] {
                // Magenta-Blue
                hue_split_points[4].push(pixel);
            } else if pixel.channels()[2] < pixel.channels()[1] {
                // Yellow-Red
                hue_split_points[0].push(pixel);
            } else {
                // Magenta-Red
                hue_split_points[5].push(pixel);
            }
        }
    }

    let mut nodes = std::collections::BinaryHeap::new();
    let mut referenced_points = std::collections::HashMap::new();

    let black_rgb = image::Rgb([0, 0, 0]);
    let white_rgb = image::Rgb([255, 255, 255]);
    let black_lin = LinearRgb::from(black_rgb);
    let white_lin = LinearRgb::from(white_rgb);
    let black_lab = Lab::from(black_lin);
    let white_lab = Lab::from(white_lin);

    // Now that the image pixels are divided, actually build the SimplexCut nodes to contain them
    for hue_idx in 0..6 {
        let cube_corners = [
            image::Rgb([255, 0, 0]),
            image::Rgb([255, 255, 0]),
            image::Rgb([0, 255, 0]),
            image::Rgb([0, 255, 255]),
            image::Rgb([0, 0, 255]),
            image::Rgb([255, 0, 255]),
            image::Rgb([255, 0, 0]),
        ];

        let prev_rgb = cube_corners[hue_idx];
        let next_rgb = cube_corners[hue_idx + 1];
        let prev_lin = LinearRgb::from(prev_rgb);
        let next_lin = LinearRgb::from(next_rgb);
        let prev_lab = Lab::from(prev_lin);
        let next_lab = Lab::from(next_lin);

        let node = SimplexCut {
            vertices_rgb: [black_rgb, white_rgb, prev_rgb, next_rgb],
            vertices_lin: [black_lin, white_lin, prev_lin, next_lin],
            vertices_lab: [black_lab, white_lab, prev_lab, next_lab],
            diameter2: 10000.0,
            diameter_edge: [0, 1],
            points: hue_split_points[hue_idx].iter().map(|&rgb| {
                let lin = LinearRgb::from(rgb);
                let shifted_points = [
                    subtract(black_lin, lin),
                    subtract(white_lin, lin),
                    subtract(prev_lin, lin),
                    subtract(next_lin, lin),
                ];
                // Tag with barycentric coords
                let d0 = determinant([shifted_points[1], shifted_points[3], shifted_points[2]]);
                let d1 = determinant([shifted_points[0], shifted_points[2], shifted_points[3]]);
                let d2 = determinant([shifted_points[0], shifted_points[3], shifted_points[1]]);
                let d3 = determinant([shifted_points[0], shifted_points[1], shifted_points[2]]);
                let d_all = d0 + d1 + d2 + d3;

                [d0 / d_all, d1 / d_all, d2 / d_all, d3 / d_all]
            }).collect()
        };
        if !node.points.is_empty() {
            for &vertex in &node.vertices_rgb {
                *referenced_points.entry(vertex).or_insert(0) += 1;
            }
            nodes.push(node);
        }
    }

    // Cut simplices until we're done!
    while !nodes.is_empty() && referenced_points.len() < palette_size {
        let mut split_node = nodes.pop().unwrap();
        eprintln!("{}, {}, {}, {}", nodes.len(), referenced_points.len(), split_node.points.len(), split_node.diameter2);
        //eprintln!("  #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}",
        //    split_node.vertices_rgb[0].channels()[0], split_node.vertices_rgb[0].channels()[1], split_node.vertices_rgb[0].channels()[2],
        //    split_node.vertices_rgb[1].channels()[0], split_node.vertices_rgb[1].channels()[1], split_node.vertices_rgb[1].channels()[2],
        //    split_node.vertices_rgb[2].channels()[0], split_node.vertices_rgb[2].channels()[1], split_node.vertices_rgb[2].channels()[2],
        //    split_node.vertices_rgb[3].channels()[0], split_node.vertices_rgb[3].channels()[1], split_node.vertices_rgb[3].channels()[2],
        //);
        split_node.optimize(&mut referenced_points, distance2);
        //eprintln!("  #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}, #{:02x}{:02x}{:02x}",
        //    split_node.vertices_rgb[0].channels()[0], split_node.vertices_rgb[0].channels()[1], split_node.vertices_rgb[0].channels()[2],
        //    split_node.vertices_rgb[1].channels()[0], split_node.vertices_rgb[1].channels()[1], split_node.vertices_rgb[1].channels()[2],
        //    split_node.vertices_rgb[2].channels()[0], split_node.vertices_rgb[2].channels()[1], split_node.vertices_rgb[2].channels()[2],
        //    split_node.vertices_rgb[3].channels()[0], split_node.vertices_rgb[3].channels()[1], split_node.vertices_rgb[3].channels()[2],
        //);
        //eprintln!("  Diameter edge: {}, {}", split_node.diameter_edge[0], split_node.diameter_edge[1]);
       
        // Choose the division point
        if split_node.points.len() == 1 {
            continue;
        }
        let end0 = split_node.vertices_rgb[split_node.diameter_edge[0]];
        let end1 = split_node.vertices_rgb[split_node.diameter_edge[1]];
        let split_vertex_lin = midpoint(split_node.vertices_lin[split_node.diameter_edge[0]], split_node.vertices_lin[split_node.diameter_edge[1]]);
        let split_vertex_rgb = Srgb8::from(split_vertex_lin);
        //eprintln!("  Split at #{:02x}{:02x}{:02x}", split_vertex_rgb.channels()[0], split_vertex_rgb.channels()[1], split_vertex_rgb.channels()[2]);
        if split_vertex_rgb == end0 || split_vertex_rgb == end1 {
            continue;
        }
        let split_vertex_lin = LinearRgb::from(split_vertex_rgb);
        let split_vertex_lab = Lab::from(split_vertex_lin);
        let other0 = (0..4).find(|&x| x != split_node.diameter_edge[0] && x != split_node.diameter_edge[1]).unwrap();
        let other1 = (0..4).rfind(|&x| x != split_node.diameter_edge[1] && x != split_node.diameter_edge[0]).unwrap();

        // Divide the pixels between the two halves
        let mut split_points = [Vec::new(), Vec::new()];
        for coords in split_node.points {
            if coords[split_node.diameter_edge[0]] > coords[split_node.diameter_edge[1]] {
                split_points[0].push([
                    coords[split_node.diameter_edge[0]] - coords[split_node.diameter_edge[1]],
                    coords[split_node.diameter_edge[1]] * 2.0,
                    coords[other0],
                    coords[other1]
                ]);
            } else {
                split_points[1].push([
                    coords[split_node.diameter_edge[1]] - coords[split_node.diameter_edge[0]],
                    coords[split_node.diameter_edge[0]] * 2.0,
                    coords[other0],
                    coords[other1]
                ]);
            }
        }

        let new_vertices_rgb = [
            [end0, split_vertex_rgb, split_node.vertices_rgb[other0], split_node.vertices_rgb[other1]],
            [end1, split_vertex_rgb, split_node.vertices_rgb[other0], split_node.vertices_rgb[other1]],
        ];
        let new_vertices_lin = [
            [split_node.vertices_lin[split_node.diameter_edge[0]], split_vertex_lin, split_node.vertices_lin[other0], split_node.vertices_lin[other1]],
            [split_node.vertices_lin[split_node.diameter_edge[1]], split_vertex_lin, split_node.vertices_lin[other0], split_node.vertices_lin[other1]],
        ];
        let new_vertices_lab = [
            [split_node.vertices_lab[split_node.diameter_edge[0]], split_vertex_lab, split_node.vertices_lab[other0], split_node.vertices_lab[other1]],
            [split_node.vertices_lab[split_node.diameter_edge[1]], split_vertex_lab, split_node.vertices_lab[other0], split_node.vertices_lab[other1]],
        ];

        for i in 0..2 {
            let mut max_distance_seen = 0.0;
            let mut max_distance_edge = [0, 1];
            for &edge in &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]] {
                let dist2 = distance2(new_vertices_lab[i][edge[0]], new_vertices_lab[i][edge[1]]);
                if dist2 > max_distance_seen {
                    max_distance_seen = dist2;
                    max_distance_edge = edge;
                }
            }

            let node = SimplexCut {
                vertices_rgb: new_vertices_rgb[i],
                vertices_lin: new_vertices_lin[i],
                vertices_lab: new_vertices_lab[i],
                diameter2: max_distance_seen,
                diameter_edge: max_distance_edge,
                points: split_points[i].clone()
            };
            if !node.points.is_empty() {
                for &vertex in &node.vertices_rgb {
                    *referenced_points.entry(vertex).or_insert(0) += 1;
                }
                nodes.push(node);
            }
        }

        for &vertex in &split_node.vertices_rgb {
            match referenced_points.entry(vertex) {
                std::collections::hash_map::Entry::Occupied(mut occ_entry) => {
                    *occ_entry.get_mut() -= 1;
                    if *occ_entry.get() == 0 {
                        occ_entry.remove();
                    }
                },
                _ => unreachable!()
            }
        }
    }

    // Post-process: improve overlarge simplices
    while let Some(mut node) = nodes.pop() {
        node.optimize(&mut referenced_points, distance2);
    }

    for &color in referenced_points.keys() {
        palette.push(color);
    }

    palette
}

#[derive(Copy, Clone)]
pub enum Split {
    Half,
    Median,
    Mean
}

pub fn make_box_palette(palette_size: usize, pixels: impl Iterator<Item=image::Rgb<u8>>, split: Split, optim: bool) -> Vec<image::Rgb<u8>> {
    let first_node = OctreeNode {
        bounding_box: [[0, 255], [0, 255], [0, 255]],
        pixels: pixels.collect()
    };

    let mut refs = std::collections::HashMap::new();
    for corner in first_node.corners() {
        refs.insert(corner, 1);
    }

    struct OctreeNode {
        bounding_box: [[u8; 2]; 3],
        pixels: Vec<image::Rgb<u8>>
    }

    impl std::cmp::PartialEq for OctreeNode {
        fn eq(&self, other: &OctreeNode) -> bool {
            self.pixels.len() == other.pixels.len()
        }
    }
    impl std::cmp::Eq for OctreeNode { }
    impl std::cmp::PartialOrd for OctreeNode {
        fn partial_cmp(&self, other: &OctreeNode) -> Option<std::cmp::Ordering> {
            self.pixels.len().partial_cmp(&other.pixels.len())
        }
    }
    impl std::cmp::Ord for OctreeNode {
        fn cmp(&self, other: &OctreeNode) -> std::cmp::Ordering {
            self.pixels.len().cmp(&other.pixels.len())
        }
    }

    impl OctreeNode {
        fn corners<'a>(&'a self) -> impl Iterator<Item=image::Rgb<u8>> + 'a {
            self.bounding_box[0].iter().flat_map(move |&r|
                self.bounding_box[1].iter().flat_map(move |&g|
                    self.bounding_box[2].iter().map(move |&b|
                        image::Rgb([r, g, b])
                    )
                )
            )
        }

        fn optimize(&mut self, corn_refs: &mut std::collections::HashMap<image::Rgb<u8>, usize>) {
            for axis in 0..3 {
                let other_axes = [[1,2],[0,2],[0,1]][axis];
                'face_loop:
                for &side in &[0,1] {
                    let fixed_value = self.bounding_box[axis][side];

                    for &other_side0 in &[0,1] {
                        for &other_side1 in &[0,1] {
                            let mut data = [0,0,0];
                            data[axis] = fixed_value;
                            data[other_axes[0]] = self.bounding_box[other_axes[0]][other_side0];
                            data[other_axes[1]] = self.bounding_box[other_axes[1]][other_side1];
                            if corn_refs.get(&image::Rgb(data)).cloned() != Some(1) {
                                continue 'face_loop;
                            }
                        }
                    }

                    let furthest = if side == 0 {
                        self.pixels.iter().map(|px| px.channels()[axis]).min().unwrap_or(self.bounding_box[axis][side])
                    } else {
                        self.pixels.iter().map(|px| px.channels()[axis]).max().unwrap_or(self.bounding_box[axis][side])
                    };

                    if furthest != fixed_value {
                        self.bounding_box[axis][side] = furthest;
                        for &other_side0 in &[0,1] {
                            for &other_side1 in &[0,1] {
                                let mut data = [0,0,0];
                                data[axis] = fixed_value;
                                data[other_axes[0]] = self.bounding_box[other_axes[0]][other_side0];
                                data[other_axes[1]] = self.bounding_box[other_axes[1]][other_side1];
                                corn_refs.remove(&image::Rgb(data));
                                data[axis] = furthest;
                                *corn_refs.entry(image::Rgb(data)).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut nodes = std::collections::BinaryHeap::new();
    nodes.push(first_node);

    while refs.len() + 4 <= palette_size {
        let mut split_node = if let Some(node) = nodes.pop() {
            node
        } else {
            break;
        };
        if optim {
            split_node.optimize(&mut refs);
        }

        let mut axis = 0;
        let mut range = split_node.bounding_box[axis][1] - split_node.bounding_box[axis][0];
        if split_node.bounding_box[1][1] - split_node.bounding_box[1][0] > range {
            axis = 1;
            range = split_node.bounding_box[axis][1] - split_node.bounding_box[axis][0];
        }
        if split_node.bounding_box[2][1] - split_node.bounding_box[2][0] > range {
            axis = 2;
            range = split_node.bounding_box[axis][1] - split_node.bounding_box[axis][0];
        }
        if range <= 1 {
            continue;
        };
        let mid = match split {
            Split::Half => split_node.bounding_box[axis][0] + range / 2,
            Split::Median => {
                split_node.pixels.sort_by(|&p1, &p2| p1.channels()[axis].cmp(&p2.channels()[axis]));
                let small_count = split_node.pixels.iter().take_while(|&p| p.channels()[axis] == split_node.bounding_box[axis][0]).count();
                let large_count = split_node.pixels.iter().rev().take_while(|&p| p.channels()[axis] == split_node.bounding_box[axis][1]).count();
                if small_count + large_count == split_node.pixels.len() {
                    split_node.bounding_box[axis][0] + range / 2
                } else {
                    split_node.pixels[small_count + (split_node.pixels.len() - small_count - large_count) / 2].channels()[axis]
                }
            },
            Split::Mean => {
                split_node.pixels.sort_by(|&p1, &p2| p1.channels()[axis].cmp(&p2.channels()[axis]));
                let total = split_node.pixels.iter().map(|&p| color::srgb_decode_channel(p.channels()[axis])).sum::<f64>();
                let mean = color::srgb_encode_channel(total / split_node.pixels.len() as f64);
                if mean <= split_node.bounding_box[axis][0] || mean >= split_node.bounding_box[axis][1] {
                    split_node.bounding_box[axis][0] + range / 2
                } else {
                    mean
                }
            }
        };

        let mut low_bounding_box = split_node.bounding_box;
        let mut high_bounding_box = split_node.bounding_box;
        low_bounding_box[axis][1] = mid;
        high_bounding_box[axis][0] = mid;

        for &bounding_box in &[low_bounding_box, high_bounding_box] {
            let node = OctreeNode {
                bounding_box: bounding_box,
                pixels: split_node.pixels.iter().filter(|&&p| (0..3).all(|channel| {
                    bounding_box[channel][0] <= p.channels()[channel] && p.channels()[channel] <= bounding_box[channel][1]
                })).cloned().collect()
            };

            if node.pixels.iter().any(|&p| p.channels()[axis] != mid) {
                for corner in node.corners() {
                    *refs.entry(corner).or_insert(0) += 1;
                }
                nodes.push(node);
            }
        }

        for corner in split_node.corners() {
            match refs.entry(corner) {
                std::collections::hash_map::Entry::Occupied(mut occ_entry) => {
                    *occ_entry.get_mut() -= 1;
                    if *occ_entry.get() == 0 {
                        occ_entry.remove();
                    }
                },
                _ => unreachable!()
            }
        }
    }

    if optim {
        let mut vec_nodes = nodes.into_vec();
        for node in &mut vec_nodes {
            node.optimize(&mut refs);
        }
        nodes = vec_nodes.into_iter().collect();
    }

    while refs.len() < palette_size {
        if let Some(node) = nodes.pop() {
            refs.insert(image::Rgb([
                node.bounding_box[0][0] + (node.bounding_box[0][1] - node.bounding_box[0][0]) / 2,
                node.bounding_box[1][0] + (node.bounding_box[1][1] - node.bounding_box[1][0]) / 2,
                node.bounding_box[2][0] + (node.bounding_box[2][1] - node.bounding_box[2][0]) / 2,
            ]), 1);
        } else {
            break;
        }
    }

    refs.keys().cloned().collect()
}
