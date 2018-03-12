#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_plain;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Test {
    FooBarBaz,
    BlahBlah,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Test2 {
    FooBarBaz,
    BlahBlah,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Test2Error;

impl From<serde_plain::Error> for Test2Error {
    fn from(_: serde_plain::Error) -> Test2Error {
        Test2Error
    }
}

forward_from_str_to_serde!(Test);
forward_display_to_serde!(Test);

forward_from_str_to_serde!(Test2, Test2Error);
forward_display_to_serde!(Test2);

#[test]
fn test_basics() {
    assert_eq!(Test::FooBarBaz.to_string(), "foo_bar_baz");
    assert_eq!("foo_bar_baz".parse::<Test>().unwrap(), Test::FooBarBaz);
}

#[test]
fn test_custom_error() {
    assert_eq!("whatever".parse::<Test2>(), Err(Test2Error));
}