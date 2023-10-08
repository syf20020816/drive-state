mod entity;
pub mod event;
pub mod action;


use std::mem;
use crate::state::State;
use crate::error::TranslationError;

/// # LifeTimeContainer
/// 存储生命周期的容器，每个状态都需要依赖它
/// 它会存储before,on,destroy生命周期的闭包，调用run_lifetime_name时激活调用
/// 这是一种延迟调用的典型方式，它存在于每一种状态中
/// 也许多数情况下使用者无需设置生命周期容器
pub struct LifeTimeContainer {
    before: Option<Box<dyn FnOnce()>>,
    on: Option<Box<dyn FnOnce()>>,
    destroy: Option<Box<dyn FnOnce()>>,
}


impl Default for LifeTimeContainer {
    fn default() -> Self {
        LifeTimeContainer {
            before: None,
            on: None,
            destroy: None,
        }
    }
}

impl LifeTimeContainer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn before<F>(&mut self, f: F) -> () where F: FnOnce() + 'static {
        self.before = Some(Box::new(f));
    }
    pub fn on<F>(&mut self, f: F) -> () where F: FnOnce() + 'static {
        self.on = Some(Box::new(f));
    }
    pub fn destroy<F>(&mut self, f: F) -> () where F: FnOnce() + 'static {
        self.destroy = Some(Box::new(f));
    }
    pub fn run_before(&mut self) -> () {
        if self.before.is_some() {
            let mut empty_closure: Box<dyn FnOnce()> = Box::new(|| {});
            // swap closure
            let _ = mem::swap(self.before.as_mut().unwrap(), &mut empty_closure);
            empty_closure();
        }
    }
    pub fn run_on(&mut self) -> () {
        if self.on.is_some() {
            let mut empty_closure: Box<dyn FnOnce()> = Box::new(|| {});
            let _ = mem::swap(self.on.as_mut().unwrap(), &mut empty_closure);
            empty_closure();
        }
    }
    pub fn run_destroy(&mut self) -> () {
        if self.destroy.is_some() {
            let mut empty_closure: Box<dyn FnOnce()> = Box::new(|| {});
            let _ = mem::swap(self.destroy.as_mut().unwrap(), &mut empty_closure);
            empty_closure();
        }
    }
}

/// 默认的状态转移生命周期
/// 若无需修改默认的生命周期执行工作，请设置传入闭包为空闭包，即: ||{}
/// before,on,destroy方法都属于延迟反应模式
/// 快速反应模式: 调用该方法时会立即触发传入的闭包函数，否则会存储到LiftTimeContainer中，直到使用call_fn_name时才会触发
/// call_fn_name无需使用者手动调用，程序会自动进行调用处理
pub trait LifeTime {
    fn new() -> Self;
    /// 进入某种状态前，状态将被创建
    fn before<F>(&mut self, f: F) -> () where F: FnOnce() + 'static;
    fn call_before(&mut self) -> ();
    /// 正式进入某种状态时
    fn on<F>(&mut self, f: F) -> () where F: FnOnce() + 'static;
    fn call_on(&mut self) -> ();
    /// 状态转移，从一种状态转移到另一种状态
    /// 当发生状态转移时会自动调用上一个状态的销毁和下一个状态的预进入
    /// 同时他会消费掉自己
    /// 实际上它并不会出现TranslationError
    fn transfer<T>(self, to: &mut T) -> Box<&mut T> where T: LifeTime;
    /// 跳出当前状态，当前状态被销毁
    fn destroy<F>(&mut self, f: F) where F: FnOnce() + 'static;
    fn call_destroy(&mut self);
}