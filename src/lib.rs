mod utils;
use serde::{ Serialize, Deserialize };
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

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
    state: StateSetter
}


#[wasm_bindgen]
impl StateManager{
    pub fn new(initial_state: JsValue) -> StateManager {
        let state: AppState = from_value(initial_state).unwrap();
        StateManager {
            state: Rc::new(RefCell::new(state)),
        }
    }

    pub fn set_state(&self, new_state: JsValue) -> Result<(), JsValue> {
        let mut state_ref = self.state.borrow_mut();
        *state_ref = from_value(new_state)?;
        Ok(())
    }

    pub fn get_state(&self) -> JsValue {
        to_value(&*self.state.borrow()).unwrap()
    }
}

