// Importa o módulo `regressao` (local)
mod regressao;

// Importa funções específicas e a struct Coeficientes para uso no main
use regressao::{regressao_linear, calcular_mse, calcular_r2, prever};

fn main() {
    // Exemplo de uma série temporal de valores (ex: vendas mensais, temperaturas, etc.)
    let dados = vec![2.0, 4.0, 5.5, 7.0, 8.3, 10.2];

    // Calcula os coeficientes da regressão linear para essa série
    let modelo = regressao_linear(&dados);

    // Exibe a equação da reta obtida
    println!("TimeWise Analytics – Regressão Linear");
    println!("Equação: y = {:.2}x + {:.2}", modelo.a, modelo.b);

    // Avalia o ajuste do modelo com MSE (erro médio quadrático) e R²
    let mse = calcular_mse(&modelo, &dados);
    let r2 = calcular_r2(&modelo, &dados);
    println!("MSE: {:.4}", mse);
    println!("R²: {:.4}", r2);

    // Faz uma previsão para o próximo período (ex: mês seguinte)
    let futuro = dados.len() as f64;
    let estimativa = prever(&modelo, futuro);
    println!("Previsão para período {}: {:.2}", futuro as usize, estimativa);
}
