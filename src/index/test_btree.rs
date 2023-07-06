
//use std::fmt::rt::v1::FormatSpec;
use std::sync::Arc;
use std::time::Duration;
use crate::index::btree::BTree;
use crate::data::log_record::DataRecord;
use super::btree::Indexer;
 #[test]
fn test_new(){
    let mut b = BTree::new();

    let v_u8:Vec<u8> = "abc".as_bytes().to_vec();

    let flag = b.put(v_u8, DataRecord{file_id:3,offset:123});
    
    // assert_eq!(b.put(v_u8, DataRecord{file_id:3,offset:123}),true)
}
#[test]
fn test_get(){
    let mut b = BTree::new();
    let v_u8:Vec<u8> = "abc".as_bytes().to_vec();
    let flag = b.put(v_u8.clone(), DataRecord{file_id:3,offset:123});
    let elem = b.get(v_u8.clone());
    println!("elem:{:?}",elem);
    // assert_eq!(b.put(v_u8, DataRecord{file_id:3,offset:123}),true)
}
#[test]
fn test_delete(){
    let b = BTree::new();
    let v_u8:Vec<u8> = "abc".as_bytes().to_vec();
    let flag = b.put(v_u8.clone(), DataRecord{file_id:3,offset:123});
    let elem = b.delete(v_u8.clone());
    let ans = b.get(v_u8.clone());
    println!("elem:{:?}",elem);
    println!("elem:{:?}",ans);
    // assert_eq!(b.put(v_u8, DataRecord{file_id:3,offset:123}),true)
}
#[test]
fn test_btreeMap(){
    use std::collections::BTreeMap;
    let mut b= BTreeMap::new();
    let r = b.insert(1,5);
    let flag =matches!(r,Some(_));
    println!("{:?}",flag);
    assert!(matches!(r, None));
    
}
#[test]
fn test_matches(){
   let a = Some(5);
   assert!(matches!(a,Some(_))); 
}
#[test]
fn test_rwlock(){
    use parking_lot::RwLock;
    use std::thread;
    let v = Arc::new(RwLock::new(vec!["a".to_string()]));
    for i in 0..10{
        let v_c = v.clone();
        thread::spawn(move ||{
            thread::sleep(Duration::from_secs(5));
            {
                let mut w_lock = v_c.write();
                
                w_lock.push(format!("第{i}个线程"));
             
            }
            let r_lock =  v_c.read();
            println!("第{i}个线程----------{:?}",r_lock);
            //我的读写锁同时存在为什么没有发生panic?
        });
    }
    thread::sleep(Duration::from_secs(10));
    // return ();
}
#[test]
fn test_copy_clone(){
    // #[derive(Clone, Copy)]
    // struct t {
    //     a:i32,
    //     b:usize
    // }
    // use crate::data::logrecord::DataRecord;
    // let d = t{a:3,b:23};
    // let d_copy = d;
    // println!("{:?}",d.a);
}