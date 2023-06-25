fn main() {
    comparando_numero();
    multiplos_if();
    expressao_let_if();

    loop {
        println!("novamente");
        break;
    }

    let mut numero = 3;
    while numero != 0 {
        println!("{numero}!");
        numero = numero -1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;
    
    // Poderiamos usar a.len()
    while indice < 5 {
        println!("O valor é: {}", a[indice]);
        indice = indice + 1;
    }

    for elemento in a.iter() {
        println!("O valor do elemento é {elemento}");
    }


    for numero in (1..4).rev() {
        println!("{numero}!");
    }
    println!("LIFTOFF!!!");

}


fn comparando_numero() {
    let numero = 5;
    if numero < 5 {
        println!("condição era verdadeira ({numero} < 5)");
    } else {
        println!("condição era falsa ({numero} >= 5)");
    }

}

fn multiplos_if() {
    let numero = 8;

    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não pe divisível por 4, 3 ou 2");
    }
}

fn expressao_let_if() {
    let condicao = false;
    let numero = if condicao {
        5
    } else {
       6 
    };
    println!("O valor do número é: {numero}");
}
