use crate::bound::Interval;
use crate::ray::Ray3;

pub struct Aabb(Interval, Interval, Interval);

impl Aabb {
    pub fn x(&self) -> &Interval {
        &self.0
    }

    pub fn y(&self) -> &Interval {
        &self.1
    }

    pub fn z(&self) -> &Interval {
        &self.2
    }

    pub fn hit(&self, ray: &Ray3, hit_range: &Interval) -> bool {
        for (ax_idx, interval) in [&self.0, &self.1, &self.2].into_iter().enumerate() {
            let adinv = 1.0 / ray.vec()[ax_idx];

            let t0 = (interval.start() - ray.origin().as_vec()[ax_idx]) * adinv;
            let t1 = (interval.end() - ray.origin().as_vec()[ax_idx]) * adinv;

            let int = if t0 < t1 {
                Interval::new(
                    if t0 > hit_range.start() {
                        t0
                    } else {
                        hit_range.start()
                    },
                    if t1 < hit_range.end() {
                        t1
                    } else {
                        hit_range.end()
                    },
                )
            } else {
                Interval::new(
                    if t1 > hit_range.start() {
                        t1
                    } else {
                        hit_range.start()
                    },
                    if t0 < hit_range.end() {
                        t0
                    } else {
                        hit_range.end()
                    },
                )
            };

            if int.end() <= int.start() {
                return false;
            }
        }
        
        return true;
    }
}
