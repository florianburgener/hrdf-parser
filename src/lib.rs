use std::{error::Error, vec};

struct A<T> {
    data: T,
}

impl<T> A<T> {
    fn new(data: T) -> Self {
        A { data }
    }
}

// fn test() -> ??? {
//     let a = vec![A::new(32), A::new(24.5)];
//     a
// }

#[derive(Debug)]
enum ParsedItem {
    U32(u32),
    F64(f64),
}

impl From<ParsedItem> for u32 {
    fn from(item: ParsedItem) -> u32 {
        match item {
            ParsedItem::U32(u32_val) => u32_val,
            _ => panic!(""),
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    print!("Hello\n");
    let a = A::new(32);
    println!("{}", a.data);
    let my_list = vec![a];
    let b: i32 = my_list[0].data;
    println!("{}", b);

    let a = "535324";
    // let aaa = a.parse::<u32>().unwrap();
    let aaa = ParsedItem::F64(a.parse::<f64>().unwrap());
    println!("{:?}", u32::from(aaa));

    Ok(())
}
