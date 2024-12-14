use image::{GenericImage, ImageBuffer, RgbImage};

// const INPUT: &'static str = include_str!("../test");
const INPUT: &'static str = include_str!("../input");

fn main() {
    let mut robots = Vec::new();
    for line in INPUT.lines() {
        let (p, v) = line.split_once(' ').unwrap();
        let (_, p) = p.split_once('=').unwrap();
        let (_, v) = v.split_once('=').unwrap();
        let pos = p
            .split_once(',')
            .map(|p| (p.0.parse::<usize>().unwrap(), p.1.parse::<usize>().unwrap()))
            .unwrap();
        let velocity = v
            .split_once(',')
            .map(|v| (v.0.parse::<i64>().unwrap(), v.1.parse::<i64>().unwrap()))
            .unwrap();
        robots.push(Robot { pos, velocity });
    }
    let time = 100;
    let width = 101;
    let height = 103;
    let quadrant_a = (0..width / 2, 0..height / 2);
    let quadrant_b = ((width / 2 + 1)..width, 0..height / 2);
    let quadrant_c = (0..width / 2, height / 2 + 1..height);
    let quadrant_d = (width / 2 + 1..width, height / 2 + 1..height);
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let new_pos: Vec<(usize, usize)> = robots
        .iter()
        .map(|r| r.calc_pos(time, width, height))
        .collect();
    for (x, y) in new_pos.iter() {
        if quadrant_a.0.contains(&x) && quadrant_a.1.contains(&y) {
            a += 1
        } else if quadrant_b.0.contains(&x) && quadrant_b.1.contains(&y) {
            b += 1
        } else if quadrant_c.0.contains(&x) && quadrant_c.1.contains(&y) {
            c += 1
        } else if quadrant_d.0.contains(&x) && quadrant_d.1.contains(&y) {
            d += 1
        }
    }
    // generate a lot of images loookie lookie find out its 8179 :)
    let new_pos: Vec<(usize, usize)> = robots
        .iter()
        .map(|r| r.calc_pos(8179, width, height))
        .collect();
    visualize(8179, &new_pos, width, height);
}

pub fn visualize(i: usize, pos: &[(usize, usize)], width: usize, height: usize) {
    let red = 255 as u8;
    let green = 0;
    let blue = 0;

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    for (x, y) in pos.iter().copied() {
        *img.get_pixel_mut(x as u32, y as u32) = image::Rgb([red, green, blue]);
    }
    img.save(format!("{i}.png")).unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (usize, usize),
    velocity: (i64, i64),
}

impl Robot {
    fn calc_pos(&self, time_elapsed: u32, width: usize, height: usize) -> (usize, usize) {
        let new_x =
            (self.pos.0 as i64 + time_elapsed as i64 * self.velocity.0).rem_euclid(width as i64);
        let new_y =
            (self.pos.1 as i64 + time_elapsed as i64 * self.velocity.1).rem_euclid(height as i64);
        (new_x as usize, new_y as usize)
    }
}
