use std::sync::{Arc, Mutex};

type StorageInner = Arc<Mutex<Vec<bool>>>;

pub struct IdStorage(StorageInner);

impl IdStorage {
    pub fn new() -> IdStorage {
        IdStorage(Arc::new(Mutex::new(Vec::new())))
    }

    pub fn gen(&mut self) -> IdBorrow {
        let mut lock = self.0.lock().unwrap();
        let mut k = 0;
        while k < lock.len() && lock[k] {
            k += 1;
        }

        if k == lock.len() {
            lock.push(true);
        } else {
            lock[k] = true;
        }

        IdBorrow {
            storage: self.0.clone(),
            id: k,
        }
    }
}

pub struct IdBorrow {
    storage: StorageInner,
    id: usize,
}

impl IdBorrow {
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Drop for IdBorrow {
    fn drop(&mut self) {
        let mut lock = self.storage.lock().unwrap();
        lock[self.id] = false;
    }
}
