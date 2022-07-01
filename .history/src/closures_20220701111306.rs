struct Cars{
    maker: &'static str,
    models:Vec<(&'static str, i32)>,
}
impl Cars{
    fn get_cars<x>(&self, x:X)
    where
    X: Fn (&Vec<(&'static , i32)>)
    {
        f(&self.models)
    }
}
fn 