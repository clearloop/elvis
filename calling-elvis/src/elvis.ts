import { Elvis as ElvisPrototype, Widget } from "elvis-web";
import Router from "./router";

interface IElvis {
  home: Widget;
  router?: Router;
}

class Elvis {
  public static call(widget: Widget) {
    new ElvisPrototype(widget).calling();
  }

  public router: Router;
  private home: Widget;
  private proto: ElvisPrototype;

  constructor(props: IElvis) {
    // init global route
    (window as any).route = () => {
      const ptr: string = window.location.pathname.slice(1);
      const widget = this.router.routes[ptr];
      this.proto = new ElvisPrototype(widget);
      this.calling();
    };

    // setters
    this.router = props.router;
    this.home = props.home;
    if (window.location.pathname === "/") {
      this.proto = new ElvisPrototype(this.home);
    } else {
      (window as any).route();
    }
  }

  public calling() {
    this.proto.calling();
  }
}

export default Elvis;
