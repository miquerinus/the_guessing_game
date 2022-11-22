// - biblioteca padrão de entrada e saida
use std::io;
// - biblioteca padrão onde contem os metodos de geração de numeros aleatorios
use rand::Rng;
// - biblioteca padrão utilizada para comparação entre 2 valores
use std::cmp::Ordering;

fn main() {
    println!("Guess my number: ");

    // - declaração variavel secret_number
    // - chamada da função thread_rng)() -> gera que nos da o numero aleatorio particular do
    // - sistema operacional
    // - chamada do metodo gen_range() ->  metodo de numeros aleatórios_
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        // - cria uma variavel de palpite mutavel
        // - chama a função STRING.NEW - a função NEW cria uma string vazia
        let mut guess = String::new();

        // - chamada a biblioteca de entrada e saida - std::io pela função stdin faz leitura da entrada fazendo referencia &mutavel
        // - ocorre uma chamada ao metodo read_line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input !");

        // -variavel guess de 32 bit sem sinal (somente positivos) para usar a tecnica de SOMBREAMENTO - reutilização
        // - TRIM -> metodo usado para apagar espaços vazios no inicio e fim
        // - PARSE -> metodo usado para fazer conversões de tipos
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // - Compara o conteudo da variavel guess com o conteudo da variavel secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Oh no, too small !!"),
            Ordering::Greater => println!("Oh no, too big !!"),
            Ordering::Equal => {
                println!("Congratulations, you win :) !! ");
                break;
            }
        }
    }
}
