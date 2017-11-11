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

pub struct RangeID{
    start_index:usize,
    length:usize
}

pub struct Drawer<V:Vertex>{
    verts:Vec<V>
}

impl<V:Vertex> Drawer<V>{

    #[inline(always)]
    pub fn new()->Drawer<V>{
        Drawer{verts:Vec::new()}
    }

    #[inline(always)]
    pub fn add(&mut self,length:usize)->RangeID{
        
        let curlen=self.verts.len();
        
        self.verts.resize(curlen+length,Default::default());
    
        RangeID{start_index:curlen,length:length}
    }

    #[inline(always)]
    pub fn get_all_ranges(&self)->&[V]{
        &self.verts
    }

    #[inline(always)]
    pub fn get_range_mut<'a>(&'a mut self,a:&'a RangeID)->&'a mut [V]{
        &mut self.verts[a.start_index..a.start_index+a.length]
    }
}