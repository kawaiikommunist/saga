use slate::STATE;

#[unsafe(no_mangle)]
pub extern "C" fn register() {
    let mut gaurd = STATE.write().unwrap();
    let mut state = gaurd.unwrap();
}
