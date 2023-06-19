use glam::Vec3A;

pub struct Onb {
    axis: [Vec3A; 3],
}

impl Onb {
    pub fn build_from_w(n: &Vec3A) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            Vec3A::new(0., 1., 0.)
        } else {
            Vec3A::new(1., 0., 0.)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);

        Self { axis: [u, v, w] }
    }

    #[inline(always)]
    pub fn u(&self) -> Vec3A {
        self.axis[0]
    }

    #[inline(always)]
    pub fn v(&self) -> Vec3A {
        self.axis[1]
    }

    #[inline(always)]
    pub fn w(&self) -> Vec3A {
        self.axis[2]
    }

    pub fn local(&self, a: &Vec3A) -> Vec3A {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}
