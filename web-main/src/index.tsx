import "./style.css";

import { start } from "test-241017-main/test_241017_main";
import WorkerFactory from "./worker?worker";

start();

const worker: Worker = WorkerFactory();
