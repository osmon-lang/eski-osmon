import search from "./search";
import pwa from "./pwa";
import popup from "./pwa-popup";

const plugins = [...search, ...pwa, ...popup]; // ...algolio if algolio finished indexing

export default plugins;
