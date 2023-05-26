macro_rules! debug {
    ($log:block) => {{
        #[cfg(debug_assertions)]
        use crate::debug_log;
        #[cfg(debug_assertions)]
        debug_log(&$log);
    }};
}
pub(crate) use debug;

macro_rules! consume_fixed_arr {
    ($decoder:ident) => {
        $decoder.array()?
    };
}
pub(crate) use consume_fixed_arr;

macro_rules! consume_arr_len {
    ($decoder:ident) => {
        match $decoder.array()? {
            Some(n) => n,
            None => return Err(decode::Error::message("Array length not specified")),
        }
    };
}
pub(crate) use consume_arr_len;

macro_rules! consume_string {
    ($decoder:ident) => {
        $decoder.str()?.to_owned()
    };
}
pub(crate) use consume_string;

macro_rules! consume_num {
    ($decoder:ident) => {
        $decoder.u32()?
    };
}
pub(crate) use consume_num;

macro_rules! consume_inum {
    ($decoder:ident) => {
        $decoder.i32()?
    };
}
pub(crate) use consume_inum;
