

use nalgebra::{Point3, Translation, Vector3};
use ndarray::prelude::*;
use kiss3d::{self, window,scene::SceneNode};
use plotters::style::GREEN;
use std::{fmt, vec};

#[derive(Debug)]
enum NonTriangleError {
    NotTriangle,
    Other(String),
}

impl fmt::Display for NonTriangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NonTriangleError::NotTriangle => write!(f, "Valor inválido fornecido"),
            NonTriangleError::Other(msg) => write!(f, "Erro: {}", msg),
        }
    }
}

impl std::error::Error for NonTriangleError {}
#[derive(Debug)]
struct Ponto3D{
    x:f64,
    y:f64,
    z:f64
}
struct Triangulo{
    a:f64,
    b:f64,
    c:f64
}
impl fmt::Display for Ponto3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ponto3D {{ x: {:.4}, y: {:.4}, z: {:.4} }}", self.x, self.y, self.z)
    }
}
impl Ponto3D {
    fn new(x:f64,y:f64,z:f64)->Ponto3D{
        Ponto3D { x, y, z }
    }
    fn to_string(&self,ponto:&Ponto3D)->String{
        format!("Ponto ({},{},{})",ponto.x,ponto.y,ponto.z)
    }
    fn distancia(&self,p1:&Ponto3D,p2:&Ponto3D)->f64{
        ((p2.x-p1.x)*(p2.x-p1.x)+(p2.y-p1.y)*(p2.y-p1.y)+(p2.z-p1.z)*(p2.z-p1.z)).sqrt()
    }
    fn area_heron(&self,p1:&Ponto3D,p2:&Ponto3D,p3:&Ponto3D)->f64{
        let a= self.distancia(&p1,&p2);
        let b= self.distancia(&p2,&p3);
        let c= self.distancia(&p1,&p3);
        let p= (a+b+c)/2.0;
        let area=(p*(p-a)*(p-b)*(p-c)).sqrt();
        area
    }
    fn centro_massa(&self,p1:&Ponto3D,p2:&Ponto3D,p3:&Ponto3D)->Ponto3D{
        let xc=(p1.x+p2.x+p3.x)/3.0;
        let yc=(p1.y+p2.y+p3.y)/3.0;
        let zc=(p1.z+p2.z+p3.z)/3.0;
        Ponto3D { x: xc, y: yc, z: zc }
    }
    fn vetor_normal_unitario(&self,p1:&Ponto3D,p2:&Ponto3D,p3:&Ponto3D)->Result<Vector3<f64>,NonTriangleError>{
        
        let v2= Vector3::from_vec(vec![p3.x-p1.x,p3.y-p1.y,p3.z-p1.z]);
        let v1=Vector3::from_vec(vec![p2.x-p1.x,p2.y-p1.y,p2.z-p1.z]);
        let cross=v1.cross(&v2);
        let norma= cross.norm();
        if norma==0.0 {
            return Err(NonTriangleError::NotTriangle)
        }
        else {
            Ok(cross/norma)
        }
    }
    fn imprime_info_triangulo(p1:&Ponto3D,p2:&Ponto3D,p3:&Ponto3D) {
        let area=p1.area_heron(p1, p2, p3);
        let centro=p1.centro_massa(p1,p2,p3);
        let normal=p1.vetor_normal_unitario(p1, p2, p3).unwrap();
        println!("P1({}),\nP2({}),\nP3({})\nÁrea:{:.4}\nCentro de massa:{}\nvetor normal unitário:{:?}",p1,p2,p3,area,centro,normal);
    }
}
#[tokio::main]
async fn main(){
    let mut janela=kiss3d::window::Window::new("");
    janela.set_light(kiss3d::light::Light::StickToCamera);
    let p1 = Ponto3D::new(0.0, 0.0, 0.0);
    let p2 = Ponto3D::new(1.0, 0.0, 0.0);
    let p3 = Ponto3D::new(0.0, 1.0, 0.0);
    let baricentro=p1.centro_massa(&p1, &p2, &p3);
    let normal=p1.vetor_normal_unitario(&p1, &p2, &p3).unwrap();
    Ponto3D::imprime_info_triangulo( &p1, &p2, &p3);
    let P1=Point3::new(p1.x as f32,p1.y as f32,p1.z as f32);
    let P2=Point3::new(p2.x as f32,p2.y as f32,p2.z as f32);
    let P3=Point3::new(p3.x as f32,p3.y as f32,p3.z as f32);
    let centro=Point3::new(baricentro.x as f32,baricentro.y as f32,baricentro.z as f32);
    let ponta=Point3::new((normal.x+baricentro.x) as f32, (normal.y+baricentro.y) as f32, (normal.z+baricentro.z) as f32);
    let mut Baricentro=janela.add_sphere(0.01);
        Baricentro.set_color(0.0,1.0,0.0);
        Baricentro.set_local_translation(Translation::from(centro.coords));
        
    while janela.render().await{
        janela.draw_line(&P1, &P2, &Point3::new(1.0,1.0,1.0));
        janela.draw_line(&P3, &P2, &Point3::new(1.0,1.0,1.0));
        janela.draw_line(&P1, &P3, &Point3::new(1.0,1.0,1.0));
        janela.draw_line(&centro, &ponta,&Point3::new(1.0,0.0,0.0));
        
    }

}