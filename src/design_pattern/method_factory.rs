//工厂方法
use super::simple_factory::*;

trait BasePizzaStore {
    fn order_pizza(&self, pizza_name: PizzaName) -> Box<Pizza> {
        let pizza = self.pizza_creator(pizza_name);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.pack();
        pizza
    }

    fn pizza_creator(&self, pizza_name: PizzaName) -> Box<Pizza>;
}

struct ShenZhenPizzaStore {
    location: String,
}
impl BasePizzaStore for ShenZhenPizzaStore {
    fn pizza_creator(&self, pizza_name: PizzaName) -> Box<Pizza> {
        match pizza_name {
            PizzaName::CheesePizza => Box::new(CheesePizza::new(self.location.clone() + " cheese")),
            PizzaName::ClamPizza => Box::new(ClamPizza::new(self.location.clone() + " clam"))
        }
    }
}

struct BeiJingPizzaStore {}
impl BasePizzaStore for BeiJingPizzaStore {
    fn pizza_creator(&self, pizza_name: PizzaName) -> Box<Pizza> {
        match pizza_name {
            PizzaName::CheesePizza => Box::new(CheesePizza::new("bei jing cheese".to_string())),
            PizzaName::ClamPizza => Box::new(ClamPizza::new("bei jing clam".to_string()))
        }
    }
}


// 某些语言中的接口中不能写方法实现，且没有继承，因此通过如下办法也能实现类似的工厂方法，
// 但是需要注意下边的方法无法为每个工厂方法定制变量（如上边ShenZhenPizzaStore中的location）。
// 虽然以闭包的形式注入PizzaStore中是可以带入定制变量，但这样做会增加代码的复杂度，
// 还不如直接使用抽象工厂。
fn shen_zhen_pizza_creator(pizza_name: PizzaName) -> Box<Pizza> {
    match pizza_name {
        PizzaName::CheesePizza => Box::new(CheesePizza::new("shen zhen cheese".to_string())),
        PizzaName::ClamPizza => Box::new(ClamPizza::new("shen zhen clam".to_string()))
    }
}

fn bei_jing_pizza_creator(pizza_name: PizzaName) -> Box<Pizza> {
    match pizza_name {
        PizzaName::CheesePizza => Box::new(CheesePizza::new("bei jing cheese".to_string())),
        PizzaName::ClamPizza => Box::new(ClamPizza::new("bei jing clam".to_string()))
    }
}

struct PizzaStore {
    pizza_creator: fn(pizza_name: PizzaName) -> Box<Pizza>,
}
impl PizzaStore {
    fn order_pizza(&self, pizza_name: PizzaName) -> Box<Pizza> {
        let pizza = (self.pizza_creator)(pizza_name);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.pack();
        pizza
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn method_factory_test() {
        let sz_store = ShenZhenPizzaStore{ location: "shen zhen".to_string() };
        sz_store.order_pizza(PizzaName::CheesePizza);
        let bj_store = BeiJingPizzaStore{};
        bj_store.order_pizza(PizzaName::CheesePizza);

        println!("--------------------------------------------------");

        let shenzhen_store = PizzaStore{pizza_creator: shen_zhen_pizza_creator};
        shenzhen_store.order_pizza(PizzaName::CheesePizza);
        let beijing_store = PizzaStore{pizza_creator: bei_jing_pizza_creator};
        beijing_store.order_pizza(PizzaName::ClamPizza);
    }
}
