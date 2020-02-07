import { Elvis as ElvisPrototype, Widget } from "../pkg";
import Router from "./router";

interface IElvis {
  home: Widget;
  router?: Router;
}

class Elvis {
  public router: Router;
  private home: Widget;
  private proto: ElvisPrototype;

  constructor(props: IElvis) {
    this.router = props.router;
    this.home = props.home;
    this.proto = new ElvisPrototype(this.home);

    (window as any).route = () => {
      const ptr: string = window.location.pathname.slice(1);
      const widget = this.router.routes[ptr];
      // widget.setProps(window.history.state);
      this.proto = new ElvisPrototype(widget);
      this.calling();
    };
  }

  public calling() {
    this.proto.calling();
  }
}

export default Elvis;
