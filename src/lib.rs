


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

    pub fn add<T:Draw,Z>(&mut self,length:usize)->Wrap{
        
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




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
