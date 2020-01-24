# Components

A bunch of components is coming as they are...

`Elvis` just provides `Image` and `Text` for now, we've got apis to compose new `Component` in rust api, join us if you like this chapter of the score.

```rust
/// Elvis' Attribute is cool.
/// 
/// The no-cool part is, Elvis has a big Attributes Enum system, 
/// it's more responsible to use enum than chars.
pub struct Attribute {
  key: Attributes,
  value: AttributeValues,
}

/// Abstract ElvisElement
pub trait ElvisElement {
  // give birth to a new life
  fn append(&mut self);
  
  // deserialize html string to ElvisElement
  fn de(s: String) -> ElvisElement;
  
  // call her/his father
  fn father(&self) -> ElvisElement;
  
  // if you don't have a girlfriend, new one.
  fn new<T>() -> T;
  
  // serlalize ElvisElement to html string
  fn ser(&self) -> String;
}

/// Love is All you Need!
///
/// Speaking easily, you don't need to write the trait above,
/// Elvis has a proc-macro to impl the trait defaultly, 
/// just paint your child.
#[derive(ElvisElement)]
pub struct MyElement {
  css: String,
  tag: String,
  attrs: Vec<Attribute>,
  child: Arc<AsRef<ElvisElement>>,
}
```

## Javascript

It's quite intresting to dress componets up for a new face, we can do anything changing Elvis we want in Javascript, it depends on if you have the power to save the world, as we know, you are the one.
