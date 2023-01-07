//Definimos la estructura persona
struct Persona {
    nombre: String,
    edad: u8,
}
//Definimos el constructor de la estructura
fn crear_persona(nombre: &str, edad: u8)->Persona{
    let persona: Persona=Persona{
        nombre: nombre.to_string(),
        edad: edad,
    };
    return persona;
}

fn main() {
//Llamamos al constructor
let persona1= crear_persona("Juan", 23);
println!("Edad: {}", persona1.edad);
println!("Nombre: {}", persona1.nombre);
}
