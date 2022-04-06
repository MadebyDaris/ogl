pub struct affine_matrix {
    pub matrix: [[f32;4]; 4]
}
impl affine_matrix {
    pub fn new() -> Self {
        Self{ matrix:[[1.0, 0.0, 0.0, 0.0],[0.0, 1.0, 0.0, 0.0],[0.0, 0.0, 1.0, 0.0],[0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn translate(self, kx: f32, ky: f32) -> [[f32;4];4] {
        let mut mat = self.matrix;
        mat[0][3] += kx;
        mat[1][3] += ky;
        return mat;
    }
    
    pub fn scale(self, kx: f32, ky: f32) -> [[f32;4];4] {
        let mut mat = self.matrix;
        mat[0][0] *= kx;
        mat[1][1] *= ky;
        return mat;
    }

    pub fn rotate(self, theta: f32) ->  [[f32;4];4] {
        let mut mat = self.matrix;
        mat[0][0] = theta.cos();
        mat[0][1] = -(theta.sin());
        mat[1][0] = theta.sin();
        mat[1][1] = theta.cos();
        return mat
    }
}