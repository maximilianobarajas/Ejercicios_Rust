fn factorial(numero: u64) ->u64 {
   if numero==1{
    return numero;
   } else {
    return numero * factorial(numero-1);
   }

}

fn main() {
println!("{}", factorial(10));
}
