use rand::{thread_rng, Rng};

pub fn event(percent: u32) -> bool {
    // A event is a number between 0..100. Then will be as many
    // random number being drawn as specified in the percent number.
    // If any of these guesses has been made before it will be ignored.
    // and the guess is repeated. If a guess matches the event n, then 
    // the event has been guessed and true is returned.
    let mut event_happend: bool = false;
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..100);
    let mut rounds: Vec<u32> = Vec::new();
    
    for _i in 0..percent {
        let mut round = rng.gen_range(0..100);
        while rounds.contains(&round) {
            round = rng.gen_range(0..100);
        }
        rounds.push(round);
        if round == n {
            event_happend = true;
            break;
        }
    }
    
    event_happend
}