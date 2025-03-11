use std::sync::Arc;

use vulkano::VulkanLibrary;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

use super::RenderContext;

pub struct VulkanContext {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    allocator: Arc<StandardMemoryAllocator>,
}

impl VulkanContext {
    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }

    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }

    pub fn queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }

    pub fn create_buffer<T>(
        &self,
        data: T,
        buffer_create_info: BufferCreateInfo,
        allocation_create_info: AllocationCreateInfo,
    ) -> Subbuffer<T>
    where
        T: BufferContents,
    {
        Buffer::from_data(
            self.allocator.clone(),
            buffer_create_info,
            allocation_create_info,
            data,
        )
        .expect("Failed to create buffer")
    }
}

impl Default for VulkanContext {
    fn default() -> Self {
        trace!("Initializing vulkan lib");
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");

        trace!("Initializing instance");
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        trace!("Getting physical device");
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");
        debug!(
            "Selected physical device: {}",
            physical_device.properties().device_name
        );

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");
        let queue = queues.next().unwrap();

        let allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        Self {
            instance,
            device,
            queue,
            allocator,
        }
    }
}

impl RenderContext for VulkanContext {}
