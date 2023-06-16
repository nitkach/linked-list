use super::*;

fn create_list() -> LinkedList {
    LinkedList::from_iter(vec![10, 20, 30])
}

#[test]
fn insert_into_empty_success() {
    let mut list = LinkedList::new();

    list.insert(0, 50);

    let expected = r#"LinkedList {
    head: Some(
        Node {
            next: None,
            elem: 50,
        },
    ),
    len: 1,
}"#;
    assert_eq!(format!("{list:#?}"), expected);
}

#[test]
fn insert_into_non_empty(){
    let mut list = LinkedList::new();

    list.insert(0, 50);
    list.insert(1, 100);

    let expected = r#"LinkedList {
    head: Some(
        Node {
            next: Some(
                Node {
                    next: None,
                    elem: 100,
                },
            ),
            elem: 50,
        },
    ),
    len: 2,
}"#;

    assert_eq!(format!("{list:#?}"), expected);
}

#[test]
fn push_into_empty_success() {
    let mut list = LinkedList::new();

    list.push(100);

    let expected = r#"LinkedList {
    head: Some(
        Node {
            next: None,
            elem: 100,
        },
    ),
    len: 1,
}"#;

    assert_eq!(format!("{list:#?}"), expected);
}

#[test]
fn push_into_non_empty_success() {
    let mut list = LinkedList::new();

    list.push(50);
    list.push(100);

    let expected = r#"LinkedList {
    head: Some(
        Node {
            next: Some(
                Node {
                    next: None,
                    elem: 100,
                },
            ),
            elem: 50,
        },
    ),
    len: 2,
}"#;
    assert_eq!(format!("{list:#?}"), expected)
}

#[test]
fn get_from_empty() {
    let list = LinkedList::new();

    assert_eq!(list.get(0), None);
    assert_eq!(list.get(1), None);
}

#[test]
fn get_from_non_empty() {
    let list = create_list();

    dbg!(&list);

    assert_eq!(list.get(0), Some(10));
    assert_eq!(list.get(1), Some(20));
    assert_eq!(list.get(2), Some(30));

    assert_eq!(list.get(3), None);
}

#[test]
#[should_panic]
fn remove_from_empty() {
    let mut list = LinkedList::new();

    list.remove(0);
}

#[test]
fn remove_from_head_non_empty() {
    let mut list = create_list();

    assert_eq!(list.remove(0), 10);
}

#[test]
fn remove_from_middle_non_empty() {
    let mut list = create_list();

    assert_eq!(list.remove(1), 20);
}

#[test]
fn remove_from_tail_non_empty() {
    let mut list = create_list();

    assert_eq!(list.remove(2), 30);
}
