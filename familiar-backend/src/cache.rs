use std::collections::HashMap;

use fre::common::meta::HasItemMeta;
use uuid::Uuid;


pub struct Cache<T : HasItemMeta> {
    internal: HashMap<Uuid, T>,
    count: HashMap<Uuid, usize>,
}

impl <T : HasItemMeta> Default for Cache<T> {
    fn default() -> Self {
        Self { internal: Default::default(), count: Default::default() }
    }
}

impl <T : HasItemMeta> Cache<T> {
    
    pub fn cache(&mut self, item : T) {
        let uuid = item.get_meta().uuid();
        println!("Adding `{uuid:?}` to cache.");
        self.internal.entry(uuid.clone()).or_insert(item);
        self.increment_count(&uuid);
    }
    
    pub fn increment_count(&mut self, uuid: &Uuid) {
        *(self.count.entry(uuid.clone()).or_insert(0)) += 1 
    }
    
    pub fn decrement_count(&mut self, uuid: &Uuid) {
        let Some(count) = self.count.get_mut(uuid) else { return };
        *count = count.saturating_sub(1);
        if *count == 0 {
            println!("Dropping `{uuid:?}` from cache.");
            self.count.remove(uuid);
            self.internal.remove(uuid);
        }
    }
    
    pub fn get(&self, uuid : &Uuid) -> Option<&T> {
        self.internal.get(uuid)
    }
}