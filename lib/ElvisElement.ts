interface ElvisElementArgs {
  tag: string,
}

// basic type of Elvis
class ElvisElement {
  public el: HTMLElement;
  
  constructor(args: ElvisElementArgs) {
    this.el = document.createElement(args.tag);
  }

  css(s: string) {
    this.el.style.cssText = s;
    return this;
  }

  text(s: string) {
    this.el.innerHTML = s;
    return this;
  }
}

// exports
export { ElvisElement };
