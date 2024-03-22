use std::fs::{File,remove_file};
use std::io::{Read,Write};
use std::path::Path;

fn main() {
    let width = 100;
    let height = 100;
    let mut pixels: Vec<[u8;3]> = create_pixels(width, height);
    draw_line(width, height, &mut pixels, (0,0), (99, 99));
    pixels_to_ppm(height,width, pixels);
}

fn pixels_to_ppm(width: usize, height: usize, pixels: Vec<[u8;3]>){
    let output_path:&str = "Output.ppm";
    let mut file:File;
    if Path::new(output_path).exists() {
       match remove_file(output_path){
        Ok(_) => (),
        Err(e)=> panic!("{}",e),
       };
    }
    
    file = match File::options().read(true).write(true).create_new(true).open(output_path){
        Ok(file) => file,
        Err(_) =>  panic!("Could not create file.")
    };

    let header = format!("P6 {} {} 255\n", width, height);
    file.write(header.as_bytes());
    for pixel in &pixels {
        file.write(pixel);
    }
}

fn create_pixels(width: usize, height: usize) -> Vec<[u8; 3]> {
    let white:[u8;3] = [0xFF;3];
    let pixels = vec![white;width*height];
    return pixels;
}

fn draw_line(width: usize, _height: usize, pixels: &mut Vec<[u8;3]>, start: (usize, usize), end: (usize, usize)){

    let pdx:i8;
    let ddx:i8;
    let pdy:i8;
    let ddy:i8;

    let deltasd:isize;
    let deltafd:isize;
    
    let mut dx:isize = (end.0 as isize) - (start.0 as isize);
    let mut dy:isize = (end.1 as isize) - (start.1 as isize);
    let incx:i8 = sign(dx);
    let incy:i8 = sign(dy);
    dx = dx.abs(); 
    dy = dy.abs();

    if dx > dy {
        pdx = incx;
        pdy = 0;

        ddx = incx;
        ddy = incy;

        deltasd = dy;
        deltafd = dx;
    }

    else{
        pdx = 0;
        pdy = incy;

        ddx = incx;
        ddy = incy;

        deltasd = dx;
        deltafd = dy;
    }

    let mut x:isize = start.0 as isize;
    let mut y:isize = start.1 as isize;

    let mut err:isize = deltafd/2;

    let start_pos = cord_transform(width, start);
    pixels[start_pos] = [0x00;3];

    let mut pos:usize;

    for _i in 0..deltafd{
        err -= deltasd;
        if err < 0 {
            err += deltafd;
            x+=ddx as isize;
            y+=ddy as isize;
        }
        else{
             x += pdx as isize;
             y += pdy as isize;
        }
        pos = cord_transform(width, (x as usize,y as usize));
    }
}

fn cord_transform(width: usize, cord: (usize, usize)) -> usize{
    return cord.0+cord.1*width;
}

fn sign(x: isize) -> i8 {
    if x > 0 {
        return 1
    }
    if x < 0 {
        return -1
    }
    return 0
}

