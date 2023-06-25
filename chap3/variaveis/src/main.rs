const THREE_HOURS_IN_SEC : u32 = 60 * 60 * 3;

use std::io;

fn main() {
    let x : u32 = 5;
    println!("O valor de x é : {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("O valor de x no escopo interno é : {x}")
    }

    println!("O valor de x é : {x}");


    let espacos = "    ";
    let espacos = espacos.len();
    println!("número de espaços: {espacos}");

    let x_float = 10.5; //f64;
    let x_int = 10; //i32;
                    //
                    //

    let t = true;
    let f : bool = false;

    let c = 'z'; //char;
    let relogio = '\u{2705}';
    println!("emoji relógio: {relogio}");

    { // Tuplas
        let tup : (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("O valores são x = {x}, y = {y} e z = {z}");
        println!("O primeiro valor de tup é {}", tup.0);
        println!("O segundo valor de tup é {}", tup.1);
    }
    { // Matrizes (Arrays)
       let a =  [1, 2, 3, 4 ,5];
       let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

       println!("O terceiro mês é {}", meses[2]);
    }
    {
       let a =  [1, 2, 3, 4 ,5];
       println!("Por favor, insira um index da matriz");

       let mut index = String::new();

       io::stdin()
           .read_line(&mut index)
           .expect("Falha ao ler entrada");

       let index : usize = index
           .trim()
           .parse()
           .expect("Falha ao converter a entrada");

        let elemento = a[index];

        println!("O valor do elemento de index {index} é : {elemento}");
    }

}
