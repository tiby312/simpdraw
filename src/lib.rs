


pub trait Vertex{
    fn set_pos(&mut self,x:f32,y:f32);
    fn set_alpha(&mut self,val:f32);
}


pub trait Draw{
    fn get_num_verts(&self)->usize;

    //As input, given verticies used from last time.
    //so do not have to update every property.
    fn update_verts<T:Vertex>(&self,fa:&mut [T]);
}



/*
pub struct Wrap<T:Draw>{
    id:usize,
    pub a:T
}
pub struct Drawer{
    vecs:Vec<(Vec<Vertex>,glium::VertexBuffer<Vertex>)>
}

impl Drawer{
    pub fn new()->Drawer{
        Drawer{vecs:Vec::new()}
    }

    pub fn add<T:Draw>(&mut self, display: &glium::Display,aa:T)->Wrap<T>{
        let a=aa.get_num_verts();

        let mut v=Vec::new();
        v.resize(a,Vertex{position:[0.0,0.0],color:[0.0,0.0,0.0,0.0]});
    
        self.vecs.push((v,(glium::VertexBuffer::empty_dynamic(display, a).unwrap())));

        Wrap{id:self.vecs.len()-1,a:aa}
    }
    pub fn draw<T:Draw>(&mut self,a:&Wrap<T>){
        let z=&mut self.vecs[a.id];
        //let (&mut x,&mut y)=*z;
        a.a.update_verts(&mut z.0);
        z.1.write(&mut z.0);
    }
}*/




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
