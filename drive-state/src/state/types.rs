// use crate::LifeTime;
// use crate::NormalDriving;

/// # Car Accident State
/// the state of the car accudent
/// ## example
/// ```code
/// use drive_state::types::State;
/// let state = State::default();
/// assert_eq!(state, State::new(0));
/// assert_eq!(State::Blast, State::from_i32(6));
/// assert_eq!(8, State::ReachingTail.as_i32());
/// ```
/// ### enums
/// - NormalDriving : 正常行驶
/// - SuddenBraking : 突然刹车
/// - Collision : 碰撞
/// - Rollover : 侧翻
/// - RunAway : 失控
/// - CatchFire : 起火
/// - Blast : 爆炸
/// - Crash : 坠毁
/// - ReachingTail : 追尾
/// - PedestrianCollision : 行人被撞
#[derive(Eq, PartialEq, Debug)]
pub enum State {
    NormalDriving = 0,
    SuddenBraking = 1,
    Collision = 2,
    Rollover = 3,
    RunAway = 4,
    CatchFire = 5,
    Blast = 6,
    Crash = 7,
    ReachingTail = 8,
    PedestrianCollision = 9,
    Other = 10,
}

impl State {
    /// new a State enum with i32
    pub fn new(id: i32) -> Self {
        State::from_i32(id)
    }

    /// convert from State to i32 it will consume the State
    pub fn as_i32(self) -> i32 {
        self as i32
    }
    /// convert from i32 to State
    pub fn from_i32(value: i32) -> Self {
        State::from(value)
    }
    // pub fn as_event(self) -> Option<Box<dyn LifeTime>> {
    //     let event = match self {
    //         State::NormalDriving => Box::new(NormalDriving),
    //         State::SuddenBraking => {}
    //         State::Collision => {}
    //         State::Rollover => {}
    //         State::RunAway => {}
    //         State::CatchFire => {}
    //         State::Blast => {}
    //         State::Crash => {}
    //         State::ReachingTail => {}
    //         State::PedestrianCollision => {}
    //         State::Other => {}
    //     };
    //     Some(event)
    // }
}

impl From<State> for i32 {
    fn from(value: State) -> Self {
        value.as_i32()
    }
}

impl From<i32> for State {
    fn from(value: i32) -> Self {
        match value {
            0 => State::NormalDriving,
            1 => State::SuddenBraking,
            2 => State::Collision,
            3 => State::Rollover,
            4 => State::RunAway,
            5 => State::CatchFire,
            6 => State::Blast,
            7 => State::Crash,
            8 => State::ReachingTail,
            9 => State::PedestrianCollision,
            _ => State::Other,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::NormalDriving
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let state = State::default();
        assert_eq!(state, State::new(0));
        assert_eq!(State::Blast, State::from_i32(6));
        assert_eq!(8, State::ReachingTail.as_i32());
    }
}
