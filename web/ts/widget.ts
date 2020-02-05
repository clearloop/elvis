import { ElvisWidget } from "../pkg";

abstract class Widget {
  protected props?: object;

  constructor(props?: object) {
    this.props = props;
  }

  public abstract render(): ElvisWidget;
}

export default Widget;
