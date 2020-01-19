# Elvis

Calling Elvis...


## Install

```
$ yarn global add elvis
```

## Usage

A hello-world is just like this

```js
// pages/index.js
import 'elvis';

export default function() {
  return Page(
	Center({
		child: Text('hello, world'),
	}),
  );
}
```

run project

```
$ elvis dev
```

build project

```
$ elvis build
```


## Style Guide

[Google JavaScript Style Guide](https://google.github.io/styleguide/jsguide.html)
