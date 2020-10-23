use serde::{Deserialize, Serialize};
use crate::game::{guard, trigger};

pub trait KeyedStore: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug {
    type KeyEnum: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug;

    fn check_guard(&self, guard: &guard::Guard<Self>) -> bool {
        match guard {
            guard::Guard::And(gg) => {
                gg.iter().all(|g| self.check_guard(g))
            }
            guard::Guard::Or(gg) => {
                gg.iter().any(|g| self.check_guard(g))
            }
            guard::Guard::Not(g) => {
                !self.check_guard(&*g)
            }
            guard::Guard::Value(g) => {
                self.check_keyed_guard(g)
            }
        }
    }

    fn check_keyed_guard(&self, guard: &guard::KeyedGuard<Self>) -> bool;

    fn update_with_value(&mut self, change: &trigger::StatChange<Self>);
}
