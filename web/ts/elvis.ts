import { Elvis as ElvisPrototype } from "../pkg";
import Widget from "./widget";

interface IElvis {
  home: Widget;
}

class Elvis {
  private home: Widget;
  private proto: ElvisPrototype;

  constructor(props: IElvis) {
    this.home = props.home;
    this.proto = new ElvisPrototype(this.home.render());
  }

  public calling() {
    this.proto.calling();
  }
}

export default Elvis;
