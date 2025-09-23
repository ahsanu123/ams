pub trait BaseCommandTrait {}

// Basic Extension Method Like

pub trait WriteLogFromCommandTrait {
    fn hello(self);
}

impl<T> WriteLogFromCommandTrait for T
where
    T: BaseCommandTrait,
{
    fn hello(self) {
        todo!()
    }
}

struct ExtensionMethodLike;

impl BaseCommandTrait for ExtensionMethodLike {}

impl ExtensionMethodLike {
    fn h(self) {
        self.hello();
    }
}
