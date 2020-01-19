import { ElvisElement } from "./ElvisElement";

// Page
class Page {
  public child: ElvisElement;

  constructor(child: ElvisElement) {
    this.child = child;
  }

  public render() {
    document.body.appendChild(this.child.el);
  }
}

// exports
export { Page };
