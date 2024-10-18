import "./style.css";

import { start } from "test-241017-all-in-one/test_241017_all_in_one";
import WorkerFactory from "./worker?worker";

start();

const worker: Worker = WorkerFactory();
