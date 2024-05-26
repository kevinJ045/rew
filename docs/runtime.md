# Runtime
`rew` can run single files or apps. after the main file has been imported, a context is created, that context is shared between files.

### Single files
```bash
rew ./file.coffee
```
You can also watch:
```bash
rew ./file.coffee --watch
```
### Apps
```bash
rew run .
```