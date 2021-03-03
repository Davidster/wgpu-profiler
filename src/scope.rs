use crate::{GpuProfiler, ProfilerCommandRecorder};

/// Scope that takes a (mutable) reference to the encoder/pass.
/// Calls end_scope on Drop.
pub struct Scope<'a, W: ProfilerCommandRecorder> {
    profiler: &'a mut GpuProfiler,
    recorder: &'a mut W,
}

/// Scope that takes ownership of the encoder/pass.
/// Calls end_scope on Drop.
pub struct OwningScope<'a, W: ProfilerCommandRecorder> {
    profiler: &'a mut GpuProfiler,
    recorder: W,
}

/// Scope that takes a (mutable) reference to the encoder/pass.
/// Does NOT call end_scope on Drop.
/// This construct is just for completeness in cases where working with scopes is preferred but one can't rely on the Drop call in the right place.
pub struct ManualOwningScope<'a, W: ProfilerCommandRecorder> {
    profiler: &'a mut GpuProfiler,
    recorder: W,
}

impl<'a, W: ProfilerCommandRecorder> Scope<'a, W> {
    /// Starts a new profiler scope. Scope is closed on drop.
    pub fn start(profiler: &'a mut GpuProfiler, recorder: &'a mut W, device: &wgpu::Device, label: &str) -> Self {
        profiler.begin_scope(label, recorder, device);
        Self { profiler, recorder }
    }

    /// Starts a scope nested within this one.
    pub fn scope(&mut self, device: &wgpu::Device, label: &str) -> Scope<'_, W> {
        Scope::start(self.profiler, self.recorder, device, label)
    }
}

impl<'a, W: ProfilerCommandRecorder> OwningScope<'a, W> {
    /// Starts a new profiler scope. Scope is closed on drop.
    pub fn start(profiler: &'a mut GpuProfiler, mut recorder: W, device: &wgpu::Device, label: &str) -> Self {
        profiler.begin_scope(label, &mut recorder, device);
        Self { profiler, recorder }
    }

    /// Starts a scope nested within this one.
    pub fn scope(&mut self, device: &wgpu::Device, label: &str) -> Scope<'_, W> {
        Scope::start(self.profiler, &mut self.recorder, device, label)
    }
}

impl<'a, W: ProfilerCommandRecorder> ManualOwningScope<'a, W> {
    /// Starts a new profiler scope. Scope is NOT closed on drop and needs to be closed manually with [`ManualOwningScope.end_scope`]
    pub fn start(profiler: &'a mut GpuProfiler, mut recorder: W, device: &wgpu::Device, label: &str) -> Self {
        profiler.begin_scope(label, &mut recorder, device);
        Self { profiler, recorder }
    }

    /// Starts a scope nested within this one
    pub fn scope(&mut self, device: &wgpu::Device, label: &str) -> Scope<'_, W> {
        Scope::start(self.profiler, &mut self.recorder, device, label)
    }

    /// Ends the scope allowing the extraction of owned the ProfilerCommandRecorder
    /// and the mutable reference to the GpuProfiler.
    pub fn end_scope(mut self) -> (W, &'a mut GpuProfiler) {
        self.profiler.end_scope(&mut self.recorder);
        (self.recorder, self.profiler)
    }
}
impl<'a> Scope<'a, wgpu::CommandEncoder> {
    /// Start a render pass wrapped in a OwningScope.
    pub fn scoped_render_pass<'b>(
        &'b mut self,
        device: &wgpu::Device,
        label: &str,
        pass_descriptor: &wgpu::RenderPassDescriptor<'b, '_>,
    ) -> OwningScope<'b, wgpu::RenderPass<'b>> {
        let render_pass = self.recorder.begin_render_pass(pass_descriptor);
        OwningScope::start(self.profiler, render_pass, device, label)
    }

    /// Start a compute pass wrapped in a OwningScope.
    pub fn scoped_compute_pass(
        &mut self,
        device: &wgpu::Device,
        label: &str,
        pass_descriptor: &wgpu::ComputePassDescriptor<'_>,
    ) -> OwningScope<wgpu::ComputePass> {
        let compute_pass = self.recorder.begin_compute_pass(pass_descriptor);
        OwningScope::start(self.profiler, compute_pass, device, label)
    }
}

impl<'a> OwningScope<'a, wgpu::CommandEncoder> {
    /// Start a render pass wrapped in an OwningScope.
    pub fn scoped_render_pass<'b>(
        &'b mut self,
        device: &wgpu::Device,
        label: &str,
        pass_descriptor: &wgpu::RenderPassDescriptor<'b, '_>,
    ) -> OwningScope<'b, wgpu::RenderPass<'b>> {
        let render_pass = self.recorder.begin_render_pass(pass_descriptor);
        OwningScope::start(self.profiler, render_pass, device, label)
    }

    /// Start a compute pass wrapped in a OwningScope.
    pub fn scoped_compute_pass(
        &mut self,
        device: &wgpu::Device,
        label: &str,
        pass_descriptor: &wgpu::ComputePassDescriptor<'_>,
    ) -> OwningScope<wgpu::ComputePass> {
        let compute_pass = self.recorder.begin_compute_pass(pass_descriptor);
        OwningScope::start(self.profiler, compute_pass, device, label)
    }
}

impl<'a> ManualOwningScope<'a, wgpu::CommandEncoder> {
    /// Start a render pass wrapped in an OwningScope.
    pub fn scoped_render_pass<'b>(
        &'b mut self,
        device: &wgpu::Device,
        label: &str,
        pass_descriptor: &wgpu::RenderPassDescriptor<'b, '_>,
    ) -> OwningScope<'b, wgpu::RenderPass<'b>> {
        let render_pass = self.recorder.begin_render_pass(pass_descriptor);
        OwningScope::start(self.profiler, render_pass, device, label)
    }

    /// Start a compute pass wrapped in an OwningScope.
    pub fn scoped_compute_pass(
        &mut self,
        device: &wgpu::Device,
        label: &str,
        pass_descriptor: &wgpu::ComputePassDescriptor<'_>,
    ) -> OwningScope<wgpu::ComputePass> {
        let compute_pass = self.recorder.begin_compute_pass(pass_descriptor);
        OwningScope::start(self.profiler, compute_pass, device, label)
    }
}

// Scope
impl<'a, W: ProfilerCommandRecorder> std::ops::Deref for Scope<'a, W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.recorder
    }
}

impl<'a, W: ProfilerCommandRecorder> std::ops::DerefMut for Scope<'a, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.recorder
    }
}

impl<'a, W: ProfilerCommandRecorder> Drop for Scope<'a, W> {
    fn drop(&mut self) {
        self.profiler.end_scope(self.recorder);
    }
}

// OwningScope
impl<'a, W: ProfilerCommandRecorder> std::ops::Deref for OwningScope<'a, W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.recorder
    }
}

impl<'a, W: ProfilerCommandRecorder> std::ops::DerefMut for OwningScope<'a, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.recorder
    }
}

impl<'a, W: ProfilerCommandRecorder> Drop for OwningScope<'a, W> {
    fn drop(&mut self) {
        self.profiler.end_scope(&mut self.recorder);
    }
}

// ManualOwningScope
impl<'a, W: ProfilerCommandRecorder> std::ops::Deref for ManualOwningScope<'a, W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.recorder
    }
}

impl<'a, W: ProfilerCommandRecorder> std::ops::DerefMut for ManualOwningScope<'a, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.recorder
    }
}
