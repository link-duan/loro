use ctor::ctor;

use loro_core::container::registry::{ContainerWrapper, LockContainer};
use loro_core::container::Container;
use loro_core::{InsertValue, LoroCore};

#[test]
fn map() {
    let mut loro = LoroCore::new(Default::default(), Some(10));
    let mut root = loro.get_map("root");
    root.insert(&loro, "haha".into(), InsertValue::Double(1.2));
    let value = root.get_value();
    assert_eq!(value.as_map().unwrap().len(), 1);
    assert_eq!(
        *value
            .as_map()
            .unwrap()
            .get("haha")
            .unwrap()
            .as_double()
            .unwrap(),
        1.2
    );
    let map_id = root.insert_obj(&loro, "map".into(), loro_core::ContainerType::Map);
    drop(root);
    let arc = loro.get_container(&map_id).unwrap();
    let mut sub_map = arc.lock_map();
    sub_map.insert(&loro, "sub".into(), InsertValue::Bool(false));
    drop(sub_map);
    let root = loro.get_map("root");
    let value = root.get_value();
    assert_eq!(value.as_map().unwrap().len(), 2);
    let map = value.as_map().unwrap();
    assert_eq!(*map.get("haha").unwrap().as_double().unwrap(), 1.2);
    assert!(map.get("map").unwrap().as_unresolved().is_some());
}

#[test]
fn two_client_text_sync() {
    let mut store = LoroCore::new(Default::default(), Some(10));
    let mut text_container = store.get_text("haha");
    text_container.insert(&store, 0, "012");
    text_container.insert(&store, 1, "34");
    text_container.insert(&store, 1, "56");
    let value = text_container.get_value();
    let value = value.as_string().unwrap();
    assert_eq!(&**value, "0563412");
    drop(text_container);

    let mut store_b = LoroCore::new(Default::default(), Some(11));
    let exported = store.export(Default::default());
    store_b.import(exported);
    let mut text_container = store_b.get_text("haha");
    text_container.with_container(|x| x.check());
    let value = text_container.get_value();
    let value = value.as_string().unwrap();
    assert_eq!(&**value, "0563412");

    text_container.delete(&store_b, 0, 2);
    text_container.insert(&store_b, 4, "789");
    let value = text_container.get_value();
    let value = value.as_string().unwrap();
    assert_eq!(&**value, "63417892");
    drop(text_container);

    store.import(store_b.export(store.vv()));
    let mut text_container = store.get_text("haha");
    let value = text_container.get_value();
    let value = value.as_string().unwrap();
    assert_eq!(&**value, "63417892");
    text_container.delete(&store, 0, 8);
    text_container.insert(&store, 0, "abc");
    let value = text_container.get_value();
    let value = value.as_string().unwrap();
    assert_eq!(&**value, "abc");

    store_b.import(store.export(Default::default()));
    let text_container = store_b.get_text("haha");
    text_container.with_container(|x| x.check());
    let value = text_container.get_value();
    let value = value.as_string().unwrap();
    assert_eq!(&**value, "abc");
}

#[ctor]
fn init_color_backtrace() {
    color_backtrace::install();
}
