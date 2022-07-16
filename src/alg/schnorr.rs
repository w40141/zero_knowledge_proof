use rand::Rng;

pub struct Proofer {
    x: i64,
    y: i64,
    p: i64,
    g: i64,
    q: Option<i64>,
    r: Option<i64>,
}

impl Proofer {
    pub fn new(x: i64, y: i64, p: i64, g: i64) -> Self {
        Self {
            x,
            y,
            p,
            g,
            q: None,
            r: None,
        }
    }

    fn q(&self) -> i64 {
        match self.q {
            Some(i) => i,
            None => {
                let mut q: i64 = self.g;
                while q != 1 {
                    q = q * self.g % self.p;
                }
                q
            }
        }
    }

    fn r(&self) -> i64 {
        match self.r {
            Some(i) => i,
            None => {
                let mut rng = rand::thread_rng();
                let r: i64 = rng.gen_range(0..=self.q() - 1);
                r
            }
        }
    }

    pub fn c(&self) -> i64 {
        self.g.pow(self.r() as u32) % self.p
    }

    pub fn z(&self, e: i64) -> i64 {
        (self.r() + (e * self.x)) % self.p
    }
}

pub struct Checker {
    x: i64,
    p: i64,
    g: i64,
    q: Option<i64>,
    e: Option<i64>,
}

impl Checker {
    pub fn new(x: i64, p: i64, g: i64) -> Self {
        Self {
            x,
            p,
            g,
            q: None,
            e: None,
        }
    }

    fn q(&self) -> i64 {
        match self.q {
            Some(i) => i,
            None => {
                let mut q: i64 = self.g;
                while q != 1 {
                    q = q * self.g % self.p;
                }
                q
            }
        }
    }

    pub fn e(&self) -> i64 {
        match self.e {
            Some(i) => i,
            None => {
                let mut rng = rand::thread_rng();
                let e: i64 = rng.gen_range(0..=self.q() - 1);
                e
            }
        }
    }

    pub fn is_correct(&self, y: i64, c: i64, z: i64) -> bool {
        let left = 1;
        for i in 1..z {
            let left = left * self.g % self.p;
        }
        let right = c;
        for i in 1..self.e() {
            let right = right * y % self.p;
        }
        left == right
    }
}
