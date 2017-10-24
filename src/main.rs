extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::collections::VecDeque;

fn rand_char<R: Rng>(range: &Range<u8>, rng: &mut R) -> u8 {
    range.ind_sample(rng) + ('a' as u8)
}

fn is_heart(buf: &VecDeque<u8>) -> bool {
    buf[0] == ('h' as u8) &&
        buf[1] == ('e' as u8) &&
        buf[2] == ('a' as u8) &&
        buf[3] == ('r' as u8) &&
        buf[4] == ('t' as u8)
}

fn is_earth(buf: &VecDeque<u8>) -> bool {
    buf[0] == ('e' as u8) &&
        buf[1] == ('a' as u8) &&
        buf[2] == ('r' as u8) &&
        buf[3] == ('t' as u8) &&
        buf[4] == ('h' as u8)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Result {
    Heart,
    Earth,
    Timeout
}

fn check<R: Rng>(range: &Range<u8>, rng: &mut R, buf: &mut VecDeque<u8>, maxtime: usize) -> Result {
    buf.clear();
    for _ in 0..5 {
        buf.push_back(rand_char(range, rng));
    }

    for _ in 0..maxtime {
        if is_heart(buf) {
            return Result::Heart;
        } else if is_earth(buf) {
            return Result::Earth;
        }
        buf.push_back(rand_char(range, rng));
        buf.pop_front();
    }
    Result::Timeout
}

fn main() {
    let range = Range::new(0, 26);
    let mut rng = rand::thread_rng();
    let mut buf = VecDeque::new();

    let mut heart_count = 0 as usize;
    let mut earth_count = 0 as usize;

    for _ in 0..1000000 {
        match check(&range, &mut rng, &mut buf, 10000) {
            Result::Heart => heart_count += 1,
            Result::Earth => earth_count += 1,
            _ => {},
        }
    }

    println!("heart: {}\nearth: {}", heart_count, earth_count);
}
