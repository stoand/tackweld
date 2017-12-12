#[macro_export]
macro_rules! tw {
    ( $file_name:expr, $($inner:tt)*  ) => {
        format!(
            include_str!(concat!(env!("OUT_DIR"), "/", "tw_tpl_", stringify!($file_name))),
            $($inner)*)
    }
}

pub enum TemplateArg {
    ev(),
    a(String),
    v(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
