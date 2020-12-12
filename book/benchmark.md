# Benchmark

A counter App is the target where elvis tests runs.

```js
/* PATH='counter/pages/index.js' */
import { Center, Text, StatefulWidget } from "calling-elvis";

class Counter extends StatefulWidget {
  state = {
    hello: "Is anybody home?"
  }

  create() {
    let start = new Date().getTime();
    for (let i = 0; i < 100000000; i++) {
      this.state.hello = i.toString();
    }
    let end = new Date().getTime();
    console.log((end - start) + "ms");
  }

  render() {
    return Center(
      Text(this.state.hello, {
        bold: true,
        italic: true,
        size: 6,
      })
    );
  }

  update() {
    console.log("update...");
  }

  dispose() {
    console.log("dispose...");
  }
}

export default Counter;
```

```sh
# PATH='counter/'
yarn dev
```

| Model                   | Processor                       | Memory                |
| ----------------------- | ------------------------------- | --------------------- |
| mbp.Catalina.17.13-inch | 2.3 GHz Dual-Core Intel Core i5 | 16 GB 2133 MHz LPDDR3 |


## DOM Update

| library | 10,000 | 100,000 | 1,000,000 | 10,000,000 | 100,000,000 |
| ------- | ------ | ------- | --------- | ---------- | ----------- |
| elvis   | ~1ms   | ~10ms   | ~100ms    | ~900ms     | ~8s         |
| react   | ~35ms  | ~125ms  | ~830ms    | ~8000ms    | broken      |
| vue     | ~7ms   | ~40ms   | ~280ms    | ~3000ms    | ~26s        |


## Bundle Size

| library | size  |
| ------- | ----- |
| elvis   | 432kb |
| react   | 484kb |
| vue     | 592kb |
