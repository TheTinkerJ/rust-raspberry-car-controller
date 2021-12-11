/// 手柄底层事件
use libc::{__s16, __u32, __u8};

/// 手柄事件:参考原始定义`/usr/include/linux/joystick.h`
/// ```c
/// #define JS_EVENT_BUTTON         0x01    /* button pressed/released */
/// #define JS_EVENT_AXIS           0x02    /* joystick moved */
/// #define JS_EVENT_INIT           0x80    /* initial state of device */
/// struct js_event {
///         __u32 time;     /* event timestamp in milliseconds */
///         __s16 value;    /* value */
///         __u8 type;      /* event type */
///         __u8 number;    /* axis/button number */
/// };
/// ...
/// ```
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct js_event_c {
    /* 手柄事件发生事件 */
    pub time: __u32,
    /* 手柄数值变化 */
    pub value: __s16,
    /* 手柄事件类型:初始化|轴事件|按钮事件 */
    pub event: __u8,
    /* 轴/按钮-编号 */
    pub number: __u8,
}
