use tcod::colors::Color;

static mut LOGS: Option<Vec<(String, Color)>> = None;

pub fn log(message: &str, color: Color) {
    unsafe {
        if LOGS.is_none() {
            LOGS = Some(vec![]);
        }
        LOGS.as_mut().unwrap().push((message.into(), color));
    }
}

pub fn logs<'a>() -> &'a [(String, Color)] {
    unsafe {
        if LOGS.is_none() {
            LOGS = Some(vec![]);
        }

        //LOGS.lock().unwrap()
        LOGS.as_ref().unwrap()
    }
}
