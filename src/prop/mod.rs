mod node;
pub use node::*;

mod edge;
pub use edge::*;

pub trait StorageProp {
    type Prop;

    fn storage_prop(&self) -> &Self::Prop;
}

pub trait StoragePropMut: StorageProp {
    fn storage_prop_mut(&mut self) -> &mut Self::Prop;
}

pub trait AddStorageProp: StorageProp {
    fn add_storage_prop(&mut self, prop: Self::Prop);
}

pub trait DelStorageProp: StorageProp {
    fn del_storage_prop(&mut self) -> Self::Prop;
}
