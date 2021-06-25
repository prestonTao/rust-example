//装饰者模式

trait Beverage {
    fn get_desc(&self) -> String ;
    fn cost(&self) -> f64;
}

struct Tee {}
impl Beverage for Tee {
    fn get_desc(&self) -> String {
        return String::from("茶")
    }

    fn cost(&self) -> f64 {
        return 2.2
    }
}

struct Coffee {}
impl Beverage for Coffee {
    fn get_desc(&self) -> String {
        return String::from("普通咖啡")
    }
    fn cost(&self) -> f64 {
        return 1.22
    }
}
struct Sugar {
    beverage: Box<Beverage>
}
impl Sugar {
    fn new(b: Box<Beverage>) -> Sugar {
        return Sugar{beverage: b}
    }
}
impl Beverage for Sugar {
    fn get_desc(&self) -> String {
        return self.beverage.get_desc() + "+糖"
    }

    fn cost(&self) -> f64 {
        return self.beverage.cost() + 0.8
    }
}
struct Milk {
    beverage: Box<Beverage>
}
impl Milk {
    fn new(b: Box<Beverage>) -> Milk {
        return Milk{beverage: b}
    }
}
impl Beverage for Milk {
    fn get_desc(&self) -> String {
        return self.beverage.get_desc() + "+牛奶"
    }

    fn cost(&self) -> f64 {
        return self.beverage.cost() + 1.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decorator_test () {
        // 菜单1：咖啡+牛奶+糖
        let order1 = Milk::new(Box::new(Sugar::new(Box::new(Coffee{}))));
        println!("{}", order1.get_desc());
        assert_eq!(3.12, order1.cost());
        // 菜单2：茶+糖
        let order2 = Sugar::new(Box::new(Tee{}));
        println!("{}", order2.get_desc());
        assert_eq!(3.0, order2.cost());
    }
}