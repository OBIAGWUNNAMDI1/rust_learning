struct Cars{
    maker: &'static str,
    models:Vec<(&'static str, i32)>,
}
impl Cars{
    fn get_cars<X>(&self, x:X)
    where
    x: Fn (&Vec<(&'staticstr, i32)>)
    {
        x(&self.models)
    }
}
fn main(){
    let car1_collection = vec![("Camry", 2015), ("Corolla", 2013), ("Avenza", 2010), ("Matrix", 2009)];
    let car2_collection = vec![("Rx330", 2006),("Lx350", 2009), ("Lx300", 2008), ("Gx570", 2012)];
    let car3_collections = vec![("PathFinder", 2009), ("Altima", 2008), ("Maxima",2007), ("Pilot",2007)];
    let toyota_cars = Cars{maker:"Toyota", models:car1_collection};
    let lexus_cars = Cars{maker:"Lexus", models: car2_collection};
    let nissan_cars = Cars{maker:"Nisssan", models: car3_collections};
    toyota_cars.get_cars(|y|{
        let res:Vec<_> = y.into_iter().filter(|y| y.1 > 2010).collect();
        println!("toyota_cars available {:?}", res);
        let res:Vec<_> = x.into_iter().map(|y| format!("{}_{}",x.1,x.0)).collect();
        println("res map{:?}", res);
    }
    )
}
