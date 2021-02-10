use rand::{thread_rng, Rng};

pub fn event(percent: u32) -> bool {
    let mut event_happend: bool = false;
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..100);
    //println!("n is {}", n);
    let mut rounds: Vec<u32> = Vec::new();
    let mut count: u32 = 0;
    for _i in 0..percent {
        count = count + 1;
        let mut round = rng.gen_range(0..100);
        while rounds.contains(&round) {
            round = rng.gen_range(0..100);
            count = count + 1;
        }
        rounds.push(round);
        //println!("{}", round);
        if round == n {
            event_happend = true;
            break;
        }
    }
    //println!("Drawed {} times...", count);
    event_happend
}