use std::mem;
use windows::{
    core::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

const MAIN_CLASS_NAME: PCWSTR = w!("TEMPLATE_CLASS_NAME");

const MAIN_WINDOW_TITLE: PCWSTR = w!("TEMPLATE_WINDOW_TITLE");
const MAIN_WINDOW_WIDTH: i32 = 800;
const MAIN_WINDOW_HEIGHT: i32 = 600;

const WINDOW_STYLES: WINDOW_STYLE = WS_OVERLAPPEDWINDOW;

#[macro_export]
macro_rules! pw {
    ($l:literal) => {
        ::windows::core::PWSTR(::windows::core::w!($l).0 as _)
    };
}

pub struct MainWindow {
    instance: HINSTANCE,
    handle: HWND,
}

impl MainWindow {
    pub fn new() -> Result<Self> {
        unsafe {
            Ok(Self {
                instance: GetModuleHandleW(None)?.into(),
                handle: HWND(0),
            })
        }
    }

    pub fn build(&mut self, show: SHOW_WINDOW_CMD) -> Result<()> {
        unsafe {
            let wc = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wnd_proc),
                hInstance: self.instance,
                hIcon: LoadIconW(None, IDI_APPLICATION)?,
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hbrBackground: HBRUSH(6), // COLOR_WINDOW + 1
                lpszClassName: MAIN_CLASS_NAME,
                hIconSm: LoadIconW(None, IDI_APPLICATION)?,
                ..Default::default()
            };

            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_win32());
            }

            self.handle = CreateWindowExW(
                WS_EX_CLIENTEDGE,
                MAIN_CLASS_NAME,
                MAIN_WINDOW_TITLE,
                WINDOW_STYLES,
                CW_USEDEFAULT,
                0,
                MAIN_WINDOW_WIDTH,
                MAIN_WINDOW_HEIGHT,
                None,
                Self::create_main_menu()?,
                self.instance,
                Some(self as *mut Self as _),
            );

            if self.handle == HWND(0) {
                return Err(Error::from_win32());
            }

            ShowWindow(self.handle, show);
            UpdateWindow(self.handle);
        }

        Ok(())
    }

    pub fn message_loop(&self) -> isize {
        unsafe {
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if msg.message == WM_QUIT {
                    break;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            msg.wParam.0 as _
        }
    }

    unsafe fn create_main_menu() -> Result<HMENU> {
        let menu = HMENU(0);

        Ok(menu)
    }

    /// The raw Window Procedure. It calls `self.message_handle` after the first message has been processed.
    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_NCCREATE {
            // Draw title bar.
            DefWindowProcW(hwnd, msg, wparam, lparam);

            let cc = &*(lparam.0 as *const CREATESTRUCTW);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, cc.lpCreateParams as _);
            LRESULT(1)
        } else {
            let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;

            match (*this).message_handle(hwnd, msg, wparam, lparam) {
                Some(res) => LRESULT(res),
                None => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }

    unsafe fn message_handle(
        &mut self,
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<isize> {
        let mut res = 0;

        match msg {
            WM_PAINT => {
                let (mut rect, mut ps) = Default::default();
                GetClientRect(hwnd, &mut rect).ok()?;

                let hdc = BeginPaint(hwnd, &mut ps);

                let text = w!("This is a long sample text, with the default font.");
                DrawTextW(
                    hdc,
                    as_mut(text.as_wide()),
                    &mut rect,
                    DT_VCENTER | DT_CENTER | DT_SINGLELINE,
                );

                EndPaint(hwnd, &ps);
            }
            WM_DESTROY => {
                PostQuitMessage(0);
            }
            _ => return None,
        }

        Some(res)
    }
}

unsafe fn as_mut<'a, T>(slice: &'a [T]) -> &'a mut [T] {
    std::slice::from_raw_parts_mut(slice.as_ptr() as _, slice.len())
}
