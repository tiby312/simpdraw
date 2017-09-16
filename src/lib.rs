


pub trait Vertex: std::default::Default+std::clone::Clone{
    fn set_pos(&mut self,x:f32,y:f32);
    fn set_alpha(&mut self,val:f32);
}


pub trait Draw{
    fn get_num_verts(&self)->usize;

    //As input, given verticies used from last time.
    //so do not have to update every property.
    fn update_verts<T:Vertex>(&self,fa:&mut [T]);
}




pub struct Wrap<T:Draw>{
    start_index:usize,
    pub a:T

    
}
pub struct Drawer<V:Vertex>{
    verts:Vec<V>
}

impl<V:Vertex> Drawer<V>{
    pub fn new()->Drawer<V>{
        Drawer{verts:Vec::new()}
    }

    pub fn add<T:Draw>(&mut self,aa:T)->Wrap<T>{
        let a=aa.get_num_verts();
        let curlen=self.verts.len();
        
        self.verts.resize(curlen+a,Default::default());
    
        Wrap{start_index:curlen,a:aa}
    }

    pub fn get_verts(&self)->&[V]{
        &self.verts
    }

    pub fn update_verts<T:Draw>(&mut self,a:&Wrap<T>){
        
        let range=&mut self.verts[a.start_index..a.start_index+a.a.get_num_verts()];
        a.a.update_verts(range);
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
