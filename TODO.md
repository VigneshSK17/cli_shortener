# TODOs

A place for me to drop ideas for this project to be done later

### Todos

### Todo

- [ ] Switch from SQLx to SeaORM (allowing for SQLite and PostgreSQL interop)
- [ ] Add unit and integration tests
- [ ] Spin up EC2 instance with this project, setting up PostgreSQL
- [ ] Create frontend for the server
- [ ] Implement DNS
- [ ] Add "https://" if not in link

### Completed
- [x] Provide clean message when port is already in use
- [x] Enable ```.with_max_level(tracing::Level::DEBUG)``` when -v or --version flag enabled
- [x] Convert unwraps to proper error types in code and for user
- [x] Log all actions in a proper matter, both for users and debugging