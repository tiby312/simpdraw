//!
//! A simple vertex memory manager. 
//!
//! Users can allocate a range of verticies and look up them to modify them later.
//! Provides a function to return a slice of all the verticies combined
//! so that they may all be sent to the gpu to be drawn in batch.
//!
//!
//!

pub trait Vertex: std::default::Default+std::clone::Clone{
    fn set_pos(&mut self,x:f32,y:f32);
    fn set_alpha(&mut self,val:f32);
}


pub trait Draw{
    fn get_num_verts<Z>(&self,extra:&Z)->usize;
}




pub struct Wrap{
    start_index:usize,
    length:usize
}

pub struct Drawer<V:Vertex>{
    verts:Vec<V>
}

impl<V:Vertex> Drawer<V>{
    pub fn new()->Drawer<V>{
        Drawer{verts:Vec::new()}
    }

    pub fn add(&mut self,length:usize)->Wrap{
        
        let curlen=self.verts.len();
        
        self.verts.resize(curlen+length,Default::default());
    
        Wrap{start_index:curlen,length:length}
    }

    pub fn get_all_verts(&self)->&[V]{
        &self.verts
    }

    pub fn get_verts_mut<'a>(&'a mut self,a:&'a Wrap)->&'a mut [V]{
        &mut self.verts[a.start_index..a.start_index+a.length]
    }
}