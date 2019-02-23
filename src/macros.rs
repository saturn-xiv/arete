#[macro_export]
macro_rules! __graphql {
    ($e:expr, $f:expr) => {{
        $f.validate()?;
        let (c, s) = $e.context();
        let v = $f.handle(c, s)?;
        Ok(v)
    }};
}
