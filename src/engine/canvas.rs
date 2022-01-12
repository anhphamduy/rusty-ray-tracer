use super::color::Color;
use crate::elementary::float::Float;

pub struct Canvas {
    data: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width: width,
            height: height,
            data: vec![
                vec![Color::new(Float::new(0.0), Float::new(0.0), Float::new(0.0)); width];
                height
            ],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        if x > self.width() {
            panic!("x must be within a range");
        } else if y > self.height() {
            panic!("y must be within a range");
        }

        self.data.get(y).unwrap().get(x).unwrap()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) -> Color {
        std::mem::replace(&mut self.data[y][x], c.clone())
    }

    pub fn save(&self) -> String {
        // write the ppm headers
        let mut data = format!(
            "{}\n{} {}\n255\n",
            "P3",
            self.width().to_string(),
            self.height().to_string()
        );
        for y in 0..self.height() {
            let mut line = String::from("");
            for x in 0..self.width() {
                let color_str = (*self.pixel_at(x, y)).clone().to_255();

                if x == 0 {
                    line = format!("{}", color_str);
                } else {
                    line = format!("{} {}", line, color_str);
                }
            }
            data = format!("{}{}\n", data, line);
        }
        let data_copy = data.clone();

        std::fs::write("data.ppm", data).expect("Unable to save canvas");

        data_copy
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::Canvas;
    use super::Color;
    use super::Float;

    #[test]
    fn can_create_a_canvas() {
        let height = 20;
        let width = 10;
        let canvas = Canvas::new(width, height);

        assert_eq!(canvas.height(), height);
        assert_eq!(canvas.width(), width);

        for y in 0..height {
            for x in 0..width {
                assert_eq!(
                    *canvas.pixel_at(x, y),
                    Color::new(Float::new(0.0), Float::new(0.0), Float::new(0.0))
                )
            }
        }
    }

    #[test]
    fn can_write_pixels_to_canvas() {
        let height = 20;
        let width = 10;
        let mut canvas = Canvas::new(width, height);

        let red = Color::new(Float::new(1.0), Float::new(0.0), Float::new(0.0));

        canvas.write_pixel(2, 3, red);
        assert_eq!(*canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn can_save_canvas() {
        let mut canvas = Canvas::new(5, 3);

        let c1 = Color::new(Float::new(1.5), Float::new(0.0), Float::new(0.0));
        let c2 = Color::new(Float::new(0.0), Float::new(0.5), Float::new(0.0));
        let c3 = Color::new(-Float::new(0.5), Float::new(0.0), Float::new(1.0));

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        assert_eq!(
            canvas.save(),
            String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n")
        );
    }
}
