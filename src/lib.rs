mod utils;
use serde::{ Serialize, Deserialize };
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use serde_wasm_bindgen::{from_value, to_value};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[derive(Serialize, Deserialize)]
struct AppState {
    counter: i64
}

type StateSetter = Rc<RefCell<AppState>>;

#[wasm_bindgen]
pub struct StateManager {
    state: StateSetter,
    needs_update: bool
}


#[wasm_bindgen]
impl StateManager{
    pub fn new(initial_state: JsValue) -> StateManager {
        let state: AppState = from_value(initial_state).unwrap();
        StateManager {
            state: Rc::new(RefCell::new(state)),
            needs_update: false
        }
    }

    pub fn set_state(&self, new_state: JsValue) -> Result<(), JsValue> {
        let mut state_ref = self.state.borrow_mut();
        *state_ref = from_value(new_state)?;
        Ok(())
    }

    pub fn update_state(&mut self, state_diff: HashMap<String, JsValue>){
        let mut state_ref = self.state.borrow_mut();
        for (key, value) in state_diff {
            match key.as_str() {
                "counter" => state_ref.counter = value.as_f64().unwrap() as i64,
                _ => {}
            }
        }
        self.needs_update = true;
    }

    pub fn get_state(&self) -> JsValue {
        to_value(&*self.state.borrow()).unwrap()
    }
}

