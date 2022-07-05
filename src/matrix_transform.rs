#[derive(Copy, Clone)]
pub struct AffineMatrix {
    pub matrix: [[f32;4]; 4]
}
impl AffineMatrix {
    pub fn new() -> Self {
        Self{ matrix:[[1.0, 0.0, 0.0, 0.0],[0.0, 1.0, 0.0, 0.0],[0.0, 0.0, 1.0, 0.0],[0.0, 0.0, 0.0, 1.0]] }
    }

    fn mat_mul(m_: AffineMatrix, m: AffineMatrix) -> [[f32;4];4] {
        let m1 = m_.matrix;
        let m2 = m.matrix;
        let mut f = AffineMatrix::new().matrix;

        for i in 0..3 {
            for j in 0..3 {
                f[i][j] = m1[i][j] * m2[j][i]
            }
        }
        return f;
    }

    pub fn vec_mat_mul(m: AffineMatrix, v: [f32;4]) -> [f32;4]{
        let mat = m.matrix;
        let mut z = [0.;4];
        for i in 0..3{
            for j in 0..3{
                let t = v[i] * mat[i][j];
                z[i] += t
            }
        }
        return z;
    }

    pub fn translate(self, kx: f32, ky: f32) -> [[f32;4];4] {
        let mut mat = self.matrix;
        mat[0][3] += kx;
        mat[1][3] += ky;
        return mat;
    }
    
    pub fn scale(self, kx: f32, ky: f32, kz:f32) -> [[f32;4];4] {
        let mut mat = self.matrix;
        mat[0][0] *= kx;
        mat[1][1] *= ky;
        mat[2][2] *= kz;
        return mat;
    }

    pub fn rotate(rot_param: (f32, f32, f32)) ->  [[f32;4];4] {
        let mut mat = AffineMatrix::new();
// X AXIS ROTATION
        let mut x_rot = AffineMatrix::new().matrix;
        x_rot[0][0] = 1.;
        x_rot[1][1] = rot_param.0.cos();
        x_rot[1][2] = -(rot_param.0.sin());
        x_rot[2][1] = (rot_param.0.sin());
        x_rot[2][2] = (rot_param.0.cos());

        let mut y_rot = AffineMatrix::new().matrix;
        y_rot[0][0] = rot_param.1.cos();
        y_rot[0][2] = -(rot_param.1.sin());
        y_rot[1][1] = 1.;
        y_rot[2][0] = (rot_param.1.sin());
        y_rot[2][2] = (rot_param.1.cos());

        let mut z_rot = AffineMatrix::new().matrix;
        z_rot[0][0] = rot_param.2.cos();
        z_rot[0][1] = -(rot_param.2.sin());
        z_rot[1][0] = (rot_param.2.sin());
        z_rot[1][1] = (rot_param.2.cos());
        z_rot[2][2] = 1.;

        let x_y_rot = AffineMatrix::mat_mul(AffineMatrix{matrix: x_rot}, AffineMatrix{matrix:y_rot});
        let f = AffineMatrix::mat_mul(AffineMatrix {matrix: x_y_rot}, AffineMatrix {matrix:z_rot} );
        return f
    }
}