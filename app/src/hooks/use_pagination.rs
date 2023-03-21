use dioxus::prelude::{to_owned, ScopeState};
use std::{cell::RefCell, ops::Rem, rc::Rc, sync::Arc};

pub fn use_pagination<T: Clone + 'static>(
    cx: &ScopeState,
    page_size: usize,
    initial_state_fn: impl FnOnce() -> Vec<T>,
) -> &UsePagination<T> {
    let hook = cx.use_hook(move || {
        let data = Rc::new(initial_state_fn());
        let current_page = Rc::new(0);
        let total_pages = Rc::new(0);
        let data_slot = Rc::new(RefCell::new(data.clone()));
        let current_page_slot = Rc::new(RefCell::new(current_page.clone()));
        let total_pages_slot = Rc::new(RefCell::new(total_pages.clone()));
        let update_callback = cx.schedule_update();
        let data_setter = Rc::new({
            to_owned![update_callback, data_slot];
            move |new| {
                {
                    let mut data_slot = data_slot.borrow_mut();

                    if let Some(val) = Rc::get_mut(&mut data_slot) {
                        *val = new;
                    } else {
                        *data_slot = Rc::new(new);
                    }
                }
                update_callback();
            }
        });

        let current_page_setter = Rc::new({
            to_owned![update_callback, current_page_slot];
            move |new| {
                {
                    let mut current_page_slot = current_page_slot.borrow_mut();

                    if let Some(val) = Rc::get_mut(&mut current_page_slot) {
                        *val = new;
                    } else {
                        *current_page_slot = Rc::new(new)
                    }
                }
                update_callback();
            }
        });

        let total_pages_setter = Rc::new({
            to_owned![update_callback, total_pages_slot];
            move |new| {
                {
                    let mut total_pages_slot = total_pages_slot.borrow_mut();

                    if let Some(val) = Rc::get_mut(&mut total_pages_slot) {
                        *val = new;
                    } else {
                        *total_pages_slot = Rc::new(new)
                    }
                }
                update_callback();
            }
        });

        UsePagination {
            data,
            data_slot,
            data_setter,
            current_page,
            current_page_slot,
            current_page_setter,
            total_pages,
            total_pages_slot,
            total_pages_setter,
            update_callback,
            page_size,
        }
    });

    hook.data = hook.data_slot.borrow().clone();
    hook.current_page = hook.current_page_slot.borrow().clone();
    hook.total_pages = hook.total_pages_slot.borrow().clone();

    hook
}

pub struct UsePagination<T: Clone + 'static> {
    data: Rc<Vec<T>>,
    data_slot: Rc<RefCell<Rc<Vec<T>>>>,
    data_setter: Rc<dyn Fn(Vec<T>)>,
    current_page: Rc<usize>,
    current_page_slot: Rc<RefCell<Rc<usize>>>,
    current_page_setter: Rc<dyn Fn(usize)>,
    total_pages: Rc<usize>,
    total_pages_slot: Rc<RefCell<Rc<usize>>>,
    total_pages_setter: Rc<dyn Fn(usize)>,
    update_callback: Arc<dyn Fn()>,
    page_size: usize,
}

impl<T: Clone + 'static> UsePagination<T> {
    pub fn set(&self, new: Vec<T>) {
        let len = new.len();
        (self.data_setter)(new);
        if len > 0 {
            let tp = len / self.page_size;
            (self.total_pages_setter)(tp);
            if len.rem(self.page_size) == 0 {
                (self.total_pages_setter)(tp - 1)
            }
        }
    }

    pub fn get(&self) -> Option<&[T]> {
        if self.data.len() > 0 {
            let start = (self.current_page() * self.page_size) as usize;
            let end = start + self.page_size as usize;
            if self.current_page == self.total_pages {
                return Some(&self.data[start..]);
            }
            return Some(&self.data[start..end]);
        }
        None
    }

    pub fn next_page(&self) {
        if self.current_page < self.total_pages {
            (self.current_page_setter)(self.current_page() + 1)
        }
    }

    pub fn prev_page(&self) {
        if self.current_page() > &0 {
            (self.current_page_setter)(self.current_page() - 1)
        }
    }

    pub fn current_page(&self) -> &usize {
        &self.current_page
    }
}

impl<T: Clone + 'static> Clone for UsePagination<T> {
    fn clone(&self) -> Self {
        UsePagination {
            data: self.data.clone(),
            data_setter: self.data_setter.clone(),
            data_slot: self.data_slot.clone(),
            current_page: self.current_page.clone(),
            current_page_slot: self.current_page_slot.clone(),
            current_page_setter: self.current_page_setter.clone(),
            total_pages: self.total_pages.clone(),
            total_pages_slot: self.total_pages_slot.clone(),
            total_pages_setter: self.total_pages_setter.clone(),
            update_callback: self.update_callback.clone(),
            page_size: self.page_size.clone(),
        }
    }
}
