fn funcion_objetivo(x: f64)->f64{
    return x*x - 2.0;
}

fn biseccion(a: f64, b: f64, tol: f64, maximo: i32) -> f64{
    let mut x0=a;
    let mut x1=b;
    let mut iteraciones =0;
    while iteraciones < maximo{
        let x2=(x0+x1)/2.0;
        if funcion_objetivo(x2).abs() < tol {
            return x2;
        }
        if funcion_objetivo(x0)*funcion_objetivo(x2)<0.00000000000 {
            x1=x2;
        }else {
            x0=x2;
        }
        iteraciones=iteraciones+1
    }
    return x1;
}
fn main() {
     println!("{}",biseccion(1.0,2.0,0.0000001,100000000));
}
