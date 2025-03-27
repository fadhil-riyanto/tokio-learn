use std::rc::Rc;

struct Owner {
    name: String,
}

struct Gadget {
    id: u32,
    owner: Rc<Owner>
}

fn main() {
    let ownerd = Rc::new(
        Owner {
            name: "fadhild".to_string()
        }
    );

    let gadget1 = Gadget {
        id: 0,
        owner: Rc::clone(&ownerd)
    };

    let gadget2 = Gadget {
        id: 1,
        owner: Rc::clone(&ownerd)
    };

    let gadget3 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget2.owner)
    };

    println!("{}. {}", Rc::strong_count(&gadget1.owner), gadget1.owner.name);

    drop(gadget1);
    println!("{}. {}", Rc::strong_count(&gadget2.owner), gadget2.owner.name);
    println!("{}. {}", Rc::strong_count(&gadget3.owner), gadget3.owner.name);
    
}