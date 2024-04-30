fn main()
{
    //scale
    let n = 4000;

    let max_counter: i32 = 1000;

    let re_start=-2.0;
    let re_end=2.0;
    let im_start=-2.0;
    let im_end=2.0;

    //resolution
    let x: u32 = n;
    let fx:f64 = n.into();
    let y: u32 = n;
    let fy:f64 = n.into();
    let mut a = Image::new((x,y));

    //numbers
    let mut c = Complex::new();
    let mut z = Complex::new();
    let mut counter: i32;
    let mut fi: f64;
    let mut fj: f64;
    for i in 0..x
    {
        for j in 0..y
        {
            fi=i.into();
            fj=j.into();
            c.real=re_start + fi/fx * (re_end - re_start);
            c.imaginary=im_start + fj/fy * (im_end - im_start);
            z.real=0.0;
            z.imaginary=0.0;
            counter=0;
            while z.module() <= 2.0 && counter < max_counter
            {
                z = z*z + c;
                counter+=1;
            }
            if counter<max_counter
            {
                a.set_pixel((i,j), (counter as u8,0,0));
            }
            else
            {
                a.set_pixel((i,j), (0,0,(z.module()*128.0) as u8));
            }
        }
    }
    a.save_bmp("fraktal.bmp");

}

pub struct Image
{
    pub size: (u32,u32),
    red: Vec<Vec<u8>>,
    green: Vec<Vec<u8>>,
    blue: Vec<Vec<u8>>
}

impl Image
{
    pub fn new(size: (u32,u32)) -> Self
    {
        Self
        {
            size,
            red: vec![vec![0;size.1 as usize];size.0 as usize],
            green: vec![vec![0;size.1 as usize];size.0 as usize],
            blue: vec![vec![0;size.1 as usize];size.0 as usize]
        }
    }
    pub fn set_pixel(&mut self, pixel: (u32,u32), color: (u8,u8,u8))
    {
        let x: usize = pixel.0 as usize;
        let y: usize = pixel.1 as usize;
        self.red[x][y] = color.0;
        self.green[x][y] = color.1;
        self.blue[x][y] = color.2;
    }
    pub fn get_pixel(&self, pixel: (u32,u32)) -> (u8,u8,u8)
    {
        let x: usize = pixel.0 as usize;
        let y: usize = pixel.1 as usize;
        (self.red[x][y],self.green[x][y],self.blue[x][y])
    }
    pub fn to_ppm(&self) -> String
    {
        let header = "P3\n".to_string() + &self.size.0.to_string() + " " + &self.size.1.to_string() + "\n255\n";
        let mut body = "".to_string();
        for j in 0..self.size.1 as usize
        {
            for i in 0..self.size.0 as usize
            {
                body = body + &self.red[i][j].to_string() + " ";
                body = body + &self.green[i][j].to_string() + " ";
                body = body + &self.blue[i][j].to_string() + " ";
            }
        }
        header+&body
    }
    pub fn save_ppm(&self, filename: &str)
    {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new(filename);
        let display = path.display();
        let mut file = match File::create(&path)
        {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(self.to_ppm().as_bytes())
        {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
    pub fn save_bmp(&self, filename: &str)
    {
        let mut padding = self.size.0*3;
        while padding%4!=0
        {
            padding +=1;
        }
        padding -= 3*self.size.0;

        let filesize=self.size.1*(3*self.size.0+padding);
        let mut header = "BM".as_bytes().to_vec(); //tag
        header.append(&mut (filesize+54).to_le_bytes().to_vec()); //filesize
        header.append(&mut 0_u32.to_le_bytes().to_vec()); //unused
        header.append(&mut 54_u32.to_le_bytes().to_vec()); //offset
        let mut dibheader = 40_u32.to_le_bytes().to_vec(); //size of dibheader
        dibheader.append(&mut (self.size.0).to_le_bytes().to_vec()); //width
        dibheader.append(&mut (self.size.1).to_le_bytes().to_vec()); //height
        dibheader.append(&mut 1_u16.to_le_bytes().to_vec()); //color planes
        dibheader.append(&mut 24_u16.to_le_bytes().to_vec()); //bits per pixel
        dibheader.append(&mut 0_u32.to_le_bytes().to_vec()); //compression
        dibheader.append(&mut 0_u32.to_le_bytes().to_vec()); //image size, ignored if no compression
        dibheader.append(&mut 0_u32.to_le_bytes().to_vec()); //vertical resolution, usually ignored
        dibheader.append(&mut 0_u32.to_le_bytes().to_vec()); //horizontal resolution, usually ignored
        dibheader.append(&mut 0_u32.to_le_bytes().to_vec()); //color palette, 0 is default
        dibheader.append(&mut 0_u32.to_le_bytes().to_vec()); //important colors, 0 is all
        let mut body = Vec::new();

        for j in 0..self.size.1 as usize
        {
            for i in 0..self.size.0 as usize
            {
                body.push(self.blue[i][self.size.1 as usize-j-1] as u8);
                body.push(self.green[i][self.size.1 as usize-j-1] as u8);
                body.push(self.red[i][self.size.1 as usize-j-1] as u8);
            }
            for _p in 0..padding
            {
                body.push(0_u8);
            }
        }

        let mut output=header;
        output.append(&mut dibheader);
        output.append(&mut body);

        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new(filename);
        let display = path.display();
        let mut file = match File::create(&path)
        {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(&output[..])
        {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

}

#[derive(Copy,Clone)]
pub struct Complex
{
    pub real: f64,
    pub imaginary: f64
}

impl Complex
{
    pub fn new() -> Self
    {
        Self
        {
            real: 0.0,
            imaginary: 0.0
        }
    }
    pub fn from(real: f64) -> Self
    {
        Self
        {
            real,
            imaginary: 0.0
        }
    }
    pub fn module(&self) -> f64
    {
        (self.real*self.real+self.imaginary*self.imaginary).sqrt()
    }
}

use std::ops;

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        let mut temp = Complex::new();
        temp.real=self.real+rhs.real;
        temp.imaginary=self.imaginary+rhs.imaginary;
        temp
    }
}
impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        let mut temp = Complex::new();
        temp.real=self.real-rhs.real;
        temp.imaginary=self.imaginary-rhs.imaginary;
        temp
    }
}
impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        let mut temp = Complex::new();
        let a=self.real;
        let b=self.imaginary;
        let c=rhs.real;
        let d=rhs.imaginary;
        temp.real=a*c-b*d;
        temp.imaginary=b*c+a*d;
        temp
    }
}
impl ops::Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
        let mut temp = Complex::new();
        let a=self.real;
        let b=self.imaginary;
        let c=rhs.real;
        let d=rhs.imaginary;
        temp.real=(a*c+b*d)/(c*c+d*d);
        temp.imaginary=(b*c-a*d)/(c*c+d*d);
        temp
    }
}
