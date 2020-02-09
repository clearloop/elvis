import { Widget } from "../pkg";

interface IRoutes {
  [name: string]: Widget;
}

// type Routes = Map<string, Widget>;
interface IPush {
  props?: object;
  title?: string;
}

class Router {
  public static back() {
    window.history.back();
  }

  public static push(path: string, pushProps = { props: {}, title: document.title }): void {
    if (window.location.pathname.slice(1) === path) {
      return;
    }

    window.history.pushState(pushProps.props, pushProps.title, path);
    (window as any).route();
  }

  public routes: IRoutes;

  constructor(routes?: IRoutes) {
    this.routes = routes;
  }
}


export default Router;
