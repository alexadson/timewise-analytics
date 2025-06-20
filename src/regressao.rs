/// Coeficientes da reta y = ax + b
pub struct Coeficientes {
    pub a: f64,
    pub b: f64,
}

/// Calcula a regressão linear simples baseada em índices como x
pub fn regressao_linear(y: &[f64]) -> Coeficientes {
    let n = y.len() as f64;
    let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();
    let media_x = x.iter().sum::<f64>() / n;
    let media_y = y.iter().sum::<f64>() / n;

    let numerador = x.iter().zip(y.iter()).map(|(xi, yi)| (xi - media_x) * (yi - media_y)).sum::<f64>();
    let denominador = x.iter().map(|xi| (xi - media_x).powi(2)).sum::<f64>();

    let a = numerador / denominador;
    let b = media_y - a * media_x;

    Coeficientes { a, b }
}

/// Retorna o valor previsto de y para um x futuro
pub fn prever(coef: &Coeficientes, x: f64) -> f64 {
    coef.a * x + coef.b
}
