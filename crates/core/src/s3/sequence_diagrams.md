
S3 iroh impl sequence diagrams

```mermaidi

sequenceDiagram
    participant Client
    participant HorizonNode
    participant IrohDocument
    
    Client->>HorizonNode: S3 Multipart Upload Request
    Note over HorizonNode: Extract from request:<br/>- bucket<br/>- key<br/>- credentials
    
    HorizonNode->>HorizonNode: Generate UUID (upload_id)
    HorizonNode->>HorizonNode: Create a hash using  access_key from credentials and key
    
    HorizonNode->>IrohDocument: Open Iroh document (bucket)
    HorizonNode->>IrohDocument: Store mapping:<br/>upload_id ↔ hash
    
    HorizonNode-->>Client: Return part_id to client
    
    Note over Client,IrohDocument: Subsequent upload parts<br/>will reference this part_id

```
