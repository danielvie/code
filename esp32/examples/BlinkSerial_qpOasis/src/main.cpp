#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include "qpOASES.hpp"


const int outputPin = 12;

int qpRun()
{
  USING_NAMESPACE_QPOASES

  const int nV = 2;
  const int nC = 3;

  real_t H[nV * nV] = {1, -1,
                       -1, 2};
  real_t f[nV] = {-2, -6};
  real_t A[nV * nC] = {1, 1,
                       -1, 2,
                       2, 1};
  real_t lb[nV] = {-1e20, -1e20};
  real_t ub[nV] = {1e20, 1e20};
  real_t lbA[nC] = {-1e20, -1e20, -1e20};
  real_t ubA[nC] = {2, 2, 3};

  QProblem example(nV, nC);
  Serial.println("estou aqui");

  Options options;
  options.printLevel = qpOASES::PL_NONE;
  example.setOptions(options);

  // initialize QP
  int_t nWSR = 10; // maximum number of working set recalculations
  example.init(H, f, A, lb, ub, lbA, ubA, nWSR);

  // solve the QP
  real_t xOpt[nV];
  example.getPrimalSolution(xOpt);

  // print the solution
  Serial.println("Solution:");
  for (int i = 0; i < nV; ++i)
  {
    Serial.print("x["); 
    Serial.print(i); 
    Serial.print("]");
    Serial.println(xOpt[i]);
  }

  return 0;
}

// put function declarations here:
int cont = 0;
int delay_ms = 200;


void TaskBlink(void *pvParameters) {
  while (true) {
    digitalWrite(outputPin, !digitalRead(outputPin));
    vTaskDelay(delay_ms / portTICK_PERIOD_MS); // Delay for 500 ms
  }

}

void TaskSerial(void *pvParameters) {
  (void) pvParameters; // To avoid unused parameter warning

  while (true) {
    if (Serial.available() > 0) {
      String incomingString = Serial.readStringUntil('\n');
      int ret = incomingString.toInt();
      bool onlynumbers = incomingString.indexOf(",") == -1;
      if (ret && onlynumbers) {
        delay_ms = ret;
      }

      Serial.println("recebi a mensagem serial");
      Serial.print("o valor de delay agora eh: ");
      Serial.println(delay_ms);

      qpRun();

    }
    vTaskDelay(100 / portTICK_PERIOD_MS); // Delay to avoid hogging the CPU
  }
}


void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
	pinMode(outputPin, OUTPUT);

  // Create the Blink task
  xTaskCreate(
    TaskBlink,        // Task function
    "Blink",          // Task name
    2048,             // Stack size (bytes)
    NULL,             // Parameter passed to the task
    1,                // Task priority
    NULL              // Task handle
  );

  // Create the Serial task
  xTaskCreate(
    TaskSerial,       // Task function
    "Serial",         // Task name
    2048*4,           // Stack size (bytes)
    NULL,             // Parameter passed to the task
    1,                // Task priority
    NULL              // Task handle
  );

}

void loop() {
  // Nothing to do here
}