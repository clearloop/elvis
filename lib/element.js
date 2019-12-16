/* element */
export default class Element {
  //@static 'b' - batch
  static b(o) {
    return o.map((options) => {
      return new Element().i(...options);
    });
  }
  
  //@static 'c' - create
  static c(tag, options = {}, methods = {}, text) {
    let el = document.createElement(tag);

    // set attributes
    Object.entries(options).reduce(
      (element, [field, value]) => {
        element.setAttribute(field, value);
        return element;
      }, el);

    // set methods
    Object.entries(methods).reduce(
      (element, [event, method]) => {
        element.addEventListener(event, method);
        return element;
      }, el);

    // set text
    text?el.innerHTML = text:'';
    return el;
  }
  
  constructor(tag, options = {}, methods = {}, text) {
    this.tag = tag;
    this.options = options;
    this.methods = methods;
    this.text = text;
    this.element = Element.c(tag, options, methods, text);
  }

  //@method 'i' - init
  i(tag, options = {}, methods = {}, text) {
    return new Element(tag, options, methods, text);
  }

  //@method 'a' - append
  a(...els) {
    els.forEach(e => this.element.appendChild(e.element));
  }

  //@method 't' - tree
  t(father) {
    father.element? father = father.element :'';
    father.appendChild(this.element);

    return this;
  }

  //@method 'p' - place
  p(text) {
    this.element.textContent = text;
    return this;
  }
}
