use crate::{
    object::Object,
    object_info::{ObjectHandle, ObjectInfo, TypedObjectHandle},
    static_root::StaticRoot,
};

#[derive(Default)]
/// An object pool that provides the backing object storage for executors.
pub struct ObjectPool {
    objects: Vec<Option<ObjectInfo>>,
    object_idx_pool: Vec<usize>,
    alloc_count: usize,
}

impl ObjectPool {
    pub fn new() -> ObjectPool {
        ObjectPool {
            objects: vec![Some(ObjectInfo::new(Box::new(StaticRoot::new())))],
            object_idx_pool: vec![],
            alloc_count: 0,
        }
    }

    /// Pins an object to the pool.
    pub fn allocate(&mut self, mut inner: Box<dyn Object>) -> usize {
        inner.initialize(self);

        let id = if let Some(id) = self.object_idx_pool.pop() {
            id
        } else {
            let objects = &mut self.objects;
            objects.push(None);
            objects.len() - 1
        };
        self.objects[id] = Some(ObjectInfo::new(inner));

        self.alloc_count += 1;

        id
    }

    #[allow(dead_code)]
    pub fn deallocate(&mut self, id: usize) {
        let objects = &mut self.objects;
        let pool = &mut self.object_idx_pool;

        assert!(objects[id].is_some());

        objects[id] = None;
        pool.push(id);
    }

    /// Gets a handle to the object at `id`.
    ///
    /// The handle can be passed around safely and
    /// the underlying object will not be garbage
    /// collected until all handles to it are released.
    ///
    /// If the object pool gets destroyed before
    /// all handles are dropped, the process will be
    /// aborted because of memory unsafety introduced
    /// by reference invalidation.
    pub fn get<'a>(&self, id: usize) -> ObjectHandle<'a> {
        self.objects[id].as_ref().unwrap().handle()
    }

    /// Gets a direct reference to the object at `id`.
    pub fn get_direct(&self, id: usize) -> &dyn Object {
        self.objects[id].as_ref().unwrap().as_object()
    }

    /// Gets a direct typed reference to the object at `id`.
    /// If downcast fails, `None` is returned.
    pub fn get_direct_typed<T: 'static>(&self, id: usize) -> Option<&T> {
        self.get_direct(id).as_any().downcast_ref::<T>()
    }

    /// Gets a direct reference to the object at `id`.
    /// If downcast fails, this raises a `RuntimeError`.
    pub fn must_get_direct_typed<T: 'static>(&self, id: usize) -> &T {
        self.get_direct_typed(id)
            .unwrap_or_else(|| panic!("Type mismatch"))
    }

    /// Gets a typed object handle to the object at `id`.
    /// If downcast fails, `None` is returned.
    pub fn get_typed<'a, T: 'static>(&self, id: usize) -> Option<TypedObjectHandle<'a, T>> {
        TypedObjectHandle::downcast_from(self.get(id))
    }

    /// Gets a typed object handle to the object at `id`.
    /// If downcast fails, this raises a `RuntimeError`.
    pub fn must_get_typed<'a, T: 'static>(&self, id: usize) -> TypedObjectHandle<'a, T> {
        self.get_typed(id)
            .unwrap_or_else(|| panic!("Type mismatch"))
    }

    pub fn get_static_root<'a>(&self) -> TypedObjectHandle<'a, StaticRoot> {
        self.get_typed(0).unwrap()
    }

    pub fn get_direct_static_root(&self) -> &StaticRoot {
        self.get_direct_typed(0).unwrap()
    }

    pub fn get_alloc_count(&self) -> usize {
        self.alloc_count
    }

    pub fn reset_alloc_count(&mut self) {
        self.alloc_count = 0;
    }
}

impl Drop for ObjectPool {
    fn drop(&mut self) {
        for obj in &mut self.objects {
            if let Some(ref mut obj) = *obj {
                obj.gc_notify();
            }
        }
    }
}
