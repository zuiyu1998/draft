use bytes::BufMut;
use fyrox_core::{
    algebra::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4},
    array_as_u8_slice,
    color::Color,
    value_as_u8_slice,
};

pub trait Std140 {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32);
}

macro_rules! default_impl_std140 {
    ($type: ty, $alignment:expr) => {
        impl Std140 for $type {
            fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
                *size += $alignment;
                dest.put_slice(value_as_u8_slice(self));
            }
        }
    };
}

impl<T: Std140> Std140 for &[T] {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        for item in self.iter() {
            item.write(dest, size);
        }
    }
}

impl<T: Std140> Std140 for Vec<T> {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        for item in self.iter() {
            item.write(dest, size);
        }
    }
}

default_impl_std140!(u32, 4);
default_impl_std140!(i32, 4);
default_impl_std140!(f32, 4);
default_impl_std140!(Vector2<f32>, 8);
default_impl_std140!(Vector3<f32>, 12);
default_impl_std140!(Vector4<f32>, 16);
default_impl_std140!(Matrix4<f32>, 16);

impl Std140 for bool {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        *size += 4;
        let integer = if *self { 1 } else { 0 };
        dest.put_i32(integer);
    }
}

impl Std140 for Matrix2<f32> {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        *size += 16;

        for row in (self as &dyn AsRef<[[f32; 2]; 2]>).as_ref() {
            dest.put_slice(array_as_u8_slice(row));
            dest.put_slice(&[0; 2 * size_of::<f32>()]);
        }
    }
}

impl Std140 for Matrix3<f32> {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        *size += 16;

        for row in (self as &dyn AsRef<[[f32; 3]; 3]>).as_ref() {
            dest.put_slice(array_as_u8_slice(row));
            dest.put_slice(&[0; size_of::<f32>()]);
        }
    }
}

impl Std140 for Color {
    fn write(&self, dest: &mut dyn BufMut, size: &mut u32) {
        *size += 16;

        let frgba = self.as_frgba();
        dest.put_slice(value_as_u8_slice(&frgba));
    }
}
