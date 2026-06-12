fn main() {
    let mut x = 5;
    println!("La valeur de x est : {}", x);
    x = 6;
    println!("La valeur de x est : {}", x);
    const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;
    println!(
        "La valeur de constante TROIS_HEURES_EN_SECONDES est : {}",
        TROIS_HEURES_EN_SECONDES
    );

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("La valeur de y dans la portée interne est : {}", y);
    }

    println!("La valeur de y est : {}", y);
}
