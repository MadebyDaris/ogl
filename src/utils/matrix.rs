#[derive(Copy, Clone)]
pub struct ModelMat {
    pub matrix: [[f32;4]; 4]
}
impl ModelMat {
    pub fn new() -> Self {
        Self{ matrix:[[1.0, 0.0, 0.0, 0.0],[0.0, 1.0, 0.0, 0.0],[0.0, 0.0, 1.0, 0.0],[0.0, 0.0, 0.0, 1.0]] }
    }

    fn mat_x_mat(m_: ModelMat, m: ModelMat) -> ModelMat {
        let m1 = m_.matrix;
        let m2 = m.matrix;
        let mut f = ModelMat::new().matrix;

        for i in 0..3 {
            for j in 0..3 {
                f[i][j] = m1[i][j] * m2[j][i]
            }
        }
        return ModelMat { matrix : f };
    }

    pub fn vec_x_mat(m: ModelMat, v: [f32;4]) -> [f32;4]{
        let mat = m.matrix;
        let mut z = [0.;4];
        for i in 0..3 {
            for j in 0..3{
                let t = v[i] * mat[i][j];
                z[i] += t
            }
        }
        return z;
    }

    pub fn translate(mut self, kx: f32, ky: f32, kz: f32) -> Self {
        let mut mat = [0.;3];
        mat[0] = kx;
        mat[1] = ky;
        mat[2] = kz;

        self.matrix[0][3] = kx;
        self.matrix[1][3] = ky;
        self.matrix[2][3] = kz;
        return self;
    }
    
    pub fn scale(mut self, kx: f32, ky: f32, kz:f32) -> ModelMat {
        self.matrix[0][0] = kx;
        self.matrix[1][1] = ky;
        self.matrix[2][2] = kz;
        return self;
    }

    pub fn rotate(&mut self, rot_param: (f32, f32, f32)) ->  ModelMat {

        let mut x_rot = ModelMat::new().matrix;
        x_rot[0][0] = 1.;
        x_rot[1][1] = rot_param.0.cos();
        x_rot[1][2] = -(rot_param.0.sin());
        x_rot[2][1] = rot_param.0.sin();
        x_rot[2][2] = rot_param.0.cos();

        let mut y_rot = ModelMat::new().matrix;
        y_rot[0][0] = rot_param.1.cos();
        y_rot[0][2] = -(rot_param.1.sin());
        y_rot[1][1] = 1.;
        y_rot[2][0] = rot_param.1.sin();
        y_rot[2][2] = rot_param.1.cos();

        let mut z_rot = ModelMat::new().matrix;
        z_rot[0][0] = rot_param.2.cos();
        z_rot[0][1] = -(rot_param.2.sin());
        z_rot[1][0] = rot_param.2.sin();
        z_rot[1][1] = rot_param.2.cos();
        z_rot[2][2] = 1.;

        let x_y_rot = ModelMat::mat_x_mat(ModelMat{matrix: x_rot}, ModelMat{ matrix: y_rot });
        let f = ModelMat::mat_x_mat(ModelMat { matrix: x_y_rot.matrix }, ModelMat { matrix:z_rot } );
        let z = ModelMat::mat_x_mat( *self, f );
        return z
    }
}