//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use oak_core::sync::OnceCell;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::*,
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable},
        tss::TaskStateSegment,
    },
};

struct Descriptors {
    gdt: GlobalDescriptorTable,
    kernel_cs_selector: SegmentSelector,
    kernel_ds_selector: SegmentSelector,
    user_cs_selector: SegmentSelector,
    user_ds_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

static TSS: TaskStateSegment = TaskStateSegment::new();
static DESCRIPTORS: OnceCell<Descriptors> = OnceCell::new();

/// Initializes the Global Descriptor Table.
///
/// This function will panic if the GDT is already initialized.
pub fn init_gdt() {
    let mut descriptors = Descriptors {
        gdt: GlobalDescriptorTable::new(),
        kernel_cs_selector: SegmentSelector::NULL,
        kernel_ds_selector: SegmentSelector::NULL,
        user_cs_selector: SegmentSelector::NULL,
        user_ds_selector: SegmentSelector::NULL,
        tss_selector: SegmentSelector::NULL,
    };
    descriptors.kernel_cs_selector = descriptors.gdt.add_entry(Descriptor::kernel_code_segment());
    descriptors.kernel_ds_selector = descriptors.gdt.add_entry(Descriptor::kernel_data_segment());
    descriptors.user_cs_selector = descriptors.gdt.add_entry(Descriptor::user_code_segment());
    descriptors.user_ds_selector = descriptors.gdt.add_entry(Descriptor::user_data_segment());
    descriptors.tss_selector = descriptors.gdt.add_entry(Descriptor::tss_segment(&TSS));

    // Make sure the GDT was not previously initialized.
    if DESCRIPTORS.set(descriptors).is_err() {
        panic!("gdt is already initialized");
    }

    let descriptors = DESCRIPTORS.get().unwrap();
    descriptors.gdt.load();

    // Safety: it's safe to load these segments as we've initialized the GDT just above.
    unsafe {
        CS::set_reg(descriptors.kernel_cs_selector);
        DS::set_reg(descriptors.kernel_ds_selector);
        ES::set_reg(descriptors.kernel_ds_selector);
        FS::set_reg(descriptors.kernel_ds_selector);
        GS::set_reg(descriptors.kernel_ds_selector);
        SS::set_reg(descriptors.kernel_ds_selector);
        load_tss(descriptors.tss_selector);
    }
}