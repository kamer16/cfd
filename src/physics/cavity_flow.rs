use std::sync::Arc;
use std::thread;
use std::mem;
use physics::Simulation;
use grid;

pub struct CavityFlow {
    pub u: Arc<grid::Grid>,
    pub v: Arc<grid::Grid>,
    pub b: Arc<grid::Grid>,
    pub p: Arc<grid::Grid>,
    pub buff: grid::Grid,
    width: u32,
    height: u32,
    dx: f32,
    dy: f32,
    sigma: f32,
    dt: f32,
    rho: f32,
    nu: f32,
}

impl CavityFlow {
    pub fn new(width: u32, height: u32) -> CavityFlow {
        let res = CavityFlow {
            u: Arc::new(grid::Grid::new(width, height)),
            v: Arc::new(grid::Grid::new(width, height)),
            b: Arc::new(grid::Grid::new(width, height)),
            p: Arc::new(grid::Grid::new(width, height)),
            buff: grid::Grid::new(width, height),
            width: width,
            height: height,
            dx: 1. / width as f32,
            dy: 1. / height as f32,
            sigma: 0.2,
            dt: 0.2 * 1. / width as f32,
            rho: 1.,
            nu: 1.,
        };
        res
    }
}

impl CavityFlow {
    pub fn build_up(&mut self, chunk_size: usize) {
        let dt = self.dt;
        let dx = self.dx;
        let dy = self.dy;
        let width = self.width;
        let height = self.height;

        {
        // TODO repeat this nit times for convergence
        let mut vec = Vec::new();
        for (chunk_id, chunk) in self.buff.chunks_mut(chunk_size).enumerate() {
            let u = self.u.clone();
            let v = self.v.clone();
            vec.push(thread::scoped(move || {
                for (id, elt) in chunk.iter_mut().enumerate() {
                    let idx = chunk_id * chunk_size + id;
                    if idx < width as usize || idx as u32 >= width * (height - 1) {
                        continue;
                    }
                    let ul = u[(idx - 1)];
                    let ur = u[(idx + 1)];
                    let ut = u[(idx - width as usize)];
                    let ub = u[(idx + width as usize)];
                    let vl = v[(idx - 1)];
                    let vr = v[(idx + 1)];
                    let vt = v[(idx - width as usize)];
                    let vb = v[(idx + width as usize)];
                    let mut val = 1. / dt * ((ur - ul) / (2.*dx) + (vt - vb) / (2.*dy)) -
                        (ur - ul) * (ur - ul) / (2.*2.*dx*dx) -
                        2. * (ut - ub) * (vr - vl) / (2.*2.*dy*dx) -
                        (vt - vb) * (vt - vb) * (2.*2.*dy*dy);


                    // Some hardware are slow on subnormal numbers
                    if !val.is_normal() && val != 0.{
                        val = 0.;
                    }
                    *elt = val;
                }
            }));
        }
        }

        // Swap buff and b
        unsafe {
            mem::swap(self.b.make_unique(), &mut self.buff);
        }
    }
    pub fn pressure_poisson(&mut self, chunk_size: usize) {
        let dx = self.dx;
        let dy = self.dy;
        let width = self.width;
        let height = self.height;
        let rho = self.rho;

        // TODO repeat this nit times for convergence
        {
        let mut vec = Vec::new();
        for (chunk_id, chunk) in self.buff.chunks_mut(chunk_size).enumerate() {
            let data = self.p.clone();
            let pres_pois = self.b.clone();
            vec.push(thread::scoped(move || {
                for (id, elt) in chunk.iter_mut().enumerate() {
                    let idx = chunk_id * chunk_size + id;
                    if idx < width as usize || idx as u32 >= width * (height - 1) {
                        continue;
                    }
                    let l = data[(idx - 1)];
                    let r = data[(idx + 1)];
                    let t = data[(idx - width as usize)];
                    let b = data[(idx + width as usize)];
                    let mut val = ((r + l) * dy*dy + (t + b) * dx*dx) / (2.*(dx*dx + dy*dy)) -
                        rho*dx*dx*dy*dy / (2.*(dx*dx + dy*dy)) * pres_pois[idx];


                    // Some hardware are slow on subnormal numbers
                    if !val.is_normal() && val != 0.{
                        val = 0.;
                    }
                    *elt = val;
                }
            }));
        }
        }

        // Bounderies for pressure
        for idx in 0..self.height {
            self.buff[(idx*self.width) as usize] =
                self.buff[(idx*self.width + 1) as usize];
            self.buff[((idx+1)*self.width - 1) as usize] = 0.;
        }
        for idx in 0..self.width {
            self.buff[idx as usize] = self.buff[(idx + self.width) as usize];
        }
        for idx in self.width*(self.height-1)..self.width*self.height {
            self.buff[idx as usize] = self.buff[(idx - self.width) as usize];
        }

        // Swap buff and p
        unsafe {
            mem::swap(self.p.make_unique(), &mut self.buff);
        }
    }

    pub fn update_u(&mut self, chunk_size: usize) {
        let dt = self.dt;
        let dx = self.dx;
        let dy = self.dy;
        let width = self.width;
        let height = self.height;
        let rho = self.rho;
        let nu = self.nu;

        {
        // TODO repeat this nit times for convergence
        let mut vec = Vec::new();
        for (chunk_id, chunk) in self.buff.chunks_mut(chunk_size).enumerate() {
            let data = self.u.clone();
            let v = self.v.clone();
            let p = self.p.clone();
            vec.push(thread::scoped(move || {
                for (id, elt) in chunk.iter_mut().enumerate() {
                    let idx = chunk_id * chunk_size + id;
                    if idx < width as usize || idx as u32 >= width * (height - 1) {
                        continue;
                    }
                    let vc = v[idx];
                    let pl = p[(idx - 1)];
                    let pr = p[(idx + 1)];
                    let c = data[idx];
                    let l = data[(idx - 1)];
                    let r = data[(idx + 1)];
                    let t = data[(idx - width as usize)];
                    let b = data[(idx + width as usize)];
                    let mut val = c - c * dt / dx * (c - l) - vc * dt / dy * (c - b) -
                        dt / (rho*2.*dx) * (pr - pl) + nu * (dt / (dx*dx) * (r - 2.*c + l) +
                        dt / (dy*dy) * (t - 2.*c + b));

                    // Some hardware are slow on subnormal numbers
                    if !val.is_normal() && val != 0.{
                        val = 0.;
                    }
                    *elt = val;
                }
            }));
        }
        }

        // Bounderies for velocity in u
        for idx in 0..self.height {
            self.buff[((idx+1)*self.width - 1) as usize] = 0.;
            self.buff[(idx*self.width) as usize] = 0.;
        }

        for idx in 0..self.width {
            self.buff[idx as usize] = 1.; // Lid
            self.buff[(idx + (self.height-1) * self.width) as usize] = 0.;
        }
        // Swap buff and p
        unsafe {
            mem::swap(self.u.make_unique(), &mut self.buff);
        }
    }

    pub fn update_v(&mut self, chunk_size: usize) {
        let dt = self.dt;
        let dx = self.dx;
        let dy = self.dy;
        let width = self.width;
        let height = self.height;
        let rho = self.rho;
        let nu = self.nu;

        {
        // TODO repeat this nit times for convergence
        let mut vec = Vec::new();
        for (chunk_id, chunk) in self.buff.chunks_mut(chunk_size).enumerate() {
            let data = self.v.clone();
            let u = self.u.clone();
            let p = self.p.clone();
            vec.push(thread::scoped(move || {
                for (id, elt) in chunk.iter_mut().enumerate() {
                    let idx = chunk_id * chunk_size + id;
                    if idx < width as usize || idx as u32 >= width * (height - 1) {
                        continue;
                    }
                    let uc = u[idx];
                    let pt = p[(idx - width as usize)];
                    let pb = p[(idx + width as usize)];
                    let c = data[idx];
                    let l = data[(idx - 1)];
                    let r = data[(idx + 1)];
                    let t = data[(idx - width as usize)];
                    let b = data[(idx + width as usize)];
                    let mut val = c - uc * dt / dx * (c - l) - c * dt / dy * (c - b) -
                        dt / (rho*2.*dy) * (pt - pb) + nu * (dt / (dx*dx) * (r - 2.*c + l) +
                        dt / (dy*dy) * (t - 2.*c + b));

                    // Some hardware are slow on subnormal numbers
                    if !val.is_normal() && val != 0.{
                        val = 0.;
                    }
                    *elt = val;
                }
            }));
        }
        }

        // Bounderies for velocity in v
        for idx in 0..self.height {
            self.buff[((idx+1)*self.width - 1) as usize] = 0.;
            self.buff[(idx*self.width) as usize] = 0.;
        }

        for idx in 0..self.width {
            self.buff[idx as usize] = 0.;
            self.buff[(idx + (self.height-1) * self.width) as usize] = 0.;
        }
        // Swap buff and p
        unsafe {
            mem::swap(self.v.make_unique(), &mut self.buff);
        }
    }
}

impl Simulation for CavityFlow {

    fn get_grid(&mut self) -> &grid::Grid {
        unsafe {
            self.p.make_unique()
        }
    }

    fn update(&mut self, nb_th: u32) {
        let chunk_size = (self.height / nb_th * self.width) as usize;
        self.build_up(chunk_size);
        self.pressure_poisson(chunk_size);
        self.update_u(chunk_size);
        self.update_v(chunk_size);
    }
}
