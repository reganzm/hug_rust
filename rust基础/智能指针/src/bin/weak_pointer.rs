use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    let car = Rc::new(Car {
        name: "国货之光-仰望U8".to_string(),
        wheels: RefCell::new(vec![]),
    });

    let wheel1 = Rc::new(Wheel {
        id: 1,
        car: car.clone(),
    });
    let wheel2 = Rc::new(Wheel {
        id: 2,
        car: car.clone(),
    });
    let wheel3 = Rc::new(Wheel {
        id: 3,
        car: car.clone(),
    });
    let wheel4 = Rc::new(Wheel {
        id: 4,
        car: car.clone(),
    });
    {
        let mut wheels = car.wheels.borrow_mut();
        // 调用downgrade得到Weak
        wheels.push(Rc::downgrade(&wheel1));
        wheels.push(Rc::downgrade(&wheel2));
        wheels.push(Rc::downgrade(&wheel3));
        wheels.push(Rc::downgrade(&wheel4));
    }

    for week_wheel in car.wheels.borrow().iter() {
        // 调用upgrade得到Rc
        let tmp = week_wheel.upgrade().unwrap();
        println!("wheel:{:?} owned by {:?}", tmp.id, tmp.car.name);
    }
}

struct Car {
    name: String,
    wheels: RefCell<Vec<Weak<Wheel>>>,
}
struct Wheel {
    id: i32,
    car: Rc<Car>,
}
