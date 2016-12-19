extern crate cocoa;
extern crate core_graphics;

use self::cocoa::base::id;
use self::cocoa::appkit::NSView;
use self::core_graphics::geometry::{CGPoint, CGRect, CGSize};

// view - NSView
pub fn resize_to_superview(view: id) {
    let bounds = unsafe { view.superview().bounds() };

    let origin = CGPoint::new(0.0, 0.0);
    let size = CGSize::new(bounds.size.width, bounds.size.height);
    let frame = CGRect::new(&origin, &size);

    unsafe {
        msg_send![
            view,
            setFrame:frame
        ];
    }
}
