
## reuniao 06 set
- ver um sw de desenho de circuito
- precisamos de uma alimentacao simetrica de +-5 V (fonte externa)
- kawa: nao pode terca a tarde e quinta a tarde
- kawa: sexta a tarde pode nao ser um bom dia


## reuniao 21 ago

1 - marcar tempo do quadprog
2 - tempo de leitura dos ad's
3 - tempo de comunicacao do matlab para o arduino
4 - dai escolhemos o circuito para que o tempo de oscilacao
5 - podemos usar os valores que o patino usou e aumentamos o capacitor e indutancia para proporcialmente
    aumentar o periodo de oscilacao
6 - capacitores eletrolitico, como nao tera tensao negativa
7 - poderia aumentar a indutancia tbm
8 - resistencia poderia ser ateh a do indutor
9 - talvez os valores reparados facam sentido


    leis de controle na embraer
    19h -> 21h
    auditorio Celso Renna
    parte de baixo do predio da eletronica e computacao

    Palestrante: Cleverson Maranhão Porto Marinho.
    Horário e local: Quarta-feira, dia 21/08, das 19h às 21h, no Auditório Celso Renna 
    (fica na parte de baixo do prédio da Eletrônica e Computação, do lado da secretaria da Comp).


## reuniao 19 jul

quando implementar temos que saber quantos ciclos de atrazo para poder fazer um 

compilar o qpOASIS na raspiberry


## 

mosfets teem que ser polarizadas, unipolar ou bipolar

functiona melhor com restencia baixa, alimentacao bipolar neg zero pos

como arduino teriamos essa limitacao e passar isso para o circuito

poderiamos usar uma fonte externa

qual a saida digital do arduino





## reuniao 19 jul






TODO:

[x] montar um circuito com led ligando e desligando por um condicional de tempo

[x] mandar mensagem do MATLAB para comecar a operacao

[x] mandar mensagem do MATLAB para mudar o tempo

[x] mandar um vetor com a sequencia de tempos e estado

[x] fazer um calculo de quadprog com o max de tempo para operacao (usar o patino2 como base, usar `tic toc`)

[ ] fazer um ciclo com 3 leds, calcular quadprog e enviar os comandos para o arduino






 - usar um tic toc

     quanto tempo leva pra resulver o quadprog

     quadprog tem que ser resolvido dentro de um ciclo

     
 ler o estado, tem o ciclo e aplicar o controle no prox ciclo

 o atraso de uma unidade entre o controle e o estado

 ver esse tempo

 depois a gente pode fazer um escalonamento do circuito
 para chegar num valor comercialmente viavel dos componentes

 `kawa`
 como montar o circuido com o arduino

 inclusao de um atraso no modelo tenho um artigo em que a gente fez isso
 :: handling the computation delay


 no instante k a gente le os valores de tensao

 monta o estado xi de k

 calcula o controle com esse xi de k

 guarda essas perturbacoes pra enviar no prox instante

 ver como mudar o controlador preditivo para colocar esse atraso

















## reuniao 11 jul
 - ficar mudando os bits para os modos e os bits
 - implementar em 3 saidas digitais do arduino
 - matlab
   - ler 3 saidas analogicas
   - calcular quadprog
   - escrever 3 bits em 3 saidas digitais

## reuniao 28 jun
 - fazer um codigo para ler um sinal analogico
 - emular um sistema de delta t
 - enviar

## reuniao 17 jun

 - usar o arduino calculando e o matlab fazerndo o calculo de dT's
 - matlab manda as referencias pro arduino
 - matlab manda referencias no proximo ciclo

## reuniao 7 jun

 video integracao arduino e matlab:
 https://www.youtube.com/watch?v=8NQ1h0gGgX8


 - para a prox reuniao:
 - [ ] fechar a malha e colocar valor de input
 - [ ] tentar arrumar uma arduino e fazer comunicacao do MATLAB

## reuniao 29 may
 - podemos alterar a referencia de corrente com o metodo phase shifted

 - fazer a referencia mudar com o tempo

 - fazer a implementacao intermediaria: 
   - matlab tem suporte para arduino
   - https://www.mathworks.com/hardware-support/arduino.html#matlab
   - entao usar o qpOasis no matlab ou na IDE do arduino
   - se sim, embarcamos o codigo no arduino
   
 - instalar o suporte de arduino no MATLAB

## 

 - tomar como alvo a sequencia da phase shift modulation que tem na tese de tolouse

 - mandei um email que calcula o intervalo de tempo a solucao padrao 

 - dai nao precisamos discutir o problema de otimizacao

 - estamos adotando a sequencia do patino


 - ! ver a tese

 - ele cria o que chama de portadoras triangulares

 - como se fosse modulacao de sinais em telecomunicacoes

 - as portadoras ficam chaveando de 0 para 1

   


 -- no artigo a gente escreve uma sentenca falando dos trabalhos que usam essa modulacao e apresentar a trajetoria alvo



 =======

 - poderiamos fazer:
   existe uma forma de gerar a seq de modos e sequencia de modos
   usando a tese dos franceses
   
   usar a solucao tradicional e fazer uma quanticzacao pra
   ver o quanto a gente perde e ver se fazendo uma otmin poderiamos
   melhorar
   
   patino pode nao sera  seq de modos adequada
   
   microcontrolador: f4 microcontroller
   
   
   equanto kawa ve a parte de trajetoria
   
   eu posso transpor o material da qualificacao, porque precisa de um artigo aceito
   
   JOTA -> Journal of Optimization Theory and Applications
   > https://link.springer.com/journal/10957
   
   veja o estilo latex do journal
   
   criar um draft to artigo no estilo latex
   
   colocar o metodo de linearizacao do estado aumentado que eh mais elegante

## 25 abril

 - criar simulacao com quantizacao de tempo de chaveamento

## 11 abril
 - otmizacao com inteiros
 - 

 MI nao linear
 https://www.scipopt.org/
 esp32
 raspberry
 knitro

## 4 abril
 - computar a quantizacao de um microcontrolador
 - teriamos que assumir um valor pra essa taxa de amostragem
 - numero de digitos para simular
 - a trajetoria de referencia precisaria ser gerada correta
 - patino aplica 22ms no segundo exemplo
 - arredondar a saida do controlador para um certo numero de digitos (multiplo de digitos)
 - estudo de robustez para a precisao do controlador
  




 despresando o erro pequeno multiplicados pelos deltas f17




 - gerar perturbacao no controle e fazer a predicao dos estados
 - entao encontrar qual dos dois metodos esta prevendo o estado melhor

 - eq. 4.29

 - ir diminuindo o delta
 - o resultado deveria convergir para a diferenca de x por delta d1

 - diminuindo os valores de tempo o valor deve convergir para o valor da predicao linearizada

## 29 fev

#### validacao numerica linearizado
   entrar em ciclo, 
   colocar uma perturbacao
   ver o estado no final do ciclo
   com a diferenca da pra calcular as sensibilidades
   
   pegar esses deltas e ir diminuindo e ver se o valor de sensibilidade vai convergindo
   esse valor limite deveria ser algum elemento da matriz gamma


   ideia de quantizar os instantes de chaveamento, colocar eles sempre multiplos de um 
   valor de tempo. (simulando a implementacao de um microcontrolador}

## 1 fev
   
 ##### tarefas
    
    - implementar comentarios prof kienitz
    - implementar comentarios prof rubens
    - estudar condicao da simulacao que esta divergindo
    
 ##### estou usando 
    
    - estou simulando para cada tstep e salvando o resultado
    - isso ta custando muito na simulacao
 
 ##### um possivel problema 
 
    - conferir tstep, multiplicade to Ts
    - experimentar o linspace para o lsim
    - garantir que a simulacao termina onde deve terminar
    
 ##### experimentacao
 
    - de fato se o dt eh muito pequeno ha um acumulos de erros
    
    quick fix
    
    - fazer a deducao da linearizacao usando o estado aumentado
    
    - terminar de reformatar a simulacao apenas com os pontos de interesse por regial, nao todos os pontos
    
    
    - preciso montar o vetor de sinal de control no cycle2
