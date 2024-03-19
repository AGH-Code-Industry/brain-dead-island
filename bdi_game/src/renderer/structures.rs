/// Describes properties of the rendered object.
pub struct Unit{

}

/// Groups objects to be rendered. 
pub struct Cluster{

    // list of objects inside cluster.
    objects : Vec<Unit>,

}

/// Backend agnostic functions.
impl Cluster{

    /// Adds unit to the cluster.
    pub fn add(unit : Unit){

    }

    /// Adds units to the cluster.
    pub fn bulk_add(units : &[Unit]){

    }

}
