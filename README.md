### Setup syloUI
```
cd syloUI
yarn install
cd ios && pod install && cd ..
## START SUBSTRATE NODE
NODE_OPTIONS=--max_old_space_size=8192 react-native start
react-native run-ios
```
---
我们Sylo前端实现出现了点问题，未能按时完成：
1. 无法连接在onfinality 部署的节点，报了一下错误 `console.error: "2019-08-04 16:57:51", " API-WS:", "disconnected from wss://mx-hashpire-test.ap1.onfinality.io code: 'undefined' reason: 'undefined'" ` 可以连接本地节点以及wss://cennznet-node-0.centrality.cloud:9944
2. commit `df0402391a9f238a8105dd63b38d50a66611fb1a` 把 commit `2424976f7af7f5eeb7740e12c3dfa3e0d270766b` 和 commit `ab7222cf7a9a06a2d58e466f3bc9c91da0c5bf9a` 进行了merge之后，Sylo前端跑不动，报了错误 `console.error: "2019-08-04 20:57:47", "  API/DECORATOR:", {}`，由于时间原因，现在还没找到具体原因和解决方式。

---
为了使得Sylo/React Native 环境能跑@Polkadot/api , 我们做了一下操作
```
yarn add node-libs-react-native
yarn add vm-browserify
yarn add bs58
yarn add @polkadot/api
yarn add @polkadot/keyring
```
修改metro.config.js
```
const nodeLibs = require("node-libs-react-native");
nodeLibs.bs58 = require.resolve("bs58");
nodeLibs.vm = require.resolve("vm-browserify");

module.exports = {
  transformer: {
    getTransformOptions: async () => ({
      transform: {
        experimentalImportSupport: false,
        inlineRequires: false
      }
    })
  },
  resolver: {
    extraNodeModules: nodeLibs
  }
};
```
在index.js增加
```
import 'node-libs-react-native/globals';
```
pod install
```
cd ios && pod install && cd ..
```


### Setup Substrate node
```
cd substrate
./restart.sh
```