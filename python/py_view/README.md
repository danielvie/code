```mermaid
graph TD
    subgraph MVC
        C[Controller]
        M[Model]
        V[View]
    end

    V -->|reads| M
    V -->|notifies| C
    C -->|updates| M
```