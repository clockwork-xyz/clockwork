use dioxus::prelude::{use_ref, ScopeState, UseRef};
use std::ops::Rem;
use uuid::Uuid;

pub fn use_pagination<T: Clone + 'static>(
    cx: &ScopeState,
    page_size: usize,
    initial_state_fn: impl FnOnce() -> Vec<T>,
) -> &UsePagination<T> {
    let data = use_ref(cx, || Pagination {
        data: initial_state_fn(),
        uuid: Uuid::new_v4(),
        page_size: page_size.clone(),
        current_page: 0,
        total_pages: 0,
    });

    cx.use_hook(|| UsePagination { data: data.clone() })
}

pub struct UsePagination<T: Clone + 'static> {
    pub data: UseRef<Pagination<T>>,
}

pub struct Pagination<T: Clone + 'static> {
    pub data: Vec<T>,
    uuid: Uuid,
    pub page_size: usize,
    pub current_page: usize,
    pub total_pages: usize,
}

impl<T: Clone + 'static> UsePagination<T> {
    pub fn set(&self, new: Vec<T>) {
        let mut data_write = self.data.write();
        let page_size = data_write.page_size;
        let len = new.len();
        data_write.data = new;
        if len > 0 {
            let tp = len / page_size;
            data_write.total_pages = tp;
            if len.rem(page_size) == 0 {
                data_write.total_pages = tp;
            }
        }
    }

    pub fn get(&self) -> Option<Vec<T>> {
        let data_read = self.data.read();
        if data_read.data.len() > 0 {
            let start = (data_read.current_page * data_read.page_size) as usize;
            let end = start + data_read.page_size as usize;
            if data_read.current_page == data_read.total_pages {
                return Some(data_read.data[start..].to_vec());
            }
            return Some(data_read.data[start..end].to_vec());
        }
        None
    }

    pub fn next_page(&self) {
        let data_read = self.data.read();
        if data_read.current_page < data_read.total_pages {
            drop(data_read);
            self.data.write().current_page += 1
        }
    }

    pub fn prev_page(&self) {
        let data_read = self.data.read();
        if data_read.current_page < data_read.total_pages {
            drop(data_read);
            self.data.write().current_page -= 1
        }
    }

    pub fn current_page(&self) -> usize {
        self.data.read().current_page + 1
    }

    pub fn total_pages(&self) -> usize {
        self.data.read().total_pages + 1
    }
}

impl<T: Clone + 'static + PartialEq> PartialEq for UsePagination<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.read().uuid == other.data.read().uuid
    }
}

impl<T: Clone + 'static> Clone for UsePagination<T> {
    fn clone(&self) -> Self {
        UsePagination {
            data: self.data.clone(),
        }
    }
}
