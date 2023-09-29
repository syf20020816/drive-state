use crate::{LifeTime, State, LifeTimeContainer};
use crate::error::TranslationError;

pub struct NormalDriving {
    lifetime: LifeTimeContainer,
}

impl Default for NormalDriving {
    fn default() -> Self {
        NormalDriving {
            lifetime: LifeTimeContainer::default()
        }
    }
}


impl LifeTime for NormalDriving {
    fn new() -> Self {
        Self::default()
    }

    fn before<F>(&mut self, f: F) -> () where F: FnOnce() + 'static {
        self.lifetime.before(f);
    }

    fn call_before(&mut self) -> () {
        self.lifetime.run_before();
    }

    fn on<F>(&mut self, f: F) -> () where F: FnOnce() + 'static {
        self.lifetime.on(f);
    }

    fn call_on(&mut self) -> () {
        self.lifetime.run_on();
    }

    fn transfer<T>(self, to: &mut T) -> Result<Box<&mut T>, TranslationError> where T: LifeTime {
        // to call_before()
        to.call_before();
        drop(self);
        Ok(Box::new(to))
    }

    fn destroy<F>(&mut self, f: F) where F: FnOnce() + 'static {
        self.lifetime.destroy(f);
    }

    fn call_destroy(&mut self) {
        self.lifetime.run_destroy();
    }
}

impl Drop for NormalDriving {
    fn drop(&mut self) {
        self.call_destroy();
    }
}
