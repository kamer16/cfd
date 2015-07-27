use time;

pub struct Fps {
    old: time::Timespec,
    elapsed: time::Duration,
    cnt: u32,
}

impl Fps {
    pub fn new() -> Fps{
        Fps {
            old: time::get_time(),
            elapsed: time::Duration::seconds(0),
            cnt: 0,
        }
    }

    // returns fps or 0 if fps has not changed
    pub fn update(&mut self) -> u32 {
        self.cnt += 1;
        let new = time::get_time();
        self.elapsed = self.elapsed + (new - self.old);
        self.old = new;
        if self.elapsed > time::Duration::seconds(1) {
            let res = self.cnt;
            self.cnt = 0;
            self.elapsed = self.elapsed - time::Duration::seconds(1);
            return res;
        }
        return 0;
    }
}
