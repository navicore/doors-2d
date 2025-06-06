use crate::room::room_component::RoomState;
use crate::room::CurrentFloorPlan;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use iyes_perf_ui::entry::PerfUiEntry;
use iyes_perf_ui::prelude::*;

#[derive(Component)]
pub struct GameWorldMonitor;

#[derive(Component)]
pub struct SystemMonitor;

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct RoomDoorCount {
    pub label: String,
    pub threshold_highlight: Option<u32>,
    pub color_gradient: ColorGradient,
    pub sort_key: i32,
}

impl Default for RoomDoorCount {
    fn default() -> Self {
        Self {
            label: String::new(),
            threshold_highlight: Some(10),
            #[allow(clippy::unwrap_used)]
            color_gradient: ColorGradient::new_preset_gyr(1.0, 5.0, 10.0).unwrap(),
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for RoomDoorCount {
    type Value = u32;
    type SystemParam = SRes<RoomState>;

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Doors in this room"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        room_state: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        #[allow(clippy::cast_possible_truncation)]
        Some(room_state.doors.len() as u32)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{value} edges")
    }

    // (optional) called every frame to determine if a custom color should be used for the value
    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        #[allow(clippy::cast_precision_loss)]
        self.color_gradient.get_color_for_value(*value as f32)
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight.is_some_and(|t| (*value) > t)
    }
}

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct WorldNodeCount {
    pub label: String,
    pub threshold_highlight: Option<u32>,
    pub color_gradient: ColorGradient,
    pub sort_key: i32,
}

impl Default for WorldNodeCount {
    fn default() -> Self {
        Self {
            label: String::new(),
            threshold_highlight: Some(500),
            #[allow(clippy::unwrap_used)]
            color_gradient: ColorGradient::new_preset_gyr(10.0, 200.0, 500.0).unwrap(),
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for WorldNodeCount {
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
        #[allow(clippy::cast_possible_truncation)]
        plan.floorplan
            .as_ref()
            .map(|plan| plan.get_world_size().0 as u32)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{value} nodes")
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        #[allow(clippy::cast_precision_loss)]
        self.color_gradient.get_color_for_value(*value as f32)
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(&self, value: &Self::Value) -> bool {
        self.threshold_highlight.is_some_and(|t| (*value) > t)
    }
}

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct WorldEdgeCount {
    pub label: String,
    pub threshold_highlight: Option<u32>,
    pub color_gradient: ColorGradient,
    pub sort_key: i32,
}

impl Default for WorldEdgeCount {
    fn default() -> Self {
        Self {
            label: String::new(),
            threshold_highlight: Some(500),
            #[allow(clippy::unwrap_used)]
            color_gradient: ColorGradient::new_preset_gyr(20.0, 400.0, 800.0).unwrap(),
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for WorldEdgeCount {
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
        #[allow(clippy::cast_possible_truncation)]
        plan.floorplan
            .as_ref()
            .map(|plan| plan.get_world_size().1 as u32)
    }

    fn format_value(&self, value: &Self::Value) -> String {
        format!("{value} edges")
    }

    // (optional) Called every frame to determine if a custom color should be used for the value
    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        #[allow(clippy::cast_precision_loss)]
        self.color_gradient.get_color_for_value(*value as f32)
    }

    // (optional) Called every frame to determine if the value should be highlighted
    fn value_highlight(&self, value: &Self::Value) -> bool {
        #[allow(clippy::cast_possible_truncation)]
        self.threshold_highlight.is_some_and(|t| (*value) > t)
    }
}

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct TimeSinceLastFloorplanRefresh {
    /// The label text to display, to allow customization
    pub label: String,
    /// Should we display units?
    pub display_units: bool,
    /// Highlight the value if it goes above this threshold
    pub threshold_highlight: Option<f32>,
    /// Support color gradients!
    pub color_gradient: ColorGradient,
    /// Width for formatting the string
    pub digits: u8,
    /// Precision for formatting the string
    pub precision: u8,

    /// Required to ensure the entry appears in the correct place in the Perf UI
    pub sort_key: i32,
}

impl Default for TimeSinceLastFloorplanRefresh {
    fn default() -> Self {
        Self {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(60.0),
            #[allow(clippy::unwrap_used)]
            color_gradient: ColorGradient::new_preset_gyr(10.0, 45.0, 60.0).unwrap(),
            digits: 3,
            precision: 2,
            // get the correct value from the library
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for TimeSinceLastFloorplanRefresh {
    type Value = f64;
    type SystemParam = (SRes<Time>, SRes<CurrentFloorPlan>);

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Floorplan Refresh"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        (time, plan): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - plan.refreshed;
        Some(d.as_secs_f64())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut s = iyes_perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        #[allow(clippy::cast_possible_truncation)]
        self.color_gradient.get_color_for_value(*value as f32)
    }

    fn value_highlight(&self, value: &Self::Value) -> bool {
        #[allow(clippy::cast_possible_truncation)]
        self.threshold_highlight
            .is_some_and(|t| (*value as f32) > t)
    }
}

#[derive(Component)]
#[require(PerfUiRoot)]
pub struct TimeSinceLastFloorplanModified {
    pub label: String,
    pub display_units: bool,
    pub threshold_highlight: Option<f32>,
    pub color_gradient: ColorGradient,
    pub digits: u8,
    pub precision: u8,
    pub sort_key: i32,
}

impl Default for TimeSinceLastFloorplanModified {
    fn default() -> Self {
        Self {
            label: String::new(),
            display_units: true,
            threshold_highlight: Some(800.0),
            #[allow(clippy::unwrap_used)]
            color_gradient: ColorGradient::new_preset_gyr(60.0, 120.0, 800.0).unwrap(),
            digits: 3,
            precision: 2,
            sort_key: iyes_perf_ui::utils::next_sort_key(),
        }
    }
}

impl PerfUiEntry for TimeSinceLastFloorplanModified {
    type Value = f64;
    type SystemParam = (SRes<Time>, SRes<CurrentFloorPlan>);

    fn label(&self) -> &str {
        if self.label.is_empty() {
            "Floorplan Modified"
        } else {
            &self.label
        }
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn update_value(
        &self,
        (time, plan): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let d = time.elapsed() - plan.modified;
        Some(d.as_secs_f64())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        let mut s = iyes_perf_ui::utils::format_pretty_float(self.digits, self.precision, *value);
        if self.display_units {
            s.push_str(" s");
        }
        s
    }

    fn value_color(&self, value: &Self::Value) -> Option<Color> {
        #[allow(clippy::cast_possible_truncation)]
        self.color_gradient.get_color_for_value(*value as f32)
    }

    fn value_highlight(&self, value: &Self::Value) -> bool {
        #[allow(clippy::cast_possible_truncation)]
        self.threshold_highlight
            .is_some_and(|t| (*value as f32) > t)
    }
}
