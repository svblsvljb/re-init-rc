use std::{
    rc::{Rc, Weak as RcWeak},
    sync::{Arc, Weak as ArcWeak},
};

#[derive(Debug, Clone)]
pub struct ReInitArc<T, F> {
    init_fn: F,
    weak_ptr: ArcWeak<T>,
}

impl<F, T> ReInitArc<T, F>
where
    F: Fn() -> T,
{
    pub fn new(init_fn: F) -> Self {
        Self {
            init_fn,
            weak_ptr: ArcWeak::new(),
        }
    }

    pub fn get(&mut self) -> Arc<T> {
        self.weak_ptr.upgrade().unwrap_or_else(|| {
            let arc = Arc::new((self.init_fn)());
            self.weak_ptr = Arc::downgrade(&arc);
            arc
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReInitRc<T, F> {
    init_fn: F,
    weak_ptr: RcWeak<T>,
}

impl<F, T> ReInitRc<T, F>
where
    F: Fn() -> T,
{
    pub fn new(init_fn: F) -> Self {
        Self {
            init_fn,
            weak_ptr: RcWeak::new(),
        }
    }

    pub fn get(&mut self) -> Rc<T> {
        self.weak_ptr.upgrade().unwrap_or_else(|| {
            let rc = Rc::new((self.init_fn)());
            self.weak_ptr = Rc::downgrade(&rc);
            rc
        })
    }
}
