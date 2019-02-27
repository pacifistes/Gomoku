

use crate::WidgetIds;
use crate::models::homepage::HomepageModel;
use crate::views::homepage::HomepageView;
use conrod::UiCell;
use std::collections::HashMap;
use conrod::widget::id::Id;

pub struct HomepageController {
    pub view: HomepageView,
    pub model: HomepageModel,
    events: HashMap<Id, fn()>
}

impl HomepageController {
    pub fn new(widget_ids: &WidgetIds) -> HomepageController {
        let model = HomepageModel::new();
        let view = HomepageView::new();
        let mut controller = HomepageController {
            view,
            model,
            events: HashMap::new(),
        };
        controller.set_events(widget_ids);
        controller
    }

    fn set_events(&mut self, widget_ids: &WidgetIds) {
        self.events.insert(widget_ids.button_player_vs_player, || {
            println!("click on button 'player vs player'");
        });

        self.events.insert(widget_ids.button_player_vs_ia, || {
            println!("click on button 'player vs ia'");
        });

        self.events.insert(widget_ids.toggle_button_weight_boxes, || {
            println!("click on toggle button");
        });

        self.events.insert(widget_ids.dropdown_button_deph, || {
            println!("click on dropdown button");
        });
    }

    pub fn show(&self, ui: &mut UiCell, widget_ids: &WidgetIds) {
        self.view.display_canvas(ui, widget_ids);
        self.view.display_button_vs_player(ui, widget_ids, self.events.get(&widget_ids.button_player_vs_player).unwrap());
        self.view.display_button_vs_ia(ui, widget_ids, self.events.get(&widget_ids.button_player_vs_ia).unwrap());
        self.view.display_toggle_button(ui, widget_ids, self.events.get(&widget_ids.toggle_button_weight_boxes).unwrap());
        self.view.display_dropdown_button_deph(ui, widget_ids, self.events.get(&widget_ids.dropdown_button_deph).unwrap());
    }
}