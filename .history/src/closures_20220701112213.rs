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
fn main(){
    let car1_collection = vec![("Camry", 2015), ("Corolla", 2013), ("Avenza", 2010), ("Matrix", 2009)]
    let car2_collection = vec![("Rx330", 2006),("Lx350", 2009), ("Lx300", 2008), ("Gx570", 2012)]
    let car3_collections = vec![("PathFinder", 2009), ("Altima", 2008), ("Maxima",2007), ("Pi")]
}