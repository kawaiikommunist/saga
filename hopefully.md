 - An average mod looks like this:
```
use saga_modding::prelude::*;

manifest!(
   require: [BASE]
);

state!();

#[build]
fn initialize(mut stage: Stage) {
    // Do Stuff
}

// Output of #[build]
#[no_mangle]
pub extern "C" fn register_mod_systems() {
    if let Ok(mut guard) = STATE.write() {
        if let Some(ref mut stage) = *guard {
            initialize(stage);
        }
    }
}

```
