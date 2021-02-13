use rand::{Rng, prelude::SliceRandom, thread_rng};

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
    
    for i in 0..percent {
        rounds.push(i);
    }
    rounds.shuffle(&mut rng);

    for i in rounds {
        if i == n {
            event_happend = true;
            break;
        }
    }
    
    event_happend
}

#[cfg(test)]
mod tests {
    
    use super::*;
    struct TestCase {
        percent: u32,
        min: f32,
        max: f32
    }
    //
    #[test]
    fn test_ranges() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{percent:0, min:0.0, max:0.0},
            TestCase{percent:10, min:0.075, max:0.125},
            TestCase{percent:20, min:0.175, max:0.225},
            TestCase{percent:30, min:0.275, max:0.325},
            TestCase{percent:40, min:0.375, max:0.425},
            TestCase{percent:50, min:0.475, max:0.525},
            TestCase{percent:60, min:0.575, max:0.625},
            TestCase{percent:70, min:0.675, max:0.725},
            TestCase{percent:80, min:0.775, max:0.825},
            TestCase{percent:90, min:0.875, max:0.925},
            TestCase{percent:100, min:0.975, max:100.25},        
        ];
    
        for test_case in test_cases {
            let ratio = iterate(test_case.percent);
            assert_eq!(
                (ratio >= test_case.min && ratio <= test_case.max), 
                true, 
                "Percent {}, ratio min {}, ratio max {}", 
                test_case.percent, 
                test_case.min, 
                test_case.max);
        }
    }
    

    fn iterate(percent: u32) -> f32 {
        let mut got_event: u32 = 0;
        for _i in 0..10000 {
            if event(percent) {
                got_event = got_event + 1;
            } 
        }
        got_event as f32 / 10000.0
    }
}
