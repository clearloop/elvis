import { Center, Elvis, Text } from "../calling-elvis";

const MyCenter = Center(Text("Is anybody home?", {}));

// entry
Elvis.call(MyCenter);
