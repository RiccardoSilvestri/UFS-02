<h1 align="center">
    <img src="./misc/banner.png">
</h1>

<p align="center">
  <i align="center">A TCP Note-Taking application 🖋</i>
</p>

<img src="https://github.com/RiccardoSilvestri/UFS-02/blob/main/misc/banner.png" alt="NetNote logo" title="NetNite" align="right" height="60" />

# NetNote

A TCP Note-Taking application written in Rust and Java.

# Server

The server is in charge of handling users and notes, and storing them to file.

# Client

The client makes the user create, delete and edit notes.

```mermaid
sequenceDiagram
    participant Server
    participant Client

    Client->>Server: LoginOption
    Server->>Client: "Request Received"
    Client->>Server: CredentialsJson
    Server->>Client: logged (bool)
    Client->>Server: NoteOption
    Server->>Client: NotesOfAuthor
    Client->>Server: Note
    Server->>Client: NotesOfAuthor
    Note over Client, Server: Back to NoteOption
```
