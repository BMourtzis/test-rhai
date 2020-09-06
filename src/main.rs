use rhai::{
    Engine,
    Scope,
    RegisterFn
};

use std::{ cell::RefCell, rc::Rc };

fn main() {
    let mut engine = Engine::new();

    let mut obj = Obj::new(4.0, 5.0, 6.0);

    obj.inject_to_engine(&mut engine);

    let filepath = "src/scripts/test.rhai";
    let ast = engine.compile_file(filepath.into()).expect("Could not compile file");

    let mut scope = Scope::new();
    scope.push("delta", 4_f64);

    let _res: bool = engine.call_fn(&mut scope, &ast, "start", ())
        .expect("Could not run fn");

    println!("x = {}", obj.transform.borrow().x);

    loop {
        let _res: bool = engine.call_fn(&mut scope, &ast, "update", ())
            .expect("Could not run fn");

        println!("x = {}", obj.transform.borrow().x);
    }
}

#[derive(Debug, Clone, Default)]
struct Transform {
    x: f64,
    y: f64,
    z: f64
}

impl Transform {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Transform {
            x,
            y,
            z
        }
    }
}

#[derive(Debug, Clone)]
struct Obj {
    transform: Rc<RefCell<Transform>>
}

impl Obj {
    fn new (x: f64, y: f64, z: f64) -> Self {
        Obj {
            transform: Rc::new(RefCell::new(Transform::new(x, y, z)))
        }
    }
}

impl EngineInjectable for Obj {
    fn inject_to_engine(&mut self, engine: &mut Engine) {
        //Transform
        let transform_clone = self.transform.clone();
        engine.register_fn("t_update_x", move |x: f64| {
            transform_clone.borrow_mut().x = x;
        });

        let transform_clone = self.transform.clone();
        engine.register_fn("t_update_y", move |y: f64| {
            transform_clone.borrow_mut().y = y;
        });

        let transform_clone = self.transform.clone();
        engine.register_fn("t_update_z", move |z: f64| {
            transform_clone.borrow_mut().z = z;
        });
    
        let transform_clone = self.transform.clone();
        engine.register_fn("t_get_x", move || {
            transform_clone.borrow().x
        });

        let transform_clone = self.transform.clone();
        engine.register_fn("t_get_y", move || {
            transform_clone.borrow().y
        });

        let transform_clone = self.transform.clone();
        engine.register_fn("t_get_z", move || {
            transform_clone.borrow().z
        });
    }
}

trait EngineInjectable {
    fn inject_to_engine(&mut self, engine: &mut Engine);
}