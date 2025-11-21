use plotters::{prelude::*, style::full_palette::{GREEN_300, GREEN_900}};
fn main() ->Result<(),Box<dyn std::error::Error>>{
    let mut v_celso:Vec<f32>=vec![];
    for t in 0..=10 {
        let v= ((9.8 * (85.0))/12.5)*(1.0 - (-(12.5/85.0)*t as f32).exp());
        v_celso.push(v);
        println!("v({}) = {} [m/s]",t,v);
    }
    let binding = (0..=10).map(|x| x as f32).collect::<Vec<f32>>();
    println!("{:?}",binding);
    let pontos=binding.iter().cloned().zip(v_celso.iter().cloned());
    
   let root = BitMapBackend::new("yan.png",(640,489)).into_drawing_area();
   root.fill(&WHITE)?;
   let mut chart=ChartBuilder::on(&root)
   .caption("Yan", ("arial",50))
   .margin(10)
   .x_label_area_size(30)
   .y_label_area_size(30)
   .build_cartesian_2d(-0.0f32..10.0f32, -0.0f32..60.0f32)?;
chart.configure_mesh().draw()?;
chart.draw_series(LineSeries::new(pontos.clone(),&GREEN))?;
chart.draw_series(PointSeries::of_element(pontos, 2, &GREEN_300, &|(x, y), s, st| {
            EmptyElement::at((x, y))
                + Circle::new((0, 0), s, st.filled())
        }))?;
   Ok(())
}
