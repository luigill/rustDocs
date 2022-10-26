//printing é feito por uma série de macros definidos em std::fmt, alguns deles são:
//format! = Escreve texto formatado em string
//print! = mesmo que o format, mas o texto é printado no console.
//println! = mesmo que print!, mas pula uma linha
//eprint! = mesmo que print!, mas o texto é printado como erro
//eprintln! = mesmo que eprint!, mas pula uma linha

fn main() {
    //os {} serão substituídos por argumentos. Eles serão stringuificados.
    println!("{} dias, {} horas", 31, 24);

    //Pode-se usar argumentos posicionais. Especificar um inteiro dentro dos {}
    // determina qual argumento será utilizado na posição.
    println!("{0}, esse é o {1}. {1}, essa é a {0}", "Alice", "Bob");

    //Argumentos podem receber apelidos também.
    println!(
        "{dia} de {mes} de {ano}",
        dia = "Dezoito",
        mes = "Dezembro",
        ano = "Dois mil"
    );

    //Diferentes tipos de formatação podem ser invocados especificando-se um
    // caracter de formatação depois de : .
    println!("Base 10 :              {}", 69420); //base 10 = vazio
    println!("Base 2 (binário):      {:b}", 69420); //binário = :b
    println!("Base 8 (octal):        {:o}", 69420); //octal = :o
    println!("Base 16 (hexadecimal): {:x}", 69420); //hexadecimal = :x
    println!("Base 16 (hexadecimal): {:X}", 69420); //hexadecimanl (CAPS) = :X

    //É possível alinhar texto com um width específico.
    //Da maneira abaixo, teria-se 4 espaços em branco e o número 1 na quinta posição.ssssss
    println!("{number:>5}", number = 1);

    //É possível também preencher um número com 0s extras.
    println!("{number:0>5}", number = 1);

    //Pode-se também apelidar os formatadores utilizando o símbolo $
    println!("{number:0>width$}", number = 1, width = 5);

    //Nas versões mais recentes do Rust, é possível utilizar variáveis como
    //argumentos de formatação.
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
