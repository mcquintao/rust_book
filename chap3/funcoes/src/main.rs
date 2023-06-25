fn main() {
    println!("Hello, world!");
    outra_funcao();
    funcao_com_parametro(5);
    funcao_com_multiplos_parametros(10, 'M');

    let y = {
        let x = 3;
        x + 1
    };

    println!("O valor de y é {}", y);
    let cinco = cinco();
    println!("A função cinco retornou {cinco}");
    let z = soma_um(10);
    println!("O retorno de soma_um com parâmetro 10 é: {z}");
}

// Imprime "outra funcao" na tela
fn outra_funcao() {
    println!("Outra função!");
}

// Imprime o parãmetro na tela
fn funcao_com_parametro(x : i32) {
    println!("O valor de x é : {x}");
}

//Imprime 2 parâmetros na tela
fn funcao_com_multiplos_parametros(x :i32, unidade : char) {
    println!("O parametro x é {x} e a unidade é {unidade}");
}

// Retorna 5
fn cinco() -> i32 {
    5
}

// retorna +1
fn soma_um(num: i32) -> i32 {
    num + 1
}
