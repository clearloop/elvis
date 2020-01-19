import { ElvisElement } from "./ElvisElement";

// Page
class Page {
  child: ElvisElement;
  
  constructor(child: ElvisElement) {
    this.child = child;
  }
  
  render() {
    document.body.appendChild(this.child.el);
  }
}

// exports
export { Page };
