use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows_sys::Win32::UI::WindowsAndMessaging::SetCursorPos;

#[no_mangle]
pub extern "C" fn getMousePos(cords_ptr:*mut [i32;2])->bool{
    let mut p=POINT{x:0,y:0};
    unsafe{
        let success=GetCursorPos(&mut p);
        if success==1 {
            let view:&mut [i32;2]=&mut *cords_ptr;
            view[0]=p.x;
            view[1]=p.y;
            return true;
        }
        return false;
    }
}
#[no_mangle]
pub extern "C" fn setMousePos(x:i32,y:i32)->bool{
    unsafe{
        let success=SetCursorPos(x,y);
        if success==1 {
            return true;
        }
        return false

    }
    
}