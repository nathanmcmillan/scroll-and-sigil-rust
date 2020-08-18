pub fn identity(matrix: &mut [f32; 16]) {
    matrix[0] = 1.0;
    matrix[1] = 0.0;
    matrix[2] = 0.0;
    matrix[3] = 0.0;

    matrix[4] = 0.0;
    matrix[5] = 1.0;
    matrix[6] = 0.0;
    matrix[7] = 0.0;

    matrix[8] = 0.0;
    matrix[9] = 0.0;
    matrix[10] = 1.0;
    matrix[11] = 0.0;

    matrix[12] = 0.0;
    matrix[13] = 0.0;
    matrix[14] = 0.0;
    matrix[15] = 1.0;
}

pub fn orthographic(matrix: &mut [f32; 16], left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
    matrix[0] = 2.0 / (right - left);
    matrix[1] = 0.0;
    matrix[2] = 0.0;
    matrix[3] = 0.0;

    matrix[4] = 0.0;
    matrix[5] = 2.0 / (top - bottom);
    matrix[6] = 0.0;
    matrix[7] = 0.0;

    matrix[8] = 0.0;
    matrix[9] = 0.0;
    matrix[10] = -2.0 / (far - near);
    matrix[11] = 0.0;

    matrix[12] = -(right + left) / (right - left);
    matrix[13] = -(top + bottom) / (top - bottom);
    matrix[14] = -(far + near) / (far - near);
    matrix[15] = 1.0;
}

pub fn frustum(matrix: &mut [f32; 16], left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
    matrix[0] = (2.0 * near) / (right - left);
    matrix[1] = 0.0;
    matrix[2] = 0.0;
    matrix[3] = 0.0;

    matrix[4] = 0.0;
    matrix[5] = (2.0 * near) / (top - bottom);
    matrix[6] = 0.0;
    matrix[7] = 0.0;

    matrix[8] = (right + left) / (right - left);
    matrix[9] = (top + bottom) / (top - bottom);
    matrix[10] = -(far + near) / (far - near);
    matrix[11] = -1.0;

    matrix[12] = 0.0;
    matrix[13] = 0.0;
    matrix[14] = -(2.0 * far * near) / (far - near);
    matrix[15] = 0.0;
}

pub fn perspective(matrix: &mut [f32; 16], fov: f32, near: f32, far: f32, aspect: f32) {
    let top = near * (fov * std::f32::consts::PI / 360.0).tan();
    let bottom = -top;
    let left = bottom * aspect;
    let right = top * aspect;

    frustum(matrix, left, right, bottom, top, near, far);
}

pub fn translate(matrix: &mut [f32; 16], x: f32, y: f32, z: f32) {
    matrix[12] = x * matrix[0] + y * matrix[4] + z * matrix[8] + matrix[12];
    matrix[13] = x * matrix[1] + y * matrix[5] + z * matrix[9] + matrix[13];
    matrix[14] = x * matrix[2] + y * matrix[6] + z * matrix[10] + matrix[14];
    matrix[15] = x * matrix[3] + y * matrix[7] + z * matrix[11] + matrix[15];
}

pub fn multiply(matrix: &mut [f32; 16], a: &[f32; 16], b: &[f32; 16]) {
    matrix[0] = a[0] * b[0] + a[4] * b[1] + a[8] * b[2] + a[12] * b[3];
    matrix[1] = a[1] * b[0] + a[5] * b[1] + a[9] * b[2] + a[13] * b[3];
    matrix[2] = a[2] * b[0] + a[6] * b[1] + a[10] * b[2] + a[14] * b[3];
    matrix[3] = a[3] * b[0] + a[7] * b[1] + a[11] * b[2] + a[15] * b[3];

    matrix[4] = a[0] * b[4] + a[4] * b[5] + a[8] * b[6] + a[12] * b[7];
    matrix[5] = a[1] * b[4] + a[5] * b[5] + a[9] * b[6] + a[13] * b[7];
    matrix[6] = a[2] * b[4] + a[6] * b[5] + a[10] * b[6] + a[14] * b[7];
    matrix[7] = a[3] * b[4] + a[7] * b[5] + a[11] * b[6] + a[15] * b[7];

    matrix[8] = a[0] * b[8] + a[4] * b[9] + a[8] * b[10] + a[12] * b[11];
    matrix[9] = a[1] * b[8] + a[5] * b[9] + a[9] * b[10] + a[13] * b[11];
    matrix[10] = a[2] * b[8] + a[6] * b[9] + a[10] * b[10] + a[14] * b[11];
    matrix[11] = a[3] * b[8] + a[7] * b[9] + a[11] * b[10] + a[15] * b[11];

    matrix[12] = a[0] * b[12] + a[4] * b[13] + a[8] * b[14] + a[12] * b[15];
    matrix[13] = a[1] * b[12] + a[5] * b[13] + a[9] * b[14] + a[13] * b[15];
    matrix[14] = a[2] * b[12] + a[6] * b[13] + a[10] * b[14] + a[14] * b[15];
    matrix[15] = a[3] * b[12] + a[7] * b[13] + a[11] * b[14] + a[15] * b[15];
}
