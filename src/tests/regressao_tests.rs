use timewise_analytics::regressao::{regressao_linear, prever, Coeficientes};

#[test]
fn testa_coeficientes() {
    let y = vec![2.0, 4.0, 6.0];
    let modelo = regressao_linear(&y);
    assert!((modelo.a - 2.0).abs() < 1e-6);
    assert!((modelo.b - 2.0).abs() < 1e-6);
}

#[test]
fn testa_previsao() {
    let coef = Coeficientes { a: 1.5, b: 1.0 };
    let resultado = prever(&coef, 4.0);
    assert!((resultado - 7.0).abs() < 1e-6);
}
