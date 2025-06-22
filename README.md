TimeWise Analytics
TimeWise Analytics é um projeto simples e educacional em Rust, desenvolvido para aplicar regressão linear em séries temporais de forma eficiente, sem o uso de bibliotecas externas.

Este projeto foi pensado para fins acadêmicos, trabalhos de faculdade ou primeiros contatos com algoritmos de análise de dados em Rust.

📌 Funcionalidades implementadas
1. regressao_linear(y: &[f64]) -> Coeficientes
Calcula os coeficientes da equação da reta y = a·x + b, assumindo que os valores de x são os índices da série (0, 1, 2,...). Retorna um struct Coeficientes contendo:

a: inclinação da reta

b: intercepto com o eixo Y

2. prever(&Coeficientes, x: f64) -> f64
Retorna o valor estimado de y para um novo ponto x, utilizando os coeficientes gerados pela regressão.

3. calcular_mse(&Coeficientes, y: &[f64]) -> f64
Calcula o Erro Quadrático Médio (MSE), que mede o quão próximo os valores estimados estão dos reais.

4. calcular_r2(&Coeficientes, y: &[f64]) -> f64
Retorna o Coeficiente de Determinação (R²), que representa a proporção da variação dos dados explicada pelo modelo (quanto mais próximo de 1, melhor).

🧪 Testes
O projeto possui testes automatizados localizados em tests/regressao_tests.rs, cobrindo:

Correção dos coeficientes

Previsões simples

Cálculo de MSE e R² para diferentes conjuntos de dados

Execute os testes com:

bash
cargo test
🚀 Como executar
Clone o repositório:

bash
git clone https://github.com/seu-usuario/timewise-analytics.git
cd timewise-analytics
Compile e rode:

bash
cargo run
📁 Estrutura básica
timewise-analytics/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── regressao.rs
└── tests/
    └── regressao_tests.rs
