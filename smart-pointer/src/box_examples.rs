// Situations to Use Box<T>
// 1. When you have a type that’s too large for the stack
// 2. When you need recursion
// 3. When you want to own a trait object (dynamic dispatch)
pub fn too_large_data() {
    struct BigData {
        // data: [u8; 9_000_000], // 1 MB!
        data: Vec<u8>,
    }

    let x = Box::new(BigData {
        data: vec![0; 9_000_000],
    });
    println!("BigData is on the heap!");
    println!("{:#?}", x.data);
}

pub fn dynamic_dispatch() {
    trait Animal {
        fn speak(&self);
    }
    struct Dog;
    struct Cat;
    impl Animal for Dog {
        fn speak(&self) {
            println!("bhaw bhaw");
        }
    }
    let pet: Box<dyn Animal> = Box::new(Dog);
    pet.speak();
}
