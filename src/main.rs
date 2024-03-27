use std::fs::{File,remove_file};
use std::io::Write;
use std::path::Path;
use std::io::BufWriter;

fn main() {
    //IMPORTANT ITS pixels[y][x]
    let width = 1000;
    let height = 1000;
    let mut pixels: Vec<Vec<[u8;3]>> = create_pixels(width, height);
    draw_quadrangle_filled(&mut pixels, (100, 100), (1000, 200), (500, 1000), (900, 700), [0,255,255]);
    pixels_to_ppm(pixels);
}

fn pixels_to_ppm(pixels: Vec<Vec<[u8;3]>>){
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

    let header = format!("P6 {} {} 255\n", pixels.len(), pixels[0].len());
    match file.write(header.as_bytes()){
        Ok(_) => (),
        Err(_) => panic!("Could not write to the file."),
    };

    let mut reader = BufWriter::new(file);
    
    for pixel in &pixels {
        for pix in pixel {
            match reader.write(pix){
                Ok(_) => (),
                Err(_) => panic!("Could not write to the file."),
            };
        }
    }
    
}

fn create_pixels(width: usize, height: usize) -> Vec<Vec<[u8; 3]>> {
    let white:[u8;3] = [0xFF;3];
    let pixels = vec![vec![white;height];width];
    return pixels;
}

fn draw_line(pixels: &mut Vec<Vec<[u8;3]>>, start: (usize, usize), end: (usize, usize)){

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

    pixels[start.1][start.0] = [0x00;3];

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
        pixels[y as usize][x as usize] = [0x00;3];
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

fn draw_triangle_outline(pixels: &mut Vec<Vec<[u8;3]>>, p1 : (usize, usize), p2 : (usize, usize), p3 : (usize, usize)){
    draw_line(pixels, p1, p2);
    draw_line(pixels, p2, p3);
    draw_line(pixels, p1, p3);
}

fn rasterize(pixels: &mut Vec<Vec<[u8;3]>>, p1 : (isize, isize), p2 : (isize, isize), p3 : (isize, isize), color:[u8;3]){
    let dx12: isize = p1.0-p2.0;
    let dx23: isize = p2.0-p3.0;
    let dx31: isize = p3.0-p1.0 ;

    let dy12: isize = p1.1-p2.1;
    let dy23: isize = p2.1-p3.1;
    let dy31: isize = p3.1-p1.1;

    let min_x = min(p1.0,p2.0,p3.0);
    let min_y = min(p1.1,p2.1,p3.1);
    
    let max_x = max(p1.0,p2.0,p3.0);
    let max_y = max(p1.1,p2.1,p3.1);

    let c1 = dy12 * p1.0 - dx12 * p1.1;
    let c2 = dy23 * p2.0 - dx23 * p2.1;
    let c3 = dy31 * p3.0 - dx31 * p3.1;

    let mut cy1 = c1 + dx12 * min_y - dy12 * min_x;
    let mut cy2 = c2 + dx23 * min_y - dy23 * min_x;
    let mut cy3 = c3 + dx31 * min_y - dy31 * min_x;

    let _block_size: isize = 100;

    for y in min_y..max_y  {
        
        let mut cx1 = cy1;
        let mut cx2 = cy2;
        let mut cx3 = cy3;


        for x in min_x..max_x {
                if cx1 > 0 && cx2 > 0 && cx3 > 0 {
                        pixels[y as usize][x as usize] = color;
                }

            cx1 -= dy12;
            cx2  -= dy23;
            cx3 -= dy31;
        }
        cy1 += dx12;
        cy2 += dx23;
        cy3 += dx31;
    }
}

fn min(a: isize, b: isize, c: isize) -> isize {
    if a < b {
        if a < c {
            return a;
        } else {
            return c;
        }
    } else {
        if b < c {
            return b;
        }
        else{
            return c;
        }
    }
}

fn max(a: isize, b: isize, c: isize) -> isize {
    if a > b {
        if a > c {
            return a;
        } else {
            return c;
        }
    } else {
        if b > c {
            return b;
        }
        else{
            return c;
        }
    }
} 

fn quadrangle_outline(pixels: &mut Vec<Vec<[u8;3]>>, p1: (usize, usize), p2: (usize, usize), p3: (usize, usize), p4: (usize, usize)){
    draw_line(pixels, p1, p2);
    draw_line(pixels, p2, p3);
    draw_line(pixels, p3, p4);
    draw_line(pixels, p4, p1);
}

fn draw_filled_triangle(pixels: &mut Vec<Vec<[u8;3]>>, p1 : (isize, isize), p2 : (isize, isize), p3 : (isize, isize), color:[u8;3]){
    let mut p1 = p1;
    let mut p2 = p2;
    let mut p3 = p3;
    if p1.0 > p2.0{
        println!("Switch");
        let temp = p1;
        p1 = p2;
        p2 = temp;
    }
    if p2.1 > p3.1{
        println!("Switch");
        let temp = p2;
        p2 = p3;
        p3 = temp;
    }
    if p1.0 > p2.0{
        println!("Switch");
        let temp = p1;
        p1 = p2;
        p2 = temp;
    }
    if p2.1 > p3.1{
        println!("Switch");
        let temp = p2;
        p2 = p3;
        p3 = temp;
    }
    if p1.0 > p2.0{
        println!("Switch");
        let temp = p1;
        p1 = p2;
        p2 = temp;
    }
    if p2.1 > p3.1{
        println!("Switch");
        let temp = p2;
        p2 = p3;
        p3 = temp;
    }
    println!("{:?} {:?} {:?}", p1, p2, p3);
    rasterize(pixels, p1, p2, p3, color);
}


fn draw_square_outline(pixels: &mut Vec<Vec<[u8;3]>>, upper_left: (usize, usize), size: usize) {
    let upper_right = (upper_left.0 + size, upper_left.1);
    let lower_left = (upper_left.0, upper_left.1 + size);
    let lower_right = (upper_left.0 + size, upper_left.1 + size);
    draw_line(pixels,  upper_left, upper_right);
    draw_line(pixels,  upper_right, lower_right);
    draw_line(pixels,  lower_right, lower_left);
    draw_line(pixels, lower_left, upper_left);
}

fn draw_square_filled(pixels: &mut Vec<Vec<[u8;3]>>, upper_left: (usize, usize), size: usize, color: [u8;3]){
    for y in upper_left.0..upper_left.0+size {
        for x in upper_left.1..upper_left.1+size {
            pixels[y as usize][x as usize] = color;
        }
    }
}

fn draw_quadrangle_outline(pixels: &mut Vec<Vec<[u8;3]>>, upper_left: (usize, usize), upper_right: (usize, usize) , lower_left: (usize, usize), lower_right: (usize, usize) ){
    draw_line(pixels,  upper_left, upper_right);
    draw_line(pixels,  upper_right, lower_right);
    draw_line(pixels,  lower_right, lower_left);
    draw_line(pixels, lower_left, upper_left);
}

fn draw_quadrangle_filled(pixels: &mut Vec<Vec<[u8;3]>>, upper_left: (isize, isize), upper_right: (isize, isize) , lower_left: (isize, isize), lower_right: (isize, isize), color: [u8;3] ){
    rasterize(pixels, upper_left, lower_left, upper_right, color);
    rasterize(pixels, lower_right, upper_right, lower_left, color);
}