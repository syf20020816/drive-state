use std::fmt::{Display, Formatter};
use std::f32::consts::PI;
use serde::{Serialize, Deserialize};


// 状态间隔
const SPACING: f32 = 1.667_f32;
// 车载行车记录仪视角角度
const VIEW_ANGLE_LOWEST: f32 = 120.0;
const VIEW_ANGLE_HIGHEST: f32 = 170.0;
const VIEW_ANGLE_AVG: f32 = 150.0;
// 车辆碰撞有效距离占比
const CAR_H_LOWEST: f32 = 0.12;
const CAR_H_AVG: f32 = 0.135;
const CAR_H_HIGHEST: f32 = 0.15;

#[derive(Serialize, Deserialize)]
pub struct LowerState {
    id: u32,
    status: LowerStatus,
    action: Option<Action>,
    time: f32,
    //被标注物
    sign: Kind,
}

impl LowerState {
    /// resolution_rate : 图片分辨率(行车记录仪分辨率)
    /// space_time : 间隔次数（决定是第几个状态）
    /// pos:标注物坐标点
    /// kind:识别类型
    pub fn new(resolution_rate_h: f32, resolution_rate_w: f32, pos: Vec<f32>, space_time: u32, kind: u32) ->Self{
        // let effective_h = (pos[3] - pos[0]) / 2.0;
        let distance = inner_deal(resolution_rate_h, resolution_rate_w, pos);

        let sign = Kind::from_i32(kind as i32);
        let action = Action::from(sign.clone());
        let mut status = LowerStatus::Normal;
        if distance < 0.5 {
            status = LowerStatus::from(action.clone())
        }

        LowerState {
            id: space_time,
            status,
            action:Some(action),
            time: SPACING * space_time as f32,
            sign,
        }
    }
    pub fn default(space_time: u32, kind: u32)->Self{
        let sign = Kind::from_i32(kind as i32);
        LowerState {
            id: space_time,
            status:LowerStatus::Normal,
            action:None,
            time: SPACING * space_time as f32,
            sign,
        }
    }
}

impl Display for LowerState{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",serde_json::to_string(self).unwrap())
    }
}


/// 状态计算
/// resolution_rate : 图片分辨率(行车记录仪分辨率)
/// h:标注物高度 -
/// w:标注物宽度 -
/// pos:标注物坐标点
/// 要通过行车记录仪的分辨率计算真实距离，首先需要了解行车记录仪的视角（FOV）和传感器尺寸。行车记录仪的视角通常在120度到170度之间，传感器尺寸通常为1/2.7英寸或1/3英寸。
/// 在以1080p为例的情况下，设行车记录仪的水平像素数为1920个像素，垂直像素数为1080个像素。我们可以利用三角函数计算真实距离。
/// 首先，我们需要计算每个像素的角度值。根据行车记录仪的视角，可以将水平视角转换为垂直视角，然后计算每个像素对应的水平和垂直角度。
/// 以水平视角为例，假设视角为150度，则每个像素对应的水平角度为150度 / 1920个像素 = 0.078125度/像素。
/// 接下来，我们可以使用三角函数来计算真实距离。假设车辆与行车记录仪之间的水平距离为D，行车记录仪距离地面的高度为H。
/// 以图片上一个物体的像素高度为h像素，则该物体到行车记录仪的距离（d）可以通过以下公式计算：
/// d = (H * h) / (tan(视角/2) * 分辨率垂直像素数)
/// 以1080p为例，假设行车记录仪距离地面的高度为1.5米，视角为150度，物体在图像中的像素高度为200像素，则可以计算出距离d：
/// d = (1.5 * 200) / (tan(150/2) * 1080) ≈ 5.52米
fn inner_deal(resolution_rate_h: f32, resolution_rate_w: f32, pos: Vec<f32>) -> f32 {
    // 标注物于车辆有效碰撞距离之间的差值
    //通过占比计算车辆有效碰撞距离
    let c = ((VIEW_ANGLE_AVG / 2.0) * PI / 180.0).tan() * 1080.0;
    let effective_dis_avg = CAR_H_AVG * resolution_rate_h * 1.5 / c;
    let dis = (resolution_rate_h - pos[3]) * 1.5 / c;
    dis - effective_dis_avg
}

#[derive(Clone,Serialize, Deserialize)]
pub enum LowerStatus {
    Normal,
    Collision,
    PedestrianCollision,
    Other(String),
}

impl From<Action> for LowerStatus {
    fn from(value: Action) -> Self {
        match value {
            Action::DiscoverPerson => LowerStatus::PedestrianCollision,
            Action::DiscoverCar => LowerStatus::Collision,
            Action::Other(s) => LowerStatus::Other(s)
        }
    }
}

#[derive(Clone,Serialize, Deserialize)]
pub enum Action {
    DiscoverPerson,
    DiscoverCar,
    Other(String),
}

impl From<Kind> for Action {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Person => Action::DiscoverPerson,
            Kind::Car | Kind::Truck | Kind::Bus => Action::DiscoverCar,
            _ => Action::Other(value.to_string())
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let action = match self {
            Action::DiscoverPerson => "discover person".to_string(),
            Action::DiscoverCar => "discover car".to_string(),
            Action::Other(s) => format!("discover {}", s),
        };
        write!(f,"{}", action)
    }
}

#[derive(Clone,Serialize, Deserialize)]
pub enum Kind {
    Person = 0,
    Bicycle = 1,
    Car = 2,
    Motorcycle = 3,
    Airplane = 4,
    Bus = 5,
    Truck = 7,
    Other,
}

impl Kind {
    pub fn from_i32(k: i32) -> Self {
        match k {
            0 => Kind::Person,
            1 => Kind::Bicycle,
            2 => Kind::Car,
            3 => Kind::Motorcycle,
            4 => Kind::Airplane,
            5 => Kind::Bus,
            7 => Kind::Truck,
            _ => Kind::Other
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Kind::Person => "person".to_string(),
            Kind::Bicycle => "bike".to_string(),
            Kind::Car => "car".to_string(),
            Kind::Motorcycle => "motorcycle".to_string(),
            Kind::Airplane => "airplane".to_string(),
            Kind::Bus => "bus".to_string(),
            Kind::Truck => "truck".to_string(),
            Kind::Other => "other".to_string()
        };
        write!(f,"{}", kind)
    }
}