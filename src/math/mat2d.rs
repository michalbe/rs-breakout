use crate::math::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Mat2d {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
}

impl Mat2d {
    pub fn empty() -> Mat2d {
        Mat2d {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m10: 1.0,
            m11: 0.0,
            m12: 0.0,
        }
    }

    pub fn from_translation(v: Vec2) -> Mat2d {
        let mut out = Mat2d::empty();
        out.m00 = 1.0;
        out.m01 = 0.0;
        out.m02 = 0.0;
        out.m10 = 1.0;
        out.m11 = v.x;
        out.m12 = v.y;

        out
    }

    pub fn get_translation(a: Mat2d) -> Vec2 {
        let mut out = Vec2::empty();

        out.x = a.m11;
        out.y = a.m12;

        out
    }

    pub fn rotate(a: Mat2d, rad: f32)  -> Mat2d {
        let mut out = Mat2d::empty();

        let s = rad.sin();
        let c = rad.cos();
        out.m00 = a.m00 * c + a.m02 * s;
        out.m01 = a.m01 * c + a.m10 * s;
        out.m02 = a.m02 * -s + a.m02 * c;
        out.m10 = a.m10 * -s + a.m10 * c;
        out.m11 = a.m11;
        out.m12 = a.m12;

        out
    }

    pub fn scale(a: Mat2d, v: Vec2)  -> Mat2d {
        let mut out = Mat2d::empty();
        out.m00 = a.m00 * v.x;
        out.m01 = a.m01 * v.x;
        out.m02 = a.m02 * v.y;
        out.m10 = a.m10 * v.y;
        out.m11 = a.m11;
        out.m12 = a.m12;

        out
    }

    pub fn invert(a: Mat2d) -> Mat2d {
        let mut out = Mat2d::empty();

        let aa = a.m00;
        let ab = a.m01;
        let ac = a.m02;
        let ad = a.m10;
        let atx = a.m11;
        let aty = a.m12;

        let mut det = aa * ad - ab * ac;

        if det == 0.0 {
            panic!("Mat2d determinant cannot be 0");
        }

        det = 1.0 / det;

        out.m00 = ad * det;
        out.m01 = -ab * det;
        out.m02 = -ac * det;
        out.m10 = aa * det;
        out.m11 = (ac * aty - ad * atx) * det;
        out.m12 = (ab * atx - aa * aty) * det;

        out
    }
}
