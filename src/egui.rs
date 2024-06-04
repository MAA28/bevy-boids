use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Legend, Line, Plot, PlotBounds, PlotPoint, PlotPoints};

use crate::{
    AlignmentGizmo, Behaviors, CohesionGizmo, FpsHistory, PhysicsGizmo, SeperationGizmo,
    SteeringGizmo,
};

pub fn egui_system(
    mut context: EguiContexts,
    mut config_store: ResMut<GizmoConfigStore>,
    mut behaviors: ResMut<Behaviors>,
    mut fps_history: ResMut<FpsHistory>,
    time: Res<Time>,
) {
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

    egui::Window::new("Strength Controller").show(context.ctx_mut(), |ui| {
        ui.add(
            egui::Slider::new(&mut behaviors.seek_mouse_strength, 0.0..=10.0)
                .text("Seek Mouse Strength"),
        );
        ui.add(
            egui::Slider::new(&mut behaviors.border_strength, 0.0..=10.0).text("Border Strength"),
        );
        ui.add(
            egui::Slider::new(&mut behaviors.separation_strength, 0.0..=10.0)
                .text("Seperation Strength"),
        );
        ui.add(
            egui::Slider::new(&mut behaviors.cohesion_strength, 0.0..=10.0)
                .text("Cohesion Strength"),
        );
        ui.add(
            egui::Slider::new(&mut behaviors.alignment_strength, 0.0..=10.0)
                .text("Alignment Strength"),
        );
    });

    egui::Window::new("Radius Controller").show(context.ctx_mut(), |ui| {
        ui.add(
            egui::Slider::new(&mut behaviors.seperation_radius, 0.0..=1000.0)
                .text("Seperation Radius"),
        );
        ui.add(
            egui::Slider::new(&mut behaviors.cohesion_radius, 0.0..=1000.0).text("Cohesion Radius"),
        );
        ui.add(
            egui::Slider::new(&mut behaviors.alignment_radius, 0.0..=1000.0)
                .text("Alignment Radius"),
        );
    });

    egui::Window::new("FPS History").show(context.ctx_mut(), |ui| {
       if time.delta_seconds() <= 0.0001 {
           return;
       } 
        let fps: f64 = (1.0 / time.delta_seconds()).into();
        fps_history.history.push(fps);
        fps_history.min = fps.min(fps_history.min);
        fps_history.max = fps.max(fps_history.max);

        let points = fps_history
            .history
            .iter()
            .enumerate()
            .map(|(i, fps1)| [i as f64, *fps1 as f64])
            .collect::<PlotPoints>();

        let line = Line::new(points);

        Plot::new("FPS")
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .show(ui, |plot_ui| {
                (
                    plot_ui.line(line),
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                        [(fps_history.history.len() as f64 - 200.0).max(0.0), fps_history.min - 15.0],
                        [fps_history.history.len() as f64, fps_history.max + 15.0],
                    )),
                )
            });
    });
}
