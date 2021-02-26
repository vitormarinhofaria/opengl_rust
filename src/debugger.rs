#[cfg(debug_assertions)]
pub fn dbg_log<S: std::fmt::Display>(string: S){
    println!("{}", string);
}

#[cfg(not(debug_assertions))]
pub fn dbg_log<S: std::fmt::Display>(string: S) {}
