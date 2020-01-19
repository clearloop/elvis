interface IElvisElementArgs {
  tag: string;
}

// basic type of Elvis
class ElvisElement {
  public el: HTMLElement;

  constructor(args: IElvisElementArgs) {
    this.el = document.createElement(args.tag);
  }

  public css(s: string) {
    this.el.style.cssText = s;
    return this;
  }

  public text(s: string) {
    this.el.innerHTML = s;
    return this;
  }
}

// exports
export { ElvisElement };
