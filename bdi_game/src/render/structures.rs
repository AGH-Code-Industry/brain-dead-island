/// Groups objects to be rendered. 
pub struct Cluster<T>{

    // list of objects inside cluster.
    pub objects : Vec<T>,

}

/// Backend agnostic functions.
impl <T>Cluster<T>{

    /// Adds unit to the cluster.
    pub fn add(&mut self, unit : T){

        self.objects.push(unit);

    }

    /// Adds units to the cluster.
    pub fn bulk_add(&mut self, units : &[T]){

    }

    pub fn new() -> Self{
        Self{
            objects : Vec::new()
        }
    }

}
