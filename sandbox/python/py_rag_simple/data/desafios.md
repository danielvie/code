[[doutorado]]

- inicialmente usei arduino, mas ele sendo 1 nucleo limitou a parte de comunicacao e controle em paralelo

- o tempo de `us` foi bem desafiador, a funcao de `wait` nao era mais suficiente para gerenciar isso e precisei ir para solucoes de tempo real (FreeRTOS)

- `us` coloca limitacoes em como fazer o calculo tambem. tive que melhorar o algoritmo para fazer menos copias e memoria e fazer uma troca rápida de sets de valores