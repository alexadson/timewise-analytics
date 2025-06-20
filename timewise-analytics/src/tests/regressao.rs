// Importa o módulo e as funções do pacote principal
use timewise_analytics::regressao::{
    regressao_linear, calcular_mse, calcular_r2, prever, Coeficientes,
};

#[test]
fn testa_coeficientes_reta_perfeita() {
    let dados = vec![2.0, 4.0, 6.0, 8.0];
    let modelo = regressao_linear(&dados);
    assert!((modelo.a - 2.0).abs() < 1e-6); // Inclinação esperada: 2.0
    assert!((modelo.b - 2.0).abs() < 1e-6); // Intercepto esperado: 2.0
}

#[test]
fn testa_previsao() {
    let coef = Coeficientes { a: 1.5, b: 2.0 };
    let resultado = prever(&coef, 4.0); // Espera 1.5 * 4 + 2 = 8
    assert!((resultado - 8.0).abs() < 1e-6);
}

#[test]
fn testa_mse_zero_quando_ajuste_perfeito() {
    let dados = vec![3.0, 6.0, 9.0];
    let modelo = regressao_linear(&dados);
    let mse = calcular_mse(&modelo, &dados);
    assert!(mse.abs() < 1e-6);
}

#[test]
fn testa_r2_igual_um_quando_ajuste_perfeito() {
    let dados = vec![5.0, 10.0, 15.0, 20.0];
    let modelo = regressao_linear(&dados);
    let r2 = calcular_r2(&modelo, &dados);
    assert!((r2 - 1.0).abs() < 1e-6);
}

#[test]
fn testa_r2_parcial_com_ruido() {
    let dados = vec![2.0, 5.0, 7.0, 11.0];
    let modelo = regressao_linear(&dados);
    let r2 = calcular_r2(&modelo, &dados);
    assert!(r2 > 0.0 && r2 < 1.0);
}
