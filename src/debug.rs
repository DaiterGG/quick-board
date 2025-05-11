#[macro_export]
macro_rules! d {
    () => {
        eprint!()
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprint!("{} = {:?}, ",
                     stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(d!($val)),+,)
    };
}
#[macro_export]
macro_rules! dl {
    () => {
        eprintln!()
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("{} = {:?}, {}:{}",
                    stringify!($val), &tmp,
                    file!(), line!());
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dl!($val)),+,)
    };
}
