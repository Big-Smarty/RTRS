pub mod vulkan_context;

pub trait RenderContext: Default {}

pub struct Context<T>
where
    T: RenderContext,
{
    subcontext: T,
}

impl<T> Context<T>
where
    T: RenderContext,
{
    pub fn subcontext(self) -> T {
        self.subcontext
    }
}

impl<T> Default for Context<T>
where
    T: RenderContext,
{
    fn default() -> Self {
        Self {
            subcontext: T::default(),
        }
    }
}

impl<T> RenderContext for Context<T> where T: RenderContext {}
