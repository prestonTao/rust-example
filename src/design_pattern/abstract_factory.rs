//抽象工厂

use super::simple_factory::*;

trait PizzaFactory {
    fn create_pizza(&self, pizza_name: PizzaName) -> Box<Pizza>;
}
struct ShenZhenFactory {}
impl PizzaFactory for ShenZhenFactory {
    fn create_pizza(&self, pizza_name: PizzaName) -> Box<Pizza> {
        match pizza_name {
            PizzaName::CheesePizza => Box::new(CheesePizza::new("shen zhen cheese".to_string())),
            PizzaName::ClamPizza => Box::new(ClamPizza::new("shen zhen clam".to_string()))
        }
    }
}
struct BeiJingFactory {}
impl PizzaFactory for BeiJingFactory {
    fn create_pizza(&self, pizza_name: PizzaName) -> Box<Pizza> {
        match pizza_name {
            PizzaName::CheesePizza => Box::new(CheesePizza::new("bei jing cheese".to_string())),
            PizzaName::ClamPizza => Box::new(ClamPizza::new("bei jing clam".to_string()))
        }
    }
}

struct PizzaStore {
    factory: Box<PizzaFactory>,
}
impl PizzaStore {
    fn order_pizza(&self, pizza_name: PizzaName) -> Box<Pizza> {
        let pizza = self.factory.create_pizza(pizza_name);
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
        let shenzhen_store = PizzaStore{factory: Box::new(ShenZhenFactory{})};
        shenzhen_store.order_pizza(PizzaName::CheesePizza);
        let beijing_store = PizzaStore{factory: Box::new(BeiJingFactory{})};
        beijing_store.order_pizza(PizzaName::ClamPizza);
    }
}