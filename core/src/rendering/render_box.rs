use super::RenderObject;
use crate::constraints::BoxConstraints;
use stretch::geometry::Size;

pub trait RenderBox {
    /// Determines the set of render objects located at the given position.
    ///
    /// Returns true, and adds any render objects that contain the point to the
    /// given hit test result, if this render object or one of its descendants
    /// absorbs the hit (preventing objects below this one from being hit).
    /// Returns false if the hit can continue to other objects below this one.
    ///
    /// The caller is responsible for transforming [position] from global
    /// coordinates to its location relative to the origin of this [RenderBox].
    /// This [RenderBox] is responsible for checking whether the given position
    /// is within its bounds.
    ///
    /// If transforming is necessary, [BoxHitTestResult.addWithPaintTransform],
    /// [BoxHitTestResult.addWithPaintOffset], or
    /// [BoxHitTestResult.addWithRawTransform] need to be invoked by the caller
    /// to record the required transform operations in the [HitTestResult].
    /// These methods will also help with applying the transform to
    /// `position`.
    ///
    /// Hit testing requires layout to be up-to-date but does not require
    /// painting to be up-to-date. That means a render object can rely upon
    /// [performLayout] having been called in [hit_test] but cannot rely
    /// upon [paint] having been called. For example, a render object might
    /// be a child of a [RenderOpacity] object, which calls [hit_test] on
    /// its children when its opacity is zero even through it does not
    /// [paint] its children.
    fn hit_test(&mut self, result: BoxHitTestResult, pos: Offset) {
        if _size.contains(position) {
            if hitTestChildren(result, pos) || hitTestSelf(position) {
                result.add(BoxHitTestEntry(this, position));
                return true;
            }
        }
    }

    /// Override this method if this render object can be hit even if its
    /// children were not hit.
    ///
    /// The caller is responsible for transforming [position] from global
    /// coordinates to its location relative to the origin of this [RenderBox].
    /// This [RenderBox] is responsible for checking whether the given position
    /// is within its bounds.
    ///
    /// Used by [hitTest]. If you override [hitTest] and do not call this
    /// function, then you don't need to implement this function.
    fn hit_test_self(&mut self, pos: Offset) -> bool {
        false
    }

    /// Override this method to check whether any children are located at the
    /// given position.
    ///
    /// Typically children should be hit-tested in reverse paint order so that
    /// hit tests at locations where children overlap hit the child that is
    /// visually "on top" (i.e., paints later).
    ///
    /// The caller is responsible for transforming [position] from global
    /// coordinates to its location relative to the origin of this [RenderBox].
    /// This [RenderBox] is responsible for checking whether the given position
    /// is within its bounds.
    ///
    /// If transforming is necessary, [HitTestResult.addWithPaintTransform],
    /// [HitTestResult.addWithPaintOffset], or
    /// [HitTestResult.addWithRawTransform] need to be invoked by the caller
    /// to record the required transform operations in the [HitTestResult].
    /// These methods will also help with applying the transform to
    /// `position`.
    ///
    /// Used by [hitTest]. If you override [hitTest] and do not call this
    /// function, then you don't need to implement this function.
    fn hit_test_children(&mut self, pos: Offset) -> bool {
        false
    }

    /// Override this method to handle pointer events that hit this render
    /// object.
    ///
    /// For [RenderBox] objects, the `entry` argument is a [BoxHitTestEntry].
    /// From this object you can determine the [PointerDownEvent]'s position
    /// in local coordinates. (This is useful because
    /// [PointerEvent.position] is in global coordinates.)
    ///
    /// If you override this, consider calling [debugHandleEvent] as follows, so
    /// that you can support [debugPaintPointersEnabled]:
    ///
    /// ```dart
    /// @override
    /// void handleEvent(PointerEvent event, HitTestEntry entry) {
    ///   assert(debugHandleEvent(event, entry));
    ///   // ... handle the event ...
    /// }
    /// ```
    fn handle_event(&mut self, event: PointEvent, entry: Box<HitTestEntry>) {}
}

#[derive(Debug)]
pub struct RenderBoxObject<R: RenderBox> {
    inner: R,
}

impl<R> RenderObject for RenderBoxObject<R>
where
    R: RenderBox,
{
    type Constraints = BoxConstraints;
}
