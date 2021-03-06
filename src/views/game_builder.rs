use crate::controllers::game_builder::GameBuilderEvent;
use crate::models::game_builder::*;
use crate::WidgetIds;
use conrod::*;

pub struct GameBuilderView {
}

#[rustfmt::skip]
impl GameBuilderView {
    pub fn new() -> GameBuilderView {
        GameBuilderView {
        }
    }

    pub fn display_canvas(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        widget::Canvas::new()
            .w(500.0)
            .border(0.0)
            .middle_of(widget_ids.window_canvas)
            .down_from(widget_ids.title, 50.0)
            .color(color::TRANSPARENT)
            .set(widget_ids.game_builder_canvas, ui);
    }


    pub fn display_button_start(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameBuilderEvent, model: &mut GameBuilder, start_text: &str) {
        if widget::Button::new()
            .w_of(widget_ids.game_builder_canvas)
            .h(75.0)
            .middle_of(widget_ids.game_builder_canvas)
            .down_from(widget_ids.title, 50.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label(start_text)
            .set(widget_ids.button_start, ui)
            .was_clicked() {
            if let GameBuilderEvent::ButtonStart(event) = event {
                event(model);
            }
        }
    }

    pub fn display_dropdown_button_game_mode(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameBuilderEvent, model: &mut GameBuilder) {
        if let Some(mode_index) = widget::DropDownList::new(&model.game_modes, Some(model.mode_index))
        .w_of(widget_ids.game_builder_canvas)
        .h(75.0)
        .middle_of(widget_ids.game_builder_canvas)
        .down_from(widget_ids.button_start, 25.0)
        .max_visible_items(4)
        .color(color::LIGHT_BROWN)
        .border(1.0)
        .scrollbar_on_top()
        .set(widget_ids.dropdown_button_game_mode, ui) {
            if let GameBuilderEvent::DropdownButtonGameMode(event) = event {
                event(model, mode_index);
            }
        }
    }

    pub fn display_toggle_button(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameBuilderEvent, model: &mut GameBuilder) {
        let label = format!("Display boxes's weight: {}", if model.display_weight {"yes"} else {"no"});
        if let Some(value) = widget::Toggle::new(model.display_weight)
            .w_of(widget_ids.game_builder_canvas)
            .h(75.0)
            .middle_of(widget_ids.game_builder_canvas)
            .down_from(widget_ids.dropdown_button_game_mode, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label(&label[..])
            .set(widget_ids.toggle_button_weight_boxes, ui)
            .last() {
            if let GameBuilderEvent::ToggleButtonWeightBoxes(event) = event {
                event(model, value);
            }
        }
    }

    pub fn display_number_dialer_first_ia_depth(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameBuilderEvent, model: &mut GameBuilder) {
       if let Some(new_depth) = widget::NumberDialer::new(model.first_ia_depth, model.min_depth, model.max_depth, 0)
            .w_of(widget_ids.game_builder_canvas)
            .h(75.0)
            .middle_of(widget_ids.game_builder_canvas)
            .down_from(widget_ids.toggle_button_weight_boxes, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("First Ia depth: ")
            .set(widget_ids.number_dialer_first_ia_depth, ui) {
            if let GameBuilderEvent::NumberDialerFirstIaDepth(event) = event {
                event(model, new_depth);
            }
        }
    }

    pub fn display_number_dialer_second_ia_depth(&self, ui: &mut UiCell, widget_ids: &WidgetIds, event: &GameBuilderEvent, model: &mut GameBuilder) {
       if let Some(new_depth) = widget::NumberDialer::new(model.second_ia_depth, model.min_depth, model.max_depth, 0)
            .w_of(widget_ids.game_builder_canvas)
            .h(75.0)
            .middle_of(widget_ids.game_builder_canvas)
            .down_from(widget_ids.number_dialer_first_ia_depth, 25.0)
            .color(color::LIGHT_BROWN)
            .border(1.0)
            .label("Second Ia depth: ")
            .set(widget_ids.number_dialer_second_ia_depth, ui) {
            if let GameBuilderEvent::NumberDialerSecondIaDepth(event) = event {
                event(model, new_depth);
            }
        }
    }
}