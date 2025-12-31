use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
use crate::gpu::{GpuContext, CaGpuResources, Uniforms};
use crate::ca;
use crate::ui::UiState;

pub struct App {
    gpu: GpuContext,
    ca_resources: CaGpuResources,
    egui_renderer: egui_wgpu::Renderer,
    egui_state: egui_winit::State,
    egui_ctx: egui::Context,
    ui_state: UiState,
    window: Arc<Window>,
    grid_width: u32,
    grid_height: u32,
    tile_cols: u32,
    tile_rows: u32,
    frame_count: u64,
}

pub struct EventResponse {
    pub consumed: bool,
}

impl App {
    pub async fn new(window: Arc<Window>) -> Self {
        let gpu = GpuContext::new(window.clone()).await;
        
        let tile_cols = 4u32;
        let tile_rows = 4u32;
        let cells_per_tile = 128u32;
        
        let grid_width = tile_cols * cells_per_tile;
        let grid_height = tile_rows * cells_per_tile;
        
        let initial_cells = ca::generate_initial_cells(grid_width, grid_height, 0.3);
        let rules = ca::generate_general_rules(tile_cols * tile_rows);
        
        let ca_resources = CaGpuResources::new(
            &gpu.device,
            gpu.config.format,
            grid_width,
            grid_height,
            tile_cols,
            tile_rows,
            &initial_cells,
            &rules,
        );
        
        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        
        let egui_renderer = egui_wgpu::Renderer::new(
            &gpu.device,
            gpu.config.format,
            None,
            1,
            false,
        );
        
        Self {
            gpu,
            ca_resources,
            egui_renderer,
            egui_state,
            egui_ctx,
            ui_state: UiState::default(),
            window,
            grid_width,
            grid_height,
            tile_cols,
            tile_rows,
            frame_count: 0,
        }
    }
    
    pub fn handle_event(&mut self, event: &WindowEvent) -> EventResponse {
        let response = self.egui_state.on_window_event(&self.window, event);
        EventResponse { consumed: response.consumed }
    }
    
    pub fn handle_keyboard(&mut self, event: &KeyEvent) {
        if !event.state.is_pressed() { return; }
        
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Space) => {
                self.ui_state.paused = !self.ui_state.paused;
            }
            PhysicalKey::Code(KeyCode::KeyR) => {
                self.randomize();
            }
            PhysicalKey::Code(KeyCode::KeyT) => {
                self.ui_state.use_totalistic = !self.ui_state.use_totalistic;
                self.update_uniforms();
            }
            _ => {}
        }
    }
    
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.gpu.resize(size);
    }
    
    pub fn update(&mut self) {
        if self.ui_state.paused {
            return;
        }
        
        if self.frame_count % self.ui_state.speed_divisor == 0 {
            self.ca_resources.step(&self.gpu.device, &self.gpu.queue, self.grid_width, self.grid_height);
            self.ca_resources.update_render_bind_group(&self.gpu.device);
        }
        
        self.frame_count += 1;
    }
    
    pub fn render(&mut self) {
        let output = match self.gpu.surface.get_current_texture() {
            Ok(t) => t,
            Err(_) => {
                self.gpu.surface.configure(&self.gpu.device, &self.gpu.config);
                return;
            }
        };
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let raw_input = self.egui_state.take_egui_input(&self.window);
        
        let ui_state = &mut self.ui_state;
        let grid_width = self.grid_width;
        let grid_height = self.grid_height;
        let tile_cols = self.tile_cols;
        let tile_rows = self.tile_rows;
        
        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            egui::Window::new("Controls")
                .default_pos([10.0, 10.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button(if ui_state.paused { "Play" } else { "Pause" }).clicked() {
                            ui_state.paused = !ui_state.paused;
                        }
                        
                        if ui.button("Randomize").clicked() {
                            ui_state.should_randomize = true;
                        }
                    });
                    
                    ui.separator();
                    
                    if ui.checkbox(&mut ui_state.use_totalistic, "Totalistic rules (T)").changed() {
                        ui_state.totalistic_changed = true;
                    }
                    
                    ui.add(egui::Slider::new(&mut ui_state.speed_divisor, 1..=60).text("Speed"));
                    
                    ui.separator();
                    
                    ui.label(format!("Grid: {}x{}", grid_width, grid_height));
                    ui.label(format!("Tiles: {}x{}", tile_cols, tile_rows));
                    ui.label("Space: pause | R: randomize | T: toggle totalistic");
                });
        });
        
        self.egui_state.handle_platform_output(&self.window, full_output.platform_output);
        
        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.gpu.config.width, self.gpu.config.height],
            pixels_per_point: full_output.pixels_per_point,
        };
        
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.gpu.device, &self.gpu.queue, *id, image_delta);
        }
        
        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        self.egui_renderer.update_buffers(
            &self.gpu.device,
            &self.gpu.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("CA Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            render_pass.set_pipeline(&self.ca_resources.render_pipeline);
            render_pass.set_bind_group(0, &self.ca_resources.render_bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }
        
        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            let mut render_pass = render_pass.forget_lifetime();
            self.egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }
        
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
        
        if self.ui_state.should_randomize {
            self.ui_state.should_randomize = false;
            self.randomize();
        }
        
        if self.ui_state.totalistic_changed {
            self.ui_state.totalistic_changed = false;
            self.update_uniforms();
        }
    }
    
    fn randomize(&mut self) {
        let initial_cells = ca::generate_initial_cells(self.grid_width, self.grid_height, 0.3);
        let rules = if self.ui_state.use_totalistic {
            ca::generate_totalistic_rules(self.tile_cols * self.tile_rows)
        } else {
            ca::generate_general_rules(self.tile_cols * self.tile_rows)
        };
        
        self.gpu.queue.write_buffer(
            &self.ca_resources.cell_buffers[0],
            0,
            bytemuck::cast_slice(&initial_cells),
        );
        self.gpu.queue.write_buffer(
            &self.ca_resources.cell_buffers[1],
            0,
            bytemuck::cast_slice(&initial_cells),
        );
        self.gpu.queue.write_buffer(
            &self.ca_resources.rule_buffer,
            0,
            bytemuck::cast_slice(&rules),
        );
    }
    
    fn update_uniforms(&mut self) {
        let uniforms = Uniforms {
            grid_width: self.grid_width,
            grid_height: self.grid_height,
            tile_cols: self.tile_cols,
            tile_rows: self.tile_rows,
            tile_width: self.grid_width / self.tile_cols,
            tile_height: self.grid_height / self.tile_rows,
            use_totalistic: if self.ui_state.use_totalistic { 1 } else { 0 },
            _padding: 0,
        };
        
        self.gpu.queue.write_buffer(
            &self.ca_resources.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniforms]),
        );
    }
}
