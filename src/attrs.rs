use nannou::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    location: Point2,
    stroke_weight: f32,
    color: Rgb8,
    alpha: u8,
    wh: (f32, f32),
    rotation: f32,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            location: Point2::new(0.0, 0.0),
            stroke_weight: 1.0,
            color: (0, 0, 0).into(),
            alpha: 255,
            wh: (0.0, 0.0),
            rotation: 0.0,
        }
    }
}

impl SettableAttributes for Attributes {
    fn get_attr_mut(&mut self) -> &mut Attributes {
        self
    }

    fn get_attr(&self) -> &Attributes {
        self
    }
}

pub trait SettableAttributes {
    fn get_attr_mut(&mut self) -> &mut Attributes;

    fn get_attr(&self) -> &Attributes;

    fn set_location(&mut self, point: Point2) -> &mut Self {
        self.get_attr_mut().location = point;
        self
    }

    fn set_stroke_weight(&mut self, sw: f32) -> &mut Self {
        self.get_attr_mut().stroke_weight = sw;
        self
    }

    fn set_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.get_attr_mut().color = (r, g, b).into();
        self.get_attr_mut().alpha = a;
        self
    }

    fn set_rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.get_attr_mut().color = (r, g, b).into();
        self
    }

    fn set_color(&mut self, color: Rgb8) -> &mut Self {
        self.get_attr_mut().color = color;
        self
    }

    fn set_width(&mut self, width: f32) -> &mut Self {
        self.get_attr_mut().wh.0 = width;
        self
    }

    fn set_height(&mut self, height: f32) -> &mut Self {
        self.get_attr_mut().wh.1 = height;
        self
    }

    fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        self.get_attr_mut().alpha = alpha;
        self
    }

    fn set_rotation(&mut self, angle: f32) -> &mut Self {
        self.get_attr_mut().rotation = angle;
        self
    }

    fn location(&self) -> Point2 {
        self.get_attr().location
    }
    fn stroke_weight(&self) -> f32 {
        self.get_attr().stroke_weight
    }
    fn rgba8(&self) -> (u8, u8, u8, u8) {
        let color = self.get_attr().color;
        (color.red, color.green, color.blue, self.get_attr().alpha)
    }
    fn rgba_f32(&self) -> (f32, f32, f32, f32) {
        let c = self.get_attr().color;
        (
            c.red as f32 / 255.0,
            c.green as f32 / 255.0,
            c.blue as f32 / 255.0,
            self.get_attr().alpha as f32 / 255.0,
        )
    }

    fn color(&self) -> Rgb8 {
        self.get_attr().color
    }

    fn width(&self) -> f32 {
        self.get_attr().wh.0
    }
    fn height(&self) -> f32 {
        self.get_attr().wh.1
    }
    fn alpha(&self) -> u8 {
        self.get_attr().rgba8().3
    }
    fn rotation(&self) -> f32 {
        self.get_attr().rotation
    }
}

#[cfg(test)]
mod test {
    use super::*;
    

    struct Object {
        attrs: Attributes,
    }

    impl SettableAttributes for Object {
        fn get_attr_mut(&mut self) -> &mut Attributes {
            &mut self.attrs
        }
        fn get_attr(&self) -> &Attributes {
            &self.attrs
        }
    }

    #[test]
    fn set_location_test() {
        let mut object = Object {
            attrs: Attributes::default(),
        };

        let new_loc = Point2::new(123.4, 56.7);
        object.set_location(new_loc);
        assert_eq!(object.get_attr().location, new_loc);
    }

    #[test]
    fn set_stroke_weight_test() {
        let mut object = Object {
            attrs: Attributes::default(),
        };

        let new_weight = 12.3;
        object.set_stroke_weight(new_weight);
        assert_eq!(object.get_attr().stroke_weight, new_weight);
    }

    #[test]
    fn set_color_test() {
        let mut object = Object {
            attrs: Attributes::default(),
        };

        let new_color = (12, 34, 56, 78);
        object.set_rgba(new_color.0, new_color.1, new_color.2, new_color.3);
        assert_eq!(
            object.get_attr().color,
            rgb8(new_color.0, new_color.1, new_color.2)
        );
        assert_eq!(object.get_attr().alpha, new_color.3);
    }

    #[test]
    fn golf_call_test() {
        let mut object = Object {
            attrs: Attributes::default(),
        };

        let new_color = (252, 253, 254, 255);
        let new_weight = 9.76;
        let new_loc = Point2::new(12.3, 4.56);
        object
            .set_rgba(new_color.0, new_color.1, new_color.2, new_color.3)
            .set_location(new_loc)
            .set_stroke_weight(new_weight);

        assert_eq!(
            object.get_attr().color,
            rgb8(new_color.0, new_color.1, new_color.2)
        );
        assert_eq!(object.get_attr().alpha, new_color.3);
        assert_eq!(object.get_attr().stroke_weight, new_weight);
    }
}
