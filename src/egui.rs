use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{AlignmentGizmo, Behaviors, CohesionGizmo, PhysicsGizmo, SeperationGizmo, SteeringGizmo};

pub fn egui_system(mut context: EguiContexts, mut config_store: ResMut<GizmoConfigStore>, mut behaviors: ResMut<Behaviors>) {
    egui::Window::new("Gizmo Controller").show(context.ctx_mut(), |ui| {
        let (physics, _) = config_store.config_mut::<PhysicsGizmo>();
        ui.checkbox(&mut (physics.enabled), "Physics Gizmo");
        let (steering, _) = config_store.config_mut::<SteeringGizmo>();
        ui.checkbox(&mut (steering.enabled), "Steering Gizmo");
        let (alignment, _) = config_store.config_mut::<AlignmentGizmo>();
        ui.checkbox(&mut (alignment.enabled), "Alignment Gizmo");
        let (seperation, _) = config_store.config_mut::<SeperationGizmo>();
        ui.checkbox(&mut (seperation.enabled), "Seperation Gizmo");
        let (cohesion, _) = config_store.config_mut::<CohesionGizmo>();
        ui.checkbox(&mut (cohesion.enabled), "Cohesion Gizmo");
    });
    
    egui::Window::new("Behavior Controller").show(context.ctx_mut(), |ui| {
        ui.checkbox(&mut behaviors.seek_mouse, "Seek Mouse");
        ui.checkbox(&mut behaviors.alignment, "Alignment");
        ui.checkbox(&mut behaviors.separation, "Separation");
        ui.checkbox(&mut behaviors.cohesion, "Cohesion");
    });
}
