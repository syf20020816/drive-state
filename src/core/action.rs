//! # Actions
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/9/30
//! @version:0.0.1
//! @description:
//! ```

/// # Action
/// 自动驾驶遇到状态发生变化时的行为
/// 行为会导致状态的变化（状态的变化需要行为辅助）
pub enum Action {
    // 该事件表示不做任何特殊的事情，维持当前事件，例如:加速->加速
    Continue,
    // 加速
    Accelerate,
    // 减速
    Moderate,
    // 停
    Stop,
    // 转弯 左
    TurningLeft,
    // 转弯 右
    TurningRight,
    Other,
}