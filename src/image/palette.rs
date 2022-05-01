use image::{
    imageops::colorops::{ColorMap},
    Pixel, Rgb,
};

// palette taken from https://github.com/cnlohr/epaper_projects/blob/master/atmega168pb_waveshare_color/tools/converter/converter.c
pub static EPAPER_PALETTE: [[u8; 3]; 8] = [
    [0, 0, 0],
    [255, 255, 255],
    [67, 138, 28],
    [100, 64, 255],
    [191, 0, 0],
    [255, 243, 56],
    [232, 126, 0],
    [194, 164, 244],
];

fn abs_diff(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[derive(Clone)]
pub struct Palette {
    p: Vec<Rgb<u8>>,
}

impl Palette {
    pub fn new(input: [[u8; 3]; 8]) -> Self {
        Self {
            p: input.iter().map(|color| *Rgb::from_slice(color)).collect(),
        }
    }
}

impl ColorMap for Palette {
    type Color = Rgb<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Rgb<u8>) -> usize {
        let distance_vec: Vec<usize> = self.p
            .iter()
            .map(|c| {
                abs_diff(color[0], c[0]) as usize
                    + abs_diff(color[1], c[1]) as usize
                    + abs_diff(color[2], c[2]) as usize
            })
            .collect();
        return distance_vec
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;
    }

    #[inline(always)]
    fn lookup(&self, index: usize) -> Option<Rgb<u8>> {
        return self.p.get(index).cloned();
    }

    fn has_lookup(&self) -> bool {
        true
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Rgb<u8>) {
        if let Some(new_color) = self.lookup(self.index_of(color)) {
            color.0 = new_color.0;
        }
    }
}
