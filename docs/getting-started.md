# Getting started
To ge started with `rew`, all you have to do is install the `@makano/rew` npm package.

```bash
npm i -g @makano/rew
```
### After Install
To create a `rew` project:
```bash
rew create ./myApp
```
It will ask for more info, like if you want to use git or not and what your app package name is.

### Running
To run your new project, you can do:
```bash
rew run .
```
> Keep in mind that you have to `cd` to your project root where `node_modules` folder in order to use `require`.

### Custom Setup
Once you have installed it, you can use it freely with the `rew` command, but i recommend you install
it to your projects as a dependency too.
```bash
npm i @makano/rew
```
If you don't install it as a dependency, you can't use `require`.