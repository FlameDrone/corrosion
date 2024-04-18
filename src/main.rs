use std::fs::{remove_file, File};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

struct Triangle {
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
}

trait Object {
    fn draw(self: &Self, pixels: &mut Vec<Vec<[u8; 3]>>, color: [u8; 3]) {}
    fn outline(self: &Self, pixels: &mut Vec<Vec<[u8; 3]>>) {}
    fn displace(self: &mut Self, vector: (isize, isize)) {}
    fn rotate(self: &mut Self, angle: f32) {}
}

impl Object for Triangle {
    fn draw(self: &Self, pixels: &mut Vec<Vec<[u8; 3]>>, color: [u8; 3]) {
        let mut p1 = self.p1;
        let mut p2 = self.p2;
        let mut p3 = self.p3;
        if p1.0 > p2.0 {
            let temp = p1;
            p1 = p2;
            p2 = temp;
        }
        if p2.1 < p3.1 {
            let temp = p2;
            p2 = p3;
            p3 = temp;
        }
        if p1.0 > p2.0 {
            let temp = p1;
            p1 = p2;
            p2 = temp;
        }
        if p2.1 < p3.1 {
            let temp = p2;
            p2 = p3;
            p3 = temp;
        }
        if p1.0 > p2.0 {
            let temp = p1;
            p1 = p2;
            p2 = temp;
        }
        if p2.1 < p3.1 {
            let temp = p2;
            p2 = p3;
            p3 = temp;
        }
        rasterize(pixels, p1, p2, p3, color);
    }
    fn outline(self: &Self, pixels: &mut Vec<Vec<[u8; 3]>>) {
        draw_line(pixels, self.p1, self.p2);
        draw_line(pixels, self.p2, self.p3);
        draw_line(pixels, self.p1, self.p3);
    }
    fn displace(self: &mut Self, vector: (isize, isize)) {
        self.p1 = add_vector(self.p1, vector);
        self.p2 = add_vector(self.p2, vector);
        self.p3 = add_vector(self.p3, vector);
    }
    fn rotate(self: &mut Self, angle: f32){
        let mid = ((self.p1.0+self.p2.0+self.p3.0)/3,(self.p1.1+self.p2.1+self.p3.1)/3);
        let mut dif1 = ((self.p1.0 as isize)-(mid.0 as isize),(self.p1.1 as isize)-(mid.1 as isize));
        let mut dif2 = ((self.p2.0 as isize)-(mid.0 as isize),(self.p2.1 as isize)-(mid.1 as isize));
        let mut dif3 = ((self.p3.0 as isize)-(mid.0 as isize),(self.p3.1 as isize)-(mid.1 as isize));
        dif1 = ((((dif1.0 as f32) * angle.cos()) as isize)-(((dif1.1 as f32) * angle.sin()) as isize), (((dif1.0 as f32)*angle.sin()) as isize)+(((dif1.1 as f32)*angle.cos()) as isize));
        dif2 = ((((dif2.0 as f32) * angle.cos()) as isize)-(((dif2.1 as f32) * angle.sin()) as isize), (((dif2.0 as f32)*angle.sin()) as isize)+(((dif2.1 as f32)*angle.cos()) as isize));
        dif3 = ((((dif3.0 as f32) * angle.cos()) as isize)-(((dif3.1 as f32) * angle.sin()) as isize), (((dif3.0 as f32)*angle.sin()) as isize)+(((dif3.1 as f32)*angle.cos()) as isize));
        self.p1 = add_vector(mid, dif1);
        self.p2 = add_vector(mid, dif2);
        self.p3 = add_vector(mid, dif3);
    }
}
fn main() {
    //IMPORTANT ITS pixels[y][x]
    let width = 1000;
    let height = 1000;
    let mut pixels: Vec<Vec<[u8; 3]>> = create_pixels(width, height);
    let mut tri = Triangle {
        p1: (200, 100),
        p2: (500, 300),
        p3: (100, 600),
    };
    tri.draw(&mut pixels, [100, 175, 235]);
    tri.rotate(90.0);
    tri.outline(&mut pixels);
    pixels_to_ppm(pixels);
}

fn pixels_to_ppm(pixels: Vec<Vec<[u8; 3]>>) {
    let output_path: &str = "Output.ppm";
    let mut file: File;
    if Path::new(output_path).exists() {
        match remove_file(output_path) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }

    file = match File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(output_path)
    {
        Ok(file) => file,
        Err(_) => panic!("Could not create file."),
    };

    let header = format!("P6 {} {} 255\n", pixels.len(), pixels[0].len());
    match file.write(header.as_bytes()) {
        Ok(_) => (),
        Err(_) => panic!("Could not write to the file."),
    };

    let mut reader = BufWriter::new(file);

    for pixel in &pixels {
        for pix in pixel {
            match reader.write(pix) {
                Ok(_) => (),
                Err(_) => panic!("Could not write to the file."),
            };
        }
    }
}

fn create_pixels(width: usize, height: usize) -> Vec<Vec<[u8; 3]>> {
    let white: [u8; 3] = [0xFF; 3];
    let pixels = vec![vec![white; height]; width];
    return pixels;
}

fn draw_line(pixels: &mut Vec<Vec<[u8; 3]>>, start: (usize, usize), end: (usize, usize)) {
    let pdx: i8;
    let ddx: i8;
    let pdy: i8;
    let ddy: i8;

    let deltasd: isize;
    let deltafd: isize;

    let mut dx: isize = (end.0 as isize) - (start.0 as isize);
    let mut dy: isize = (end.1 as isize) - (start.1 as isize);
    let incx: i8 = sign(dx);
    let incy: i8 = sign(dy);
    dx = dx.abs();
    dy = dy.abs();

    if dx > dy {
        pdx = incx;
        pdy = 0;

        ddx = incx;
        ddy = incy;

        deltasd = dy;
        deltafd = dx;
    } else {
        pdx = 0;
        pdy = incy;

        ddx = incx;
        ddy = incy;

        deltasd = dx;
        deltafd = dy;
    }

    let mut x: isize = start.0 as isize;
    let mut y: isize = start.1 as isize;

    let mut err: isize = deltafd / 2;

    pixels[start.1][start.0] = [0x00; 3];

    for _i in 0..deltafd {
        err -= deltasd;
        if err < 0 {
            err += deltafd;
            x += ddx as isize;
            y += ddy as isize;
        } else {
            x += pdx as isize;
            y += pdy as isize;
        }
        pixels[y as usize][x as usize] = [0x00; 3];
    }
}

fn sign(x: isize) -> i8 {
    if x > 0 {
        return 1;
    }
    if x < 0 {
        return -1;
    }
    return 0;
}

fn draw_triangle_outline(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
) {
    draw_line(pixels, p1, p2);
    draw_line(pixels, p2, p3);
    draw_line(pixels, p1, p3);
}

fn rasterize(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
    color: [u8; 3],
) {
    let dx12: isize = (p1.0 as isize) - (p2.0 as isize);
    let dx23: isize = (p2.0 as isize) - (p3.0 as isize);
    let dx31: isize = (p3.0 as isize) - (p1.0 as isize);

    let dy12: isize = (p1.1 as isize) - (p2.1 as isize);
    let dy23: isize = (p2.1 as isize) - (p3.1 as isize);
    let dy31: isize = (p3.1 as isize) - (p1.1 as isize);

    let min_x = min(p1.0, p2.0, p3.0);
    let min_y = min(p1.1, p2.1, p3.1);

    let max_x = max(p1.0, p2.0, p3.0);
    let max_y = max(p1.1, p2.1, p3.1);

    let c1 = dy12 * (p1.0 as isize) - dx12 * (p1.1 as isize);
    let c2 = dy23 * (p2.0 as isize) - dx23 * (p2.1 as isize);
    let c3 = dy31 * (p3.0 as isize) - dx31 * (p3.1 as isize);

    let mut cy1 = c1 + dx12 * (min_y as isize) - dy12 * (min_x as isize);
    let mut cy2 = c2 + dx23 * (min_y as isize) - dy23 * (min_x as isize);
    let mut cy3 = c3 + dx31 * (min_y as isize) - dy31 * (min_x as isize);

    for y in min_y..max_y {
        let mut cx1 = cy1;
        let mut cx2 = cy2;
        let mut cx3 = cy3;

        for x in min_x..max_x {
            if cx1 > 0 && cx2 > 0 && cx3 > 0 {
                pixels[y][x] = color;
            }

            cx1 -= dy12;
            cx2 -= dy23;
            cx3 -= dy31;
        }
        cy1 += dx12;
        cy2 += dx23;
        cy3 += dx31;
    }
}

fn min(a: usize, b: usize, c: usize) -> usize {
    if a < b {
        if a < c {
            return a;
        } else {
            return c;
        }
    } else {
        if b < c {
            return b;
        } else {
            return c;
        }
    }
}

fn max(a: usize, b: usize, c: usize) -> usize {
    if a > b {
        if a > c {
            return a;
        } else {
            return c;
        }
    } else {
        if b > c {
            return b;
        } else {
            return c;
        }
    }
}

fn quadrangle_outline(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
    p4: (usize, usize),
) {
    draw_line(pixels, p1, p2);
    draw_line(pixels, p2, p3);
    draw_line(pixels, p3, p4);
    draw_line(pixels, p4, p1);
}

fn draw_filled_triangle(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
    color: [u8; 3],
) {
    let mut p1 = p1;
    let mut p2 = p2;
    let mut p3 = p3;
    if p1.0 > p2.0 {
        let temp = p1;
        p1 = p2;
        p2 = temp;
    }
    if p2.1 < p3.1 {
        let temp = p2;
        p2 = p3;
        p3 = temp;
    }
    if p1.0 > p2.0 {
        let temp = p1;
        p1 = p2;
        p2 = temp;
    }
    if p2.1 < p3.1 {
        let temp = p2;
        p2 = p3;
        p3 = temp;
    }
    if p1.0 > p2.0 {
        let temp = p1;
        p1 = p2;
        p2 = temp;
    }
    if p2.1 < p3.1 {
        let temp = p2;
        p2 = p3;
        p3 = temp;
    }
    rasterize(pixels, p1, p2, p3, color);
}

fn draw_square_outline(pixels: &mut Vec<Vec<[u8; 3]>>, upper_left: (usize, usize), size: usize) {
    let upper_right = (upper_left.0 + size, upper_left.1);
    let lower_left = (upper_left.0, upper_left.1 + size);
    let lower_right = (upper_left.0 + size, upper_left.1 + size);
    draw_line(pixels, upper_left, upper_right);
    draw_line(pixels, upper_right, lower_right);
    draw_line(pixels, lower_right, lower_left);
    draw_line(pixels, lower_left, upper_left);
}

fn draw_square_filled(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    upper_left: (usize, usize),
    size: usize,
    color: [u8; 3],
) {
    for column in pixels.iter_mut().skip(upper_left.0).take(size) {
        for pixel in column.iter_mut().skip(upper_left.1).take(size) {
            *pixel = color;
        }
    }
}

fn draw_quadrangle_outline(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    upper_left: (usize, usize),
    upper_right: (usize, usize),
    lower_left: (usize, usize),
    lower_right: (usize, usize),
) {
    draw_line(pixels, upper_left, upper_right);
    draw_line(pixels, upper_right, lower_right);
    draw_line(pixels, lower_right, lower_left);
    draw_line(pixels, lower_left, upper_left);
}

fn draw_quadrangle_filled(
    pixels: &mut Vec<Vec<[u8; 3]>>,
    upper_left: (usize, usize),
    upper_right: (usize, usize),
    lower_left: (usize, usize),
    lower_right: (usize, usize),
    color: [u8; 3],
) {
    draw_filled_triangle(pixels, upper_left, lower_left, upper_right, color);
    draw_filled_triangle(pixels, lower_right, upper_right, lower_left, color);
}

fn add_vector(p: (usize, usize), vector: (isize, isize)) -> (usize, usize){
    let p0: isize = (p.0 as isize)+(vector.0);
    let p1: isize = (p.1 as isize)+(vector.1);
    if(p0 < 0 || p1 < 0) {
        panic!("Displaced out of bounds!");
    }
    (p0 as usize, p1 as usize)
}