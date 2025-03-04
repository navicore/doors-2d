use crate::room::CurrentFloorPlan;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use iyes_perf_ui::entry::PerfUiEntry;
use iyes_perf_ui::prelude::*;

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct PerfUiWorldNodeCount {
    pub label: String,
    pub threshold_highlight: Option<u32>,
    pub color_gradient: ColorGradient,
    pub sort_key: i32,
}

impl Default for PerfUiWorldNodeCount {
    fn default() -> Self {
        PerfUiWorldNodeCount {
            label: String::new(),
            threshold_highlight: Some(500),
            color_gradient: ColorGradient::new_preset_gyr(10.0, 200.0, 500.0).unwrap(),
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiWorldNodeCount {
    type Value = u32;
    type SystemParam = SRes<CurrentFloorPlan>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Rooms"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        plan: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        plan.floorplan
            .as_ref()
            .map(|plan| plan.get_world_size().0 as u32)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{value} nodes")
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight
            .map(|t| (*value) > t)
            .unwrap_or(false)
    }
}

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct PerfUiWorldEdgeCount {
    pub label: String,
    pub threshold_highlight: Option<u32>,
    pub color_gradient: ColorGradient,
    pub sort_key: i32,
}

impl Default for PerfUiWorldEdgeCount {
    fn default() -> Self {
        PerfUiWorldEdgeCount {
            label: String::new(),
            threshold_highlight: Some(500),
            color_gradient: ColorGradient::new_preset_gyr(20.0, 400.0, 800.0).unwrap(),
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for PerfUiWorldEdgeCount {
    type Value = u32;
    type SystemParam = SRes<CurrentFloorPlan>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Doors"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        plan: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        plan.floorplan
            .as_ref()
            .map(|plan| plan.get_world_size().1 as u32)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{value} edges")
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        self.color_gradient.get_color_for_value(*value as f32)
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight
            .map(|t| (*value) > t)
            .unwrap_or(false)
    }
}
