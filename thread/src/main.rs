use itertools::Itertools;
use std::sync::{mpsc, Arc, Mutex};
use std::{iter, thread};
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    first_fork: Arc<Mutex<Fork>>,
    second_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is trying to eat", &self.name);
        let ff = self.first_fork.lock().unwrap();
        let sf = self.second_fork.lock().unwrap();

        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));

        drop(sf);
        drop(ff);
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create forks
    let forks = PHILOSOPHERS.iter().map(|_| Arc::new(Mutex::new(Fork {})))
        .collect::<Vec<_>>();

    let fork_pairs = forks.iter().cloned().enumerate().circular_tuple_windows();
    // let fork_pairs = iter::zip(
    //     forks.iter().cloned().enumerate(),
    //     forks.iter().cloned().enumerate().cycle().skip(1)
    // );

    let (tx, rx) = mpsc::channel();

    // Create philosophers
    let handles = PHILOSOPHERS.iter().zip(fork_pairs).zip(iter::repeat_with(|| tx.clone()))
        .map(
            |((n, ((left_fork_idx, left_fork), (right_fork_idx, right_fork))), tx)| {
                let (first_fork, second_fork) = if left_fork_idx < right_fork_idx {
                    (left_fork, right_fork)
                } else {
                    (right_fork, left_fork)
                };
                Philosopher {
                    name: n.to_string(),
                    first_fork,
                    second_fork,
                    thoughts: tx,
                }
            }
        ).map(|p| {
        thread::spawn(move || {
            // Make each of them think and eat 100 times
            for _ in 0..100 {
                p.think();
                p.eat();
            }
        })
    }).collect::<Vec<_>>();


    drop(tx);

    // Output their thoughts
    for thought in rx {
        println!("{thought}");
    }

    for h in handles {
        h.join().unwrap();
    }
}
