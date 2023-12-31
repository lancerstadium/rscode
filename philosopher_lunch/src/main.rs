
// 哲学家进餐问题
use std::{thread, time};
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}



impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is eating.", self.name);
        let thousand_ms = time::Duration::from_millis(1000);
        thread::sleep(thousand_ms);
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table {forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        Philosopher::new("Michel Foucault", 4, 0),
    ];

    // for p in &philosophers {
    //     p.eat();
    // }

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
    

}
