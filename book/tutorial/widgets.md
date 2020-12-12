# Widgets

## Single Widget

Elvis has `StatefulWidget` and `Widget`, usually, `StatefulWidget` composed by `class` and `Widget` composed by `variable` or `function`.

### StatefulWidget

```js
class MyWidget extends StatefulWidget {
  state = {
    intro: "Is anybody home?",
  }
  
  render() {
    return Center(
      Text(this.state.intro),
    );
  }
  
  create() {
    console.log("create...");
    this.state.intro = "Roll up for the magical mystery tour!";
  }
  
  update() {
    console.log("update...");
  }
  
  dispose() {
    console.log("dispose...");
  }
}
```

### Widget

```js
// for variable
const myWidget = Center(Text("This is a line of chars"));
```

```js
// for function
const myWidget = (line: string) => Center(Text(line));
```

## Complex Widgets

We got some widgets made by our selvies sometimes.

```js
class Child extends StatefulWidget {
  state = {
    name: "",
  }

  constructor(name: string) {
     this.state.name = name;
  }
  
  render() {
    return Text("My name is" + this.state.name);
  }
}
```

> **Wrong example**: Do not `new` Widget inside widgets, **unless you want to pass props to it**.
>
> ```js
> class Parent extends StatefulWidget {
>   render() {
>     return Center(new Child("Elvis"));
>   }
> }
> ```
> If you did this, the `Child` widget will re-render everytime the parent widget changes, 
> cause every `StatefulWidget` got a reflect `pointer` in wasm, each of them gots its own 
> **widget tree and stylesheet.**

```js
class Parent extends StatefulWidget {
   widgets = {
     child: new Child("Elvis"),
   }
   
   render() {
     return Center(this.widgets.child);
   }
}
```

```js
class Parent extends StatefulWidget {
   state = {
     name: "Elvis",
   }
   
   render() {
     return Center(new Child(this.state.name));
   }
}
```
