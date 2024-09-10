```mermaid
graph TD
    A[View] -->|user actions| B[ViewModel]
    B -->|updates model| C[Model]
    C -->|returns values| B
    B -->|notifies UI| A

    subgraph Model-View-ViewModel
        C
        B
        A
    end

```
