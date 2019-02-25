#[macro_export]
macro_rules! __graphql {
    ($e:expr, $f:expr) => {{
        $f.validate()?;
        let (c, s) = $e.context();
        let v = $f.handle(c, s)?;
        Ok(v)
    }};
}

#[macro_export]
macro_rules! __i18n_e {
    ($d:expr, $l:expr, $c:expr) => {{
        Err(I18n::e($d, $l, $c, &None::<String>).into())
    }};
    ($d:expr, $l:expr, $c:expr, $v:expr) => {{
        Err(I18n::e($d, $l, $c, $v).into())
    }};
}

#[macro_export]
macro_rules! __i18n_l {
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr) => {{
        let m = I18n::t($d, $l, $c, &None::<String>);
        LogDao::add($d, $u, $i, m)
    }};
    ($d:expr, $u:expr, $i:expr, $l:expr, $c:expr, $v:expr) => {{
        let m = I18n::t($d, $l, $c, $v);
        LogDao::add($d, $u, $i, m)
    }};
}
