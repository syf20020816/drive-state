//! # core car condition
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/9/29
//! @version:0.0.1
//! @description:
//! ```

use crate::state::State;



pub struct Condition {
    // 行驶状态 , 状态可能是同时存在多个的
    states: Vec<State>,
    // 状态优先级字典 , 表示每个状态和事件的重要性，以便在处理多个事件时确定执行顺序
    // priority: Priority,
    // 推荐，推荐指的是下一步如何做才能规避意外(只会在意外前设置)
    // recommend: Recommend,
}
