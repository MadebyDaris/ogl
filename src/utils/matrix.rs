#[derive(Copy, Clone)]
pub struct ModelMat {
    pub matrix: [[f32;4]; 4]
}
impl ModelMat {
    pub fn identity() -> Self {
        Self{ matrix:[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]] 
        }
    }

    fn mat_x_mat(m_: ModelMat, m: ModelMat) -> ModelMat {
        let m1 = m_.matrix;
        let m2 = m.matrix;
        let mut f = [[0.0; 4]; 4];

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    f[i][j] += m1[i][k] * m2[k][j]
                }
            }
        }
        return ModelMat { matrix : f };
    }

    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix[3][0] += x;
        self.matrix[3][1] += y;
        self.matrix[3][2] += z;
        self
    }
    
    pub fn scale(self, kx: f32, ky: f32, kz:f32) -> ModelMat {
        let mut mat = self.matrix;
        mat[0][0] *= kx;
        mat[1][1] *= ky;
        mat[2][2] *= kz;
        return ModelMat { matrix: mat };
    }

    pub fn rotate(&mut self, rot_param: (f32, f32, f32)) ->  ModelMat {
        let mut x_rot = ModelMat::identity().matrix;
        x_rot[0][0] = 1.;
        x_rot[1][1] = rot_param.0.cos();
        x_rot[1][2] = -(rot_param.0.sin());
        x_rot[2][1] = rot_param.0.sin();
        x_rot[2][2] = rot_param.0.cos();

        let mut y_rot = ModelMat::identity().matrix;
        y_rot[0][0] = rot_param.1.cos();
        y_rot[0][2] = -(rot_param.1.sin());
        y_rot[1][1] = 1.;
        y_rot[2][0] = rot_param.1.sin();
        y_rot[2][2] = rot_param.1.cos();

        let mut z_rot = ModelMat::identity().matrix;
        z_rot[0][0] = rot_param.2.cos();
        z_rot[0][1] = -(rot_param.2.sin());
        z_rot[1][0] = rot_param.2.sin();
        z_rot[1][1] = rot_param.2.cos();
        z_rot[2][2] = 1.;

        let x_y_rot = ModelMat::mat_x_mat(ModelMat{matrix: x_rot}, ModelMat{ matrix: y_rot });
        let combined = ModelMat::mat_x_mat(ModelMat { matrix: x_y_rot.matrix }, ModelMat { matrix:z_rot } );
        let result = ModelMat::mat_x_mat( *self, combined );
        return result
    }

    pub fn print_matrix(matrix: [[f32; 4]; 4]) {
        for row in matrix.iter() {
            for element in row.iter() {
                print!("{} ", element);
            }
            println!();
        }
    }
}