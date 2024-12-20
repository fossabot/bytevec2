#[macro_use]
extern crate bytevec2;

use bytevec2::{ByteDecodable, ByteEncodable};

#[test]
fn test_serialize_vec() {
    bytevec_decl! {
        #[derive(PartialEq, Eq, Debug)]
        struct Employee {
            id: u32,
            profile: Profile,
            dept: String
        }

        #[derive(PartialEq, Eq, Debug)]
        struct Profile {
            id: u32,
            name: String,
            last_name: String
        }
    }

    let employees_1 = vec![
        Employee {
            id: 1,
            profile: Profile {
                id: 10000,
                name: "Michael".to_string(),
                last_name: "Jackson".to_string(),
            },
            dept: "music".to_string(),
        },
        Employee {
            id: 2,
            profile: Profile {
                id: 10001,
                name: "John".to_string(),
                last_name: "Cena".to_string(),
            },
            dept: "wrestling".to_string(),
        },
    ];
    let bytes = employees_1.encode::<u32>().unwrap();
    let employees_2 = Vec::<Employee>::decode::<u32>(&bytes).unwrap();
    assert_eq!(employees_1, employees_2);
}

bytevec_decl! {
    #[derive(PartialEq, Eq, Debug)]
    struct MeetingsLog {
        id: u32,
        owner: String,
        meetings: std::collections::HashMap<String, String>
    }
}

#[test]
fn test_serialize_slices() {
    let slice = &['1', '2', '3'];
    let bytes = slice.encode::<u32>().unwrap();
    let vec = Vec::<char>::decode::<u32>(&bytes).unwrap();
    assert_eq!(vec, slice);
}

#[test]
fn test_serialize_set() {
    let mut set_1 = std::collections::HashSet::new();
    set_1.insert(("One!".to_string(), 1u32));
    set_1.insert(("Two!".to_string(), 2));
    set_1.insert(("Three!".to_string(), 3));
    let bytes = set_1.encode::<u32>().unwrap();
    let set_2 = std::collections::HashSet::decode::<u32>(&bytes).unwrap();
    assert_eq!(set_1, set_2);
}

#[test]
fn test_serialize_tuple() {
    let pair1 = ("Hello".to_string(), "World".to_string());
    let bytes = pair1.encode::<u8>().unwrap();
    let pair2 = <(String, String)>::decode::<u8>(&bytes).unwrap();
    assert_eq!(pair1, pair2);
}

#[test]
fn test_serialize_map() {
    let mut classes_1 = std::collections::HashMap::new();
    classes_1.insert(101usize, "Programming 1".to_string());
    classes_1.insert(102, "Basic CS".to_string());
    let bytes = classes_1.encode::<u32>().unwrap();
    let classes_2 = std::collections::HashMap::decode::<u32>(&bytes).unwrap();
    assert_eq!(classes_1, classes_2);
}

#[test]
fn test_serialize_slice_with_map_containers() {
    let slice = &[
        MeetingsLog {
            id: 1,
            owner: "Jack".to_string(),
            meetings: {
                let mut meetings_jack = std::collections::HashMap::new();
                meetings_jack.insert("New York".to_string(), "Michael".to_string());
                meetings_jack.insert("Nippon".to_string(), "Koichi".to_string());
                meetings_jack
            },
        },
        MeetingsLog {
            id: 2,
            owner: "Juan".to_string(),
            meetings: {
                let mut meeting_juan = std::collections::HashMap::new();
                meeting_juan.insert("España".to_string(), "José".to_string());
                meeting_juan.insert("Korea".to_string(), "Lee Hyun".to_string());
                meeting_juan
            },
        },
    ];

    let bytes = slice.encode::<u64>().unwrap();
    let vec = <Vec<MeetingsLog>>::decode::<u64>(&bytes).unwrap();
    assert_eq!(vec, slice);
}

#[test]
fn test_u8_upper_limit_encode_err() {
    let vec = vec![0u32; 100];
    assert!(vec.encode::<u8>().is_err());
}

#[test]
fn test_decode_limit_err() {
    let vec = vec![0u32; 100];
    let bytes = vec.encode::<u32>().unwrap();
    assert!(<Vec<u32>>::decode_max::<u32>(&bytes, 100).is_err());
}

#[test]
fn test_serialize_bool() {
    bytevec_decl! {
        #[derive(PartialEq, Eq, Clone, Debug)]
        struct BooleanStuff {
            simple: bool,
            optional: Vec<bool>
        }
    }

    let data1 = vec![
        BooleanStuff {
            simple: true,
            optional: vec![true, false, false, false],
        },
        BooleanStuff {
            simple: false,
            optional: vec![true, false, true, false],
        },
    ];
    let bytes = data1.encode::<u32>().unwrap();
    let data2 = Vec::<BooleanStuff>::decode::<u32>(&bytes).unwrap();
    assert_eq!(data1, data2);
}

#[test]
fn test_serialize_options() {
    bytevec_decl! {
        #[derive(PartialEq, Eq, Clone, Debug)]
        struct OptionStuff {
            simple: Option<bool>,
            some: Option<i64>,
            some_vec: Option<Vec<u32>>
        }
    }

    let data1 = vec![
        OptionStuff {
            simple: None,
            some: Some(8i64),
            some_vec: Some(vec![7, 3, 5, 9, 1, 4, 4]),
        },
        OptionStuff {
            simple: Some(true),
            some: Some(50i64),
            some_vec: Some(vec![1, 2, 3, 5, 4, 6, 7, 8, 9, 10]),
        },
    ];
    let bytes = data1.encode::<u32>().unwrap();
    let data2 = Vec::<OptionStuff>::decode::<u32>(&bytes).unwrap();
    assert_eq!(data1, data2);
}
