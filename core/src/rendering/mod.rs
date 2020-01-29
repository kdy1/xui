pub use self::render_box::{RenderBox, RenderBoxObject};
use crate::Constraints;

mod render_box;

pub trait RenderObject {
    type Constraints: Constraints;

    /// Mark this render object's layout information as dirty, and either
    /// register this object with its [PipelineOwner], or defer to the
    /// parent, depending on whether this object is a relayout boundary or
    /// not respectively.
    ///
    /// ## Background
    ///
    /// Rather than eagerly updating layout information in response to writes
    /// into a render object, we instead mark the layout information as
    /// dirty, which schedules a visual update. As part of the visual
    /// update, the rendering pipeline updates the render object's layout
    /// information.
    ///
    /// This mechanism batches the layout work so that multiple sequential
    /// writes are coalesced, removing redundant computation.
    ///
    /// If a render object's parent indicates that it uses the size of one of
    /// its render object children when computing its layout information,
    /// this function, when called for the child, will also mark the parent
    /// as needing layout. In that case, since both the parent and the child
    /// need to have their layout recomputed, the pipeline owner is only
    /// notified about the parent; when the parent is laid out, it will call
    /// the child's [layout] method and thus the child will be laid out as
    /// well.
    ///
    /// Once [markNeedsLayout] has been called on a render object,
    /// [debugNeedsLayout] returns true for that render object until just after
    /// the pipeline owner has called [layout] on the render object.
    ///
    /// ## Special cases
    ///
    /// Some subclasses of [RenderObject], notably [RenderBox], have other
    /// situations in which the parent needs to be notified if the child is
    /// dirtied (e.g., if the child's intrinsic dimensions or baseline changes).
    /// Such subclasses override markNeedsLayout and either call
    /// `super.markNeedsLayout()`, in the normal case, or call
    /// [markParentNeedsLayout], in the case where the parent needs to be laid
    /// out as well as the child.
    ///
    /// If [sizedByParent] has changed, calls
    /// [markNeedsLayoutForSizedByParentChange] instead of [markNeedsLayout].
    fn mark_needs_layout(&mut self);

    /// Whether the constraints are the only input to the sizing algorithm (in
    /// particular, child nodes have no impact).
    ///
    /// Returning false is always correct, but returning true can be more
    /// efficient when computing the size of this render object because we don't
    /// need to recompute the size if the constraints don't change.
    ///
    /// Typically, subclasses will always return the same value. If the value
    /// can change, then, when it does change, the subclass should make sure
    /// to call [markNeedsLayoutForSizedByParentChange].
    fn sized_by_paren(&self) -> bool {
        sizedByParent
    }

    /// Do the work of computing the layout for this render object.
    ///
    /// Do not call this function directly: call [layout] instead. This function
    /// is called by [layout] when there is actually work to be done by this
    /// render object during layout. The layout constraints provided by your
    /// parent are available via the [constraints] getter.
    ///
    /// If [sizedByParent] is true, then this function should not actually
    /// change the dimensions of this render object. Instead, that work
    /// should be done by [performResize]. If [sizedByParent] is false, then
    /// this function should both change the dimensions of this render
    /// object and instruct its children to layout.
    ///
    /// In implementing this function, you must call [layout] on each of your
    /// children, passing true for parentUsesSize if your layout information is
    /// dependent on your child's layout information. Passing true for
    /// parentUsesSize ensures that this render object will undergo layout if
    /// the child undergoes layout. Otherwise, the child can change its
    /// layout information without informing this render object.
    fn perform_layout(&self, constraints: &Self::Constraints);

    /// Whether this render object repaints separately from its parent.
    ///
    /// Override this in subclasses to indicate that instances of your class
    /// ought to repaint independently. For example, render objects that
    /// repaint frequently might want to repaint themselves without
    /// requiring their parent to repaint.
    ///
    /// If this getter returns true, the [paintBounds] are applied to this
    /// object and all descendants. The framework automatically creates an
    /// [OffsetLayer] and assigns it to the [layer] field. Render objects
    /// that declare themselves as repaint boundaries must not replace the
    /// layer created by the framework.
    ///
    /// Warning: This getter must not change value over the lifetime of this
    /// object.
    fn is_repaint_boundary() -> bool {
        false
    }

    /// Override this method to handle pointer events that hit this render
    /// object.
    fn handle_event(&mut self, event: PointEvent, entry: Box<HitTestEntry>) {}
}
