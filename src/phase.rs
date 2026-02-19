use tokio::time::{self, Instant, Duration};

// rust mods go
// private enum | struct
// traits with above as defaults and interface for below
// parameter struct impl for init
// fn impl for methods

const nat:f64 = 6.0;

enum state {
    A,
    B,
    C,
    D,
    E,
    F,
}

pub struct Phaser {
    pub frequency_root: f64,
    pub frequency_target: f64,
    pub frequency_actual: f64,
    pub frequency_operating: f64,
    sinceBegin: Instant,
    sinceLast: Instant,
    sincePrev: Instant,
    pub current: f64,
    state: state,
    phase_offset: f64,
}

impl Phaser{
    pub fn new(fq: f64) -> Self {
        Self {
            frequency_root: fq,
            frequency_target: fq,
            frequency_actual: fq,
            frequency_operating: 0.0,
            sinceBegin: Instant::now(),
            sinceLast: Instant::now(),
            sincePrev: Instant::now(),
            current: 0.0,
            state: state::A,
            phase_offset: 0.0,
        }
    }
    pub fn incr_frequency(&mut self){
        self.frequency_target *= 3.0;
    }
    pub fn decr_frequency(&mut self){
        self.frequency_target *= 1.0 / 3.0;
    }
    pub fn offset(&mut self, n: f64){
        let p = n * 1.0/6.0;
        self.phase_offset = p;
    }
    pub fn tick(&mut self) -> bool {

        // lo-fi time for network hamornics to ripple
        let mut t = self.sinceBegin.elapsed().as_millis() as f64;

        let last = self.sinceLast.elapsed().as_nanos() as f64;
        self.frequency_operating = 1000000000.0 / last;        
        self.sinceLast = Instant::now();

        // 3 phase switch at 6 hertz automatically
        // dividing by nat=6 returns desired frequency
        // should be checking that frequency_actual is double or more freq-root
        t = t * self.frequency_root / nat;
        
        // time to seconds
        t = t / 1000.0;

        // add phase offset from network phase protocol
        t = t + self.phase_offset;

        // get three phases of time offset 120 degrees
        let q1: f64 = sq(t);
        let q2: f64 = sq(t + 1.0 / 3.0);
        let q3: f64 = sq(t + 2.0 / 3.0); 
        
        // the alternating switch (current) @ frequency f
        let c:f64 = q1 + q2 + q3;

        let b:usize = (c == self.current).into();
        let a = [tru, fals]; 
        // test frequency of alternating current
        if c == self.current { a[b]() } else {
            self.frequency_actual = 1000000000.0 / self.sincePrev.elapsed().as_nanos() as f64;
            println!("actual switch frequency: {}", self.frequency_actual);
            println!("operating frequency: {}", self.frequency_operating);
            println!("the three phases: {:?}", [q1, q2, q3]);
            println!("the current: {}", c);
            //println!("the state: {}", c);
            self.sincePrev = Instant::now();
            self.current = c;
            true
        }


        //println!("three phasers going: {:?}", [q1, q2, q3]);
        //println!("alternating current: {}", q1+ q2+ q3);
        
        

    }
    pub fn nano_tick(&mut self){
    }
}

fn tru() -> bool { true }
fn fals() -> bool { false }
fn dish<F: FnOnce() -> &'static dyn Fn()-> bool>(func: F) -> impl Fn() -> bool{
    func()
}


// noif square wave function
#[inline]
fn sq(n: f64) -> f64 {
    const sqar:[f64; 2] = [0.0, 1.0];
    let x = n % 1.0;
    let b = x < 0.5;
    let i:usize = b.into();
    sqar[i]
}
