```mermaid
graph TD
    subgraph MVP
        V[View]
        M[Model]
        P[PRESENTER]
    end

    M -->|notifies| P
    P -->|updates| M
    P -->|updates| V
    V -->|notifies| P
```