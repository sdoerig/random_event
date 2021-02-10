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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_will_never_occur() {
        assert_eq!(event(0), false);
    }
    #[test]
    fn test_event_will_always_occur() {
        assert_eq!(event(10000), true);
    }
    #[test]
    fn test_event_10_percent() {
        let ratio = iterate(10);
        assert_eq!((ratio > 0.05 && ratio < 0.15), true);
    }
    #[test]
    fn test_event_20_percent() {
        let ratio = iterate(20);
        assert_eq!((ratio > 0.15 && ratio < 0.25), true);
    }


    #[test]
    fn test_event_30_percent() {
        let ratio = iterate(30);
        assert_eq!((ratio > 0.25 && ratio < 0.35), true);
    }

    #[test]
    fn test_event_40_percent() {
        let ratio = iterate(40);
        assert_eq!((ratio > 0.35 && ratio < 0.45), true);
    }

    #[test]
    fn test_event_50_percent() {
        let ratio = iterate(50);
        assert_eq!((ratio > 0.45 && ratio < 0.55), true);
    }
    #[test]
    fn test_event_60_percent() {
        let ratio = iterate(60);
        assert_eq!((ratio > 0.55 && ratio < 0.65), true);
    }
    #[test]
    fn test_event_70_percent() {
        let ratio = iterate(70);
        assert_eq!((ratio > 0.65 && ratio < 0.75), true);
    }

    #[test]
    fn test_event_80_percent() {
        let ratio = iterate(80);
        assert_eq!((ratio > 0.75 && ratio < 0.85), true);
    }
    #[test]
    fn test_event_90_percent() {
        let ratio = iterate(90);
        assert_eq!((ratio > 0.85 && ratio < 0.95), true);
    }
    #[test]
    fn test_event_100_percent() {
        let ratio = iterate(100);
        assert_eq!((ratio > 0.95 && ratio < 1.05), true);
    }

    fn iterate(percent: u32) -> f32 {
        let mut got_event: u32 = 0;
        for _i in 0..10000 {
            let event_happend = event(percent);
            if event_happend {
                got_event = got_event + 1;
            } 
        }
        got_event as f32 / 10000.0
    }


}
