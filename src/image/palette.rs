use image::{imageops::colorops::ColorMap, Pixel, Rgb};

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

pub static WAVESHARE_PALETTE: [[u8; 3]; 7] = [
    [0, 0, 0],
    [255, 255, 255],
    [0, 255, 0],
    [0, 0, 255],
    [255, 0, 0],
    [255, 255, 0],
    [200, 180, 160],
];

// rgb(75,75,80)
// rgb(200,200,150)
// rgb(97,115,85)
// rgb(95,85,110)
// rgb(175,75,59)
// rgb(200,160,60)
// rgb(200,180,160)

// New colours
// rgb(50,50,80)
// rgb(225,225,225)
// rgb(87,130,75)
// rgb(65,65,150)
// rgb(175,45,39)
// rgb(220,200,110)
// rgb(225,225,195)

pub static WAVESHARE_PALETTE_REAL: [[u8; 3]; 7] = [
    [50, 50, 80],    // black
    [225, 225, 225], // white
    [87, 130, 75],   // green
    [65, 65, 150],   // blue
    [175, 45, 39],   // red
    [220, 200, 110], // yellow
    [225, 225, 195], // beige
];

pub static STRICT_PALETTE: [[u8; 3]; 8] = [
    [0, 0, 0],
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    [255, 255, 0],
    [0, 255, 255],
    [255, 0, 255],
    [255, 255, 255],
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
    index_p: Option<Vec<Rgb<u8>>>,
}

impl Palette {
    pub fn new(input: Vec<[u8; 3]>, index_p: Option<Vec<[u8; 3]>>) -> Self {
        Self {
            p: input.iter().map(|color| *Rgb::from_slice(color)).collect(),
            index_p: index_p.map(|palette| {
                palette
                    .iter()
                    .map(|color| *Rgb::from_slice(color))
                    .collect()
            }),
        }
    }
}

impl ColorMap for Palette {
    type Color = Rgb<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Rgb<u8>) -> usize {
        let distance_vec: Vec<usize> = self
            .p
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
        if let Some(palette) = &self.index_p {
            return palette.get(index).cloned();
        }
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
