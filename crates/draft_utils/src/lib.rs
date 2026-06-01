use fyrox_core::algebra::{Affine3, Vector4};

pub trait AffineExt {
    fn to_transpose(self) -> [Vector4<f32>; 3];

    fn inverse_transpose_3x3(self) -> ([Vector4<f32>; 2], f32);
}

impl AffineExt for Affine3<f32> {
    fn inverse_transpose_3x3(self) -> ([Vector4<f32>; 2], f32) {
        let inverse_transpose_3x3 = self.inverse().matrix().transpose();

        (
            [
                Vector4::new(
                    inverse_transpose_3x3.column(0).x,
                    inverse_transpose_3x3.column(0).y,
                    inverse_transpose_3x3.column(0).z,
                    inverse_transpose_3x3.column(1).x,
                ),
                Vector4::new(
                    inverse_transpose_3x3.column(1).y,
                    inverse_transpose_3x3.column(1).z,
                    inverse_transpose_3x3.column(2).x,
                    inverse_transpose_3x3.column(2).y,
                ),
            ],
            inverse_transpose_3x3.column(2).z,
        )
    }

    fn to_transpose(self) -> [Vector4<f32>; 3] {
        let transpose_3x3 = self.matrix().transpose();

        [
            transpose_3x3.column(0).into(),
            transpose_3x3.column(1).into(),
            transpose_3x3.column(2).into(),
        ]
    }
}
