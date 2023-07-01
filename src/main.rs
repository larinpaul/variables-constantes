fn main() {

    let x = 3;
    println!("El valor de x es: {}", x);
    // x = 7; // error
    println!("El valor de x es: {}", x);

    let mut xx = 3;
    println!("El valor de xx es: {}", xx);
    xx = 7;
    println!("El valor de xx es: {}", xx);

    const XXX: f32 = 6.66;
    // A diferencia de las variables no pueden llevar la palabra mut y hay que
    // definir su tipo de dato obligatoriamente.
    // Se pueden declarar en cualquier ámbito, incluso global.
    // Sólo pueden almacenar un valor, no el resultado de un cálculo.
    println!("El valor de XXX es: {}", XXX);

    //// Redefioniendo variables (shadowing)
    // Podemos volver a crear una misma variable, para poder asignarle un nuevo valor,
    // incluso un tipo istinto.
    let xxxx = 42;
    println!("El valor de xxxx es: {}",xxxx);
    let xxxx = "La cuestion del universo y todo...";
    println!("El valor de xxxx es: {}",xxxx);

}
