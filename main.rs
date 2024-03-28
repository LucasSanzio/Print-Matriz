use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn somar_matriz(matriz: Vec<Vec<i64>>) -> (i64, Vec<(usize, usize)>, i64) {
    let mut soma = 0;
    let mut indices_utilizados = Vec::new();
    let mut linha:usize = 1;
    let mut coluna:usize = 11;
    let mut contador = 0;

    while linha <= 5 {
        while coluna <= 11 {
            soma += matriz[linha][coluna];
            indices_utilizados.push((linha, coluna));
            contador += 1;
            coluna += 1;
        }
        linha += 1;
        coluna = 12 - linha;
    }

    while linha >= 6 && linha <= 11 { coluna = linha + 1;
        while coluna <= 11 {
            soma += matriz[linha][coluna];
            indices_utilizados.push((linha, coluna));
            contador += 1;
            coluna += 1;
        }
        linha += 1;

    }
            coluna += 1;

        linha += 1;
        coluna += 0;

    (soma, indices_utilizados, contador)
}

fn imprimir_matriz(matriz: &Vec<Vec<i64>>, indices_utilizados: &Vec<(usize, usize)>) {
    for (i, linha) in matriz.iter().enumerate() {
        for (j, &valor) in linha.iter().enumerate() {
            if indices_utilizados.contains(&(i, j)) {
                print!("x ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
}

fn main() {
    let mut o: String = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i64>> = serde_json::from_reader(reader).unwrap();
    let (soma, indices_utilizados, contador) = somar_matriz(matriz.clone());

    println!("Digite a operação desejada: ");
    print!("Soma S ou Média M => ");
    stdin().read_line(&mut o).unwrap();
    let o = o.trim();

    if o == "S" {
        println!("Soma: {}", soma);
    } else {
        println!("Média: {}", soma / contador);
    }

    println!("Matriz:");
    imprimir_matriz(&matriz, &indices_utilizados);
}
