# DOM
> **Note**: Elvis just render static html pages now, DOM is in progress.

Like the other UI libraries, webassemblly Elvis arms a Virtual-DOM, too.

```js
import { Center, Elvis, GestureDetector, State, Text } from "calling-elvis";

class MyState extends State {
  state = {
    count: 0,
  }
  
  create() {
    console.log("The first step is creating the dom.");
  }
  
  update() {
    console.log("Then we can update it, just like what you saw.");
  }
  
  render() {
    return Center(
      GestureDetector(
        Text("I'm a Elvis fans."),
        onTap: () => this.setState({ 
            count: this.count + 1 
        });
      ),
    );
  }
  
  drop() {
    console.log("Don't be crue, if you want to drop me.");
  }
}
```

I won't tell you that I'm not only a `React` fan but also a `Flutter` fan, and you'll never know I'm a `Emacs` fan.

## LifeCycle

Already knew you care about lifecycles, Elvis will never force you to recite 11 or more methods, Elvis just got 3, and it is enough ; )

Furthermore, to be serious, we don't recommend you to write big projects such as `facebook`, `reddit`, or `twitter` calling Elvis, `Elvis` is under **Proof-of-Concept** for now, and...you know, we **strongly recommend** you to use Elvis building your persenol website, make the web better, more interesting, awesome as it has never been 🌈

And if you are building a rocky start-up project, believe me, **CALLING ELVIS**, and the force will be with you!

Life Story, Love and Glory.

### Diff

Elvis' diff algorithm is quite simple **flying with wasm**, we compare the new node and the old one using dps, and then patch the updates to the old one.

A TreeNode in Elvis is like this: 

```rust
#[derive(Clone, Eq, PartEq)]
pub struct TreeNode {
  css: CSS,
  tag: String,
  attrs: Vec<Attribute>,
  children: Vec<Rc<AsRef<TreeNode>>>,
}
```

### Patch

Elvis prefers to cure the naughty node's father node, if there are complex changes inside it, for Example: 

```html
<father :="I'm the naughty nodes' father">
  <naughty>Up</naughty>
  <naughty>Side</naughty>
  <naughty>Down</naughty>
</father>
```

Now we upside down the `Up`, `Side`, `Down` List:

```html
<father :="plz don't make me heartbreak for twice.">
  <naughty>Down</naughty>
  <naughty>Side</naughty>
  <naughty>Up</naughty>
</father>
```

Elvis with not trying to swap `<li>Up</li>` and `<li>Down</li>`, it will operate DOM twice over, we just replace the whole `<ol>...</ol>` for once.
