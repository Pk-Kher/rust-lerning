#[derive(Debug)]
struct Data {
    name: String,
    value: String,
}
struct LifeTimeStruct<'a> {
    data: &'a Data,
}
impl<'a> LifeTimeStruct<'a> {
    fn show_the_data(&self) {
        println!("name:{},value:{}", self.data.name, self.data.value);
    }
}
pub fn life_time_ex() {
    let s1 = String::from("this is string s1");
    {
        let s2 = "this is s2";
        let result = longest_str(s1.as_str(), s2);
        longest(s1.as_str(), s2);
        println!("{}", result);
    }
    // let name=String::from("pk");
    let data_struct = Data {
        name: String::from("pk"),
        value: String::from("none"),
    };
    let life = LifeTimeStruct { data: &data_struct };
    life.show_the_data();
    // println!("{:#?} {:#?}", life.data, data_struct);
}
fn longest_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// fn longest_v3<'a>(x:&'a str,_y:&str)->&'a str{
//    let data=String::from("hello");
//     data.as_str()
// }

fn no_need_lifetime(s: &str) -> &str {
    "hello"
}
pub fn foo_life_time() {
    struct Foo<'a> {
        bar: &'a i32,
    }

    fn baz(f: Foo) -> &i32 {
        f.bar
    }
    fn baz2<'a, 'b>(f: &'b Foo<'a>) -> &'a i32 {
        f.bar
    }
    let value: i32 = 90;
    let foo_data = Foo { bar: &value };
    println!("{}", baz2(&foo_data));
    // println!("{}", baz(foo_data));
}
