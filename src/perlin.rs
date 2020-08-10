pub use crate::random::*;
pub use crate::vec3::Vec3;

const POINTCOUNT: usize = 256;

fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], fu: f64, fv: f64, fw: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(fu - i as f64, fv - j as f64, fw - k as f64);
                accum += (i as f64 * fu + ((1 - i) as f64) * (1.0 - fu))
                    * (j as f64 * fv + ((1 - j) as f64) * (1.0 - fv))
                    * (k as f64 * fw + ((1 - k) as f64) * (1.0 - fw))
                    * (c[i][j][k].clone() * weight_v);
            }
        }
    }
    accum
}

pub struct Perlin {
    pub ranvec: [Vec3; POINTCOUNT],
    pub perm_x: [usize; POINTCOUNT],
    pub perm_y: [usize; POINTCOUNT],
    pub perm_z: [usize; POINTCOUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::needless_range_loop)]
impl Perlin {
    pub fn permute(p: &mut [usize; POINTCOUNT]) {
        for i in 0..POINTCOUNT {
            let j = random_int_range(i as i32, POINTCOUNT as i32) as usize;
            p.swap(i, j);
        }
    }
    pub fn perlin_generate_perm() -> [usize; POINTCOUNT] {
        let mut p: [usize; POINTCOUNT] = [0; POINTCOUNT];
        for i in 0..POINTCOUNT {
            p[i] = i;
        }
        Self::permute(&mut p);
        p
    }
    pub fn new() -> Self {
        let mut ranvec = [
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
            Vec3::default(),
        ];
        for i in 0..POINTCOUNT {
            ranvec[i] = Vec3::random_range(-1.0, 1.0);
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        /*let i = (((4.0 * p.x) as i32) & (POINTCOUNT as i32 - 1)) as usize;
        let j = (((4.0 * p.y) as i32) & (POINTCOUNT as i32 - 1)) as usize;
        let k = (((4.0 * p.z) as i32) & (POINTCOUNT as i32 - 1)) as usize;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]*/
        let mut fu = p.x - p.x.floor();
        let mut fv = p.y - p.y.floor();
        let mut fw = p.z - p.z.floor();
        fu = fu * fu * (3.0 - 2.0 * fu);
        fv = fv * fv * (3.0 - 2.0 * fv);
        fw = fw * fw * (3.0 - 2.0 * fw);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut cc = [
            [
                [Vec3::default(), Vec3::default()],
                [Vec3::default(), Vec3::default()],
            ],
            [
                [Vec3::default(), Vec3::default()],
                [Vec3::default(), Vec3::default()],
            ],
        ];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    cc[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                        .clone();
                }
            }
        }
        trilinear_interp(cc, fu, fv, fw)
    }
    pub fn turb(&self, p: Vec3) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut temp_p = p;
        for _i in 0..7 {
            accum += weight * self.noise(temp_p.clone());
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
