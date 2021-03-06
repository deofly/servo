/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUAdapterBinding::GPULimits;
use crate::dom::bindings::codegen::Bindings::GPURenderPipelineBinding::GPURenderPipelineMethods;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object, DomObject, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::USVString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::gpubindgrouplayout::GPUBindGroupLayout;
use dom_struct::dom_struct;
use std::string::String;
use webgpu::{WebGPUBindGroupLayout, WebGPURenderPipeline};

#[dom_struct]
pub struct GPURenderPipeline {
    reflector_: Reflector,
    label: DomRefCell<Option<USVString>>,
    render_pipeline: WebGPURenderPipeline,
    bind_group_layouts: Vec<WebGPUBindGroupLayout>,
}

impl GPURenderPipeline {
    fn new_inherited(
        render_pipeline: WebGPURenderPipeline,
        label: Option<USVString>,
        bgls: Vec<WebGPUBindGroupLayout>,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            render_pipeline,
            bind_group_layouts: bgls,
        }
    }

    pub fn new(
        global: &GlobalScope,
        render_pipeline: WebGPURenderPipeline,
        label: Option<USVString>,
        bgls: Vec<WebGPUBindGroupLayout>,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPURenderPipeline::new_inherited(
                render_pipeline,
                label,
                bgls,
            )),
            global,
        )
    }
}

impl GPURenderPipeline {
    pub fn id(&self) -> WebGPURenderPipeline {
        self.render_pipeline
    }
}

impl GPURenderPipelineMethods for GPURenderPipeline {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<USVString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<USVString>) {
        *self.label.borrow_mut() = value;
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpupipelinebase-getbindgrouplayout
    fn GetBindGroupLayout(&self, index: u32) -> Fallible<DomRoot<GPUBindGroupLayout>> {
        if index > self.bind_group_layouts.len() as u32 || index > GPULimits::empty().maxBindGroups
        {
            return Err(Error::Range(String::from("Index out of bounds")));
        }
        return Ok(GPUBindGroupLayout::new(
            &self.global(),
            self.bind_group_layouts[index as usize],
            None,
        ));
    }
}
