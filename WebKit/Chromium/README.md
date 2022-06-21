# Chromium Browser

- Why Node.js and Chromium
  - Good explanation is [here](https://stackoverflow.com/questions/38166617/what-does-it-mean-for-electron-to-combine-node-js-and-chromium-contexts)

## Electron

- From [1]
  - Electron uses Chromium
  - Electron is a container that allows Node.js and Chromium to work together without any further modification or intervention
    - Chromium has been modified to be able to execute Node

### Electron and Chromium

- From [1]
  - Chromium uses Google's V8 engine
  - Electron uses generic Node.js engine
    - To enable `require`
    - And to avoid using browserify

## Tools

- [Browserify](https://browserify.org/)
  - Browser's don't have the `require` method defined, but Node.js does
    - Mainly to used to access local files in a browser
  - With browserify, you can write code that uses `require` in the same way that you would use it in Node.

## Links

- [1] JS Engine in Electron and Chromium
  - <https://stackoverflow.com/questions/38166617/what-does-it-mean-for-electron-to-combine-node-js-and-chromium-contexts>

