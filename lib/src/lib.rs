use std::fmt;

#[macro_export]
macro_rules! tw_args {
    ( $key:ident=$val:expr ) => {
        $key=$val
    };
}

#[macro_export]
macro_rules! tw {
    ( $file_name:expr, $($key:ident = $val:expr)* ) => {
        $crate::TemplateItem::Raw(format!(
            include_str!(concat!(env!("OUT_DIR"), "/", "tw_tpl_", stringify!($file_name))),
            // prevent strings from being passed directly here, only template
            // items are allowed so the proper escaping can be done
             $( $key = $crate::TemplateItem::from($val) )*))
    };
}

pub enum TemplateItem {
    Ev(),
    Attr(String),
    Val(String),
    Raw(String),
}

pub fn val<T: fmt::Display>(v: T) -> TemplateItem {
    TemplateItem::Val(v.to_string())
}

pub fn att<T: fmt::Display>(v: T) -> TemplateItem {
    TemplateItem::Attr(v.to_string())
}

impl From<Vec<TemplateItem>> for TemplateItem {
    fn from(vec: Vec<TemplateItem>) -> TemplateItem {
        let item_strings = vec.into_iter().map(|i| i.to_string()).collect::<Vec<_>>();
        TemplateItem::Val(item_strings.join(""))
    }
}

impl fmt::Display for TemplateItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TemplateItem::*;

        let val = match self {
            // TODO: sanitize html
            &Ev() => "",
            &Attr(ref t) => t,
            &Val(ref t) => t,
            &Raw(ref t) => t,
        };

        write!(f, "{}", val)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert_eq!
        assert_eq!(2 + 2, 4);
    }
}
