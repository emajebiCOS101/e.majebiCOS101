struct Laptop{
   hp:u32,
   ibm:u32,
   toshiba:u32,
   dell:u32

}
   fn main() {
    let small = Laptop{
        hp:650_000,
        ibm:755_000,
        toshiba:550_000,
        dell:850_000
    };

impl Laptop{
    fn hp_order(&self)->u32{
        self.hp * 3
    }
}
impl Laptop{
    fn ibm_order(&self)->u32{
        self.ibm * 3
    }
}
impl Laptop{
    fn toshiba_order(&self)->u32{
        self.toshiba * 3
    }
}
impl Laptop{
    fn dell_order(&self)->u32{
        self.dell * 3
    }
}
 impl Laptop{
    fn total_order(&self)->u32{
     self.hp_order() + self.ibm_order() + self.toshiba_order() + self.dell_order()
    }
 }
println!(" HP costs {} \n IBM costs {} \n Toshiba costs {} \n Dell costs {} \n 
    Your total order is {}",small.hp_order(),small.ibm_order(),small.toshiba_order(),small.dell_order(),small.total_order());
}
