
### Project structure:
```
src/
├── adapters
│   ├── crossterm
│   │   └── input.rs
│   ├── crossterm.rs
│   ├── sysinfo
│   │   └── sysinfo_datasource.rs
│   └── sysinfo.rs
├── adapters.rs
├── app.rs
├── components
│   ├── process_table
│   │   ├── component.rs
│   │   └── state.rs
│   └── process_table.rs
├── components.rs
├── core
│   ├── common
│   │   └── bounded_queue.rs
│   ├── common.rs
│   ├── process
│   │   ├── model.rs
│   │   └── primitive.rs
│   ├── process.rs
│   └── README.md
├── core.rs
├── events
│   ├── app_event.rs
│   └── README.md
├── events.rs
├── lib.rs
├── main.rs
├── README.md
├── services
│   └── sysinfo_worker.rs
└── services.rs
```

