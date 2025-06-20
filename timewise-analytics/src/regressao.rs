/// Estrutura que representa os coeficientes da reta (modelo)
/// y = a * x + b
pub struct Coeficientes {
    pub a: f64, // Inclinação (coeficiente angular)
    pub b: f64, // Intercepto (valor de y quando x = 0)
}

/// Implementa a regressão linear simples a partir de uma série de valores y.
/// O eixo x é representado pelos índices dos elementos (0, 1, 2, ...)
pub fn regressao_linear(y: &[f64]) -> Coeficientes {
    let n = y.len() as f64;

    // Cria o vetor x com os índices da série
    let x: Vec<f64> = (0..y.len()).map(|i| i as f64).collect();

    // Calcula as médias de x e y
    let media_x = x.iter().sum::<f64>() / n;
    let media_y = y.iter().sum::<f64>() / n;

    // Numerador da fórmula da inclinação
    let numerador = x.iter().zip(y.iter())
        .map(|(xi, yi)| (xi - media_x) * (yi - media_y))
        .sum::<f64>();

    // Denominador da fórmula da inclinação
    let denominador = x.iter()
        .map(|xi| (xi - media_x).powi(2))
        .sum::<f64>();

    // Inclinação e intercepto
    let a = numerador / denominador;
    let b = media_y - a * media_x;

    Coeficientes { a, b }
}

/// Utiliza os coeficientes para prever o valor de y para um valor futuro de x.
pub fn prever(coef: &Coeficientes, periodo: f64) -> f64 {
    coef.a * periodo + coef.b
}

/// Calcula o Erro Quadrático Médio (Mean Squared Error)
/// Mede a média dos erros ao quadrado entre valores reais e previstos
pub fn calcular_mse(coef: &Coeficientes, y: &[f64]) -> f64 {
    let n = y.len() as f64;

    let soma_erros = y.iter().enumerate()
        .map(|(i, yi)| {
            let xi = i as f64;
            let y_pred = coef.a * xi + coef.b;
            (yi - y_pred).powi(2)
        })
        .sum::<f64>();

    soma_erros / n
}

/// Calcula o coeficiente de determinação R²
/// Mede a proporção da variância explicada pelo modelo
pub fn calcular_r2(coef: &Coeficientes, y: &[f64]) -> f64 {
    let media_y = y.iter().sum::<f64>() / y.len() as f64;

    let ss_tot = y.iter()
        .map(|yi| (yi - media_y).powi(2))
        .sum::<f64>();

    let ss_res = y.iter().enumerate()
        .map(|(i, yi)| {
            let xi = i as f64;
            let y_pred = coef.a * xi + coef.b;
            (yi - y_pred).powi(2)
        })
        .sum::<f64>();

    1.0 - (ss_res / ss_tot)
}
