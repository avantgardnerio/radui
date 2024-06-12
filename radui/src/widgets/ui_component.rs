use crate::events::{Signal, SignalType};
use crate::generated::models;
use as_any::AsAny;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, FontId};
use std::slice::{Iter, IterMut};

pub struct DrawContext {
    pub canvas: Canvas<OpenGl>,
    pub font: FontId,
}

pub trait IUIComponent: AsAny {
    fn get_model(&self) -> &models::UIComponent;

    fn get_model_mut(&mut self) -> &mut models::UIComponent;

    fn get_x(&self) -> f64 {
        self.get_model().x.unwrap_or(0.0)
    }

    fn get_y(&self) -> f64 {
        self.get_model().y.unwrap_or(0.0)
    }

    fn set_x(&mut self, x: f64) {
        self.get_model_mut().x = Some(x);
    }

    fn set_y(&mut self, y: f64) {
        self.get_model_mut().y = Some(y);
    }

    fn set_width(&mut self, width: f64) {
        self.get_model_mut().width = Some(width);
        self.get_model_mut().explicit_width = Some(width);
    }

    fn set_height(&mut self, height: f64) {
        self.get_model_mut().height = Some(height);
        self.get_model_mut().explicit_height = Some(height);
    }

    /// Sizes the object.
    ///  Unlike directly setting the <code>width</code> and <code>height</code>
    ///  properties, calling the <code>setActualSize()</code> method
    ///  does not set the <code>explictWidth</code> and
    ///  <code>explicitHeight</code> properties, so a future layout
    ///  calculation can result in the object returning to its previous size.
    ///  This method is used primarily by component developers implementing
    ///  the <code>updateDisplayList()</code> method, by Effects,
    ///  and by the LayoutManager.
    fn set_actual_size(&mut self, width: f64, height: f64) {
        self.get_model_mut().width = Some(width);
        self.get_model_mut().height = Some(height);
    }

    ///  Moves the component to a specified position within its parent.
    ///  Calling this method is exactly the same as
    ///  setting the component's <code>x</code> and <code>y</code> properties.
    ///
    ///  <p>If you are overriding the <code>updateDisplayList()</code> method
    ///  in a custom component, call the <code>move()</code> method
    ///  rather than setting the <code>x</code> and <code>y</code> properties.
    ///  The difference is that the <code>move()</code> method changes the location
    ///  of the component and then dispatches a <code>move</code> event when you
    ///  call the method, while setting the <code>x</code> and <code>y</code>
    ///  properties changes the location of the component and dispatches
    ///  the event on the next screen refresh.</p>
    fn moove(&mut self, x: f64, y: f64) {
        self.get_model_mut().x = Some(x);
        self.get_model_mut().y = Some(y);
    }

    /// Draws the object and/or sizes and positions its children.
    /// This is an advanced method that you might override
    /// when creating a subclass of UIComponent.
    ///
    ///  <p>You do not call this method directly. Flex calls the
    //  <code>updateDisplayList()</code> method when the component is added to a container
    //  using the <code>addChild()</code> method, and when the component's
    //  <code>invalidateDisplayList()</code> method is called. </p>
    //
    //  <p>If the component has no children, this method
    //  is where you would do programmatic drawing
    //  using methods on the component's Graphics object
    //  such as <code>graphics.drawRect()</code>.</p>
    //
    //  <p>If the component has children, this method is where
    //  you would call the <code>move()</code> and <code>setActualSize()</code>
    //  methods on its children.</p>
    fn update_display_list(&mut self, _width: f64, _height: f64) {}

    /// Validates the measured size of the component
    /// If the LayoutManager.invalidateSize() method is called with this ILayoutManagerClient,
    /// then the validateSize() method is called when it's time to do measurements.
    fn validate_size(&mut self, recursive: bool, ctx: &mut DrawContext) {
        if recursive {
            for child in self.get_children_mut() {
                child.validate_size(true, ctx);
            }
        }

        // TODO: invalid_size_flag
        self.measure_sizes(ctx);
    }

    fn measure_sizes(&mut self, ctx: &mut DrawContext) {
        if self.can_skip_measurement() {
            self.get_model_mut().measured_min_width = Some(0.0);
            self.get_model_mut().measured_min_height = Some(0.0);
        } else {
            self.measure(ctx);

            if let Some(emw) = self.get_model().explicit_min_width {
                if self.get_measured_width() < self.get_explicit_min_width() {
                    self.set_measured_width(emw);
                }
            }
            if let Some(emw) = self.get_model().explicit_max_width {
                if self.get_measured_width() > self.get_explicit_min_width() {
                    self.set_measured_width(emw);
                }
            }

            if let Some(emh) = self.get_model().explicit_min_height {
                if self.get_measured_height() < self.get_explicit_min_height() {
                    self.set_measured_height(emh);
                }
            }
            if let Some(emh) = self.get_model().explicit_max_height {
                if self.get_measured_height() > self.get_explicit_max_height() {
                    self.set_measured_height(emh);
                }
            }

            // TODO: computed has_changed and return it
        }
    }

    ///  Determines if the call to the <code>measure()</code> method can be skipped.
    //
    //  @return Returns <code>true</code> when the <code>measureSizes()</code> method can skip the call to
    //  the <code>measure()</code> method. For example this is usually <code>true</code> when both <code>explicitWidth</code> and
    //  <code>explicitHeight</code> are set. For paths, this is <code>true</code> when the bounds of the path
    //  have not changed.
    fn can_skip_measurement(&self) -> bool {
        self.get_model().explicit_width.is_some() && self.get_model().explicit_height.is_some()
    }

    fn measure(&mut self, ctx: &mut DrawContext);

    fn set_measured_width(&mut self, width: f64) {
        self.get_model_mut().measured_width = Some(width);
    }

    fn set_measured_height(&mut self, height: f64) {
        self.get_model_mut().measured_height = Some(height);
    }

    fn get_measured_width(&self) -> f64 {
        self.get_model().measured_width.unwrap()
    }

    fn get_explicit_min_width(&self) -> f64 {
        self.get_model().explicit_min_width.unwrap()
    }

    fn get_explicit_min_height(&self) -> f64 {
        self.get_model().explicit_min_height.unwrap()
    }

    fn get_explicit_max_height(&self) -> f64 {
        self.get_model().explicit_max_height.unwrap()
    }

    fn get_width(&self) -> f64 {
        self.get_model().width.unwrap_or(0.0)
    }

    fn get_height(&self) -> f64 {
        self.get_model().height.unwrap_or(0.0)
    }

    fn get_percent_width(&self) -> Option<f64> {
        self.get_model().percent_width
    }

    fn get_percent_height(&self) -> Option<f64> {
        self.get_model().percent_height
    }

    fn get_measured_height(&self) -> f64 {
        self.get_model().measured_height.unwrap()
    }

    fn get_min_width(&self) -> f64 {
        self.get_model().min_width.unwrap()
    }

    fn get_min_height(&self) -> f64 {
        self.get_model().min_height.unwrap()
    }

    fn get_explicit_or_measured_width(&self) -> f64 {
        if let Some(explicit_width) = self.get_model().explicit_width {
            return explicit_width;
        }
        self.get_measured_width()
    }

    fn get_explicit_or_measured_height(&self) -> f64 {
        if let Some(explicit_height) = self.get_model().explicit_height {
            return explicit_height;
        }
        self.get_measured_height()
    }

    fn add_event_listener(&mut self, _typ: SignalType, _id: Vec<String>) {
        todo!()
    }

    fn get_name(&self) -> Option<&str> {
        self.get_model().id.as_ref().map(|id| id.as_str())
    }

    fn get_id(&self) -> &String {
        self.get_model().uid.as_ref().unwrap()
    }

    fn get_children_mut(&mut self) -> IterMut<'_, Box<dyn IUIComponent>>;

    fn get_children(&self) -> Iter<'_, Box<dyn IUIComponent>>;

    fn draw(&self, ctx: &mut DrawContext) {
        for widget in self.get_children() {
            ctx.canvas.save();
            ctx.canvas.translate(widget.get_x() as f32, widget.get_y() as f32);

            widget.draw(ctx);

            ctx.canvas.restore();
        }
    }

    fn handle_event(&mut self, path: &mut Vec<String>, event: &Signal, dispatch: &mut Box<dyn FnMut(Signal) + '_>) {
        path.push(self.get_id().clone());

        self.get_children_mut().for_each(|widget| widget.handle_event(path, event, dispatch));
        self.handle_own_event(path, event, dispatch);

        path.pop();
    }

    fn handle_own_event(
        &mut self,
        _path: &mut Vec<String>,
        _event: &Signal,
        _dispatch: &mut Box<dyn FnMut(Signal) + '_>,
    ) {
    }

    fn find_by_name(&mut self, id: &str) -> Option<&mut Box<dyn IUIComponent>> {
        for widget in self.get_children_mut() {
            if Some(id) == widget.get_name() {
                return Some(widget);
            }
            if let Some(child) = widget.find_by_name(id) {
                return Some(child);
            }
        }
        None
    }
}
