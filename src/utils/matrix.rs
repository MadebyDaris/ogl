#[derive(Copy, Clone)]
pub struct TransformMatrix {
    pub matrix: [[f32;4]; 4]
}
impl TransformMatrix {
    pub fn identity() -> Self {
        Self{ matrix:[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]] 
        }
    }

    fn mat_x_mat(m_: TransformMatrix, m: TransformMatrix) -> TransformMatrix {
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
        return TransformMatrix { matrix : f };
    }

    pub fn translate(mut self, kx: f32, ky: f32, kz: f32) -> Self {
        self.matrix[3][0] += kx;
        self.matrix[3][1] += ky;
        self.matrix[3][2] += kz;
        self
    }
    
    pub fn scale(mut self, kx: f32, ky: f32, kz:f32) -> Self {
        self.matrix[0][0] *= kx;
        self.matrix[1][1] *= ky;
        self.matrix[2][2] *= kz;
        self
    }

    pub fn rotate(&mut self, rot_param: (f32, f32, f32)) ->  TransformMatrix {
        let mut x_rot = TransformMatrix::identity().matrix;
        x_rot[0][0] = 1.;
        x_rot[1][1] = rot_param.0.cos();
        x_rot[1][2] = -(rot_param.0.sin());
        x_rot[2][1] = rot_param.0.sin();
        x_rot[2][2] = rot_param.0.cos();

        let mut y_rot = TransformMatrix::identity().matrix;
        y_rot[0][0] = rot_param.1.cos();
        y_rot[0][2] = -(rot_param.1.sin());
        y_rot[1][1] = 1.;
        y_rot[2][0] = rot_param.1.sin();
        y_rot[2][2] = rot_param.1.cos();

        let mut z_rot = TransformMatrix::identity().matrix;
        z_rot[0][0] = rot_param.2.cos();
        z_rot[0][1] = -(rot_param.2.sin());
        z_rot[1][0] = rot_param.2.sin();
        z_rot[1][1] = rot_param.2.cos();
        z_rot[2][2] = 1.;

        let x_y_rot = TransformMatrix::mat_x_mat(TransformMatrix{matrix: x_rot}, TransformMatrix{ matrix: y_rot });
        let combined = TransformMatrix::mat_x_mat(TransformMatrix { matrix: x_y_rot.matrix }, TransformMatrix { matrix:z_rot } );
        let result = TransformMatrix::mat_x_mat( *self, combined );
        return result
    }
}