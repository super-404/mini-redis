use crate::data::log_record::DataRecord;
use std::collections::BTreeMap;
use parking_lot::RwLock;
use std::sync::Arc;
#[derive(Default)]
pub struct BTree {
    tree: Arc<RwLock<BTreeMap<Vec<u8>, DataRecord>>>,
    // item:DataRecord
}
pub trait Indexer {
    fn get(&self, key: Vec<u8>) -> Option<DataRecord>;
    fn put(&self, key: Vec<u8>, item: DataRecord) -> bool;
    fn delete(&self, key: Vec<u8>) -> bool;
}
impl BTree {
    pub fn new() -> BTree {
        BTree {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl Indexer for BTree {
    fn put(&self, key: Vec<u8>, item: DataRecord) -> bool {
        //获取写锁,并写数据，注意作用域
        {
            let mut w_lock = self.tree.write();
            w_lock.insert(key, item);
        } 
        //写锁释放
        //matches!(self.tree.insert(key, item), Some(_))
        true
    }
    fn get(&self, key: Vec<u8>) -> Option<DataRecord> {
        //获取读锁
        {
            let r_lock = self.tree.read();
            r_lock.get(&key).copied()
        }
    }
    fn delete(& self, key: Vec<u8>) -> bool {
        //获取写锁,并写数据，注意作用域
        {
            let mut w_lock = self.tree.write();
            w_lock.remove(&key);
        }
        true
        //matches!(self.tree.remove(&key), Some(_))
    }
}
