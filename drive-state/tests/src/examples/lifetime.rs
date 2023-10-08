use drive_state::NormalDriving;
use drive_state::LifeTime;

fn main() {
    let mut state1 = NormalDriving::new();
    let _ = state1.before(|| println!("this is before!"));
    let _ = state1.destroy(|| println!("this is destroy!"));
    let mut state2 = NormalDriving::new();
    let _ = state2.before(|| { println!("another state before!") });
    let _ = state2.destroy(|| println!("another destroy"));
    let mut res = state1.transfer(&mut state2);
    res.on(|| { println!("on!!!") });
    res.call_on();
    println!("1");
    println!("2");
    // assert_eq!(*res, state2);
}
