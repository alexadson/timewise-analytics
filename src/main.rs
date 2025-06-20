mod regressao;
use regressao::{regressao_linear, prever};

fn main() {
    let dados = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Série temporal simples

    let modelo = regressao_linear(&dados);
    println!("y = {:.2}x + {:.2}", modelo.a, modelo.b);

    let proximo = dados.len() as f64;
    let predicao = prever(&modelo, proximo);
    println!("Previsão para o próximo período: {:.2}", predicao);
}
