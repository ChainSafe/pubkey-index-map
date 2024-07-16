import {itBench} from "@dapplion/benchmark";

import {PubkeyIndexMap} from "../../";
import {PubkeyIndexMap as PubkeyIndexMapNaive} from "./naive";
import { randomBytes } from "crypto";

describe("get/set", () => {
  for (const N of [1_000, 1_000_000]) {
    const p0 = new PubkeyIndexMap();
    const p1 = new PubkeyIndexMapNaive();
    const pks = Array.from({length: N}, (_, i) => randomBytes(48));
    for (let i = 0; i < N; i++) {
      p0.set(pks[i], 0);
      p1.set(pks[i], 0);
    }

    itBench({
      id: `get values - ${N}`,
      beforeEach: () => {
        return Math.floor(Math.random() * N);
      },
      fn: (n) => {
        p0.get(pks[n]);
      }
    });

    itBench({
      id: `get values - naive - ${N}`,
      beforeEach: () => {
        return Math.floor(Math.random() * N);
      },
      fn: (n) => {
        p1.get(pks[n]);
      }
    });
    itBench({
      id: `set values - ${N}`,
      beforeEach: () => {
        return Math.floor(Math.random() * N);
      },
      fn: (n) => {
        p0.set(pks[n], 0);
      }
    });

    itBench({
      id: `set values - naive - ${N}`,
      beforeEach: () => {
        return Math.floor(Math.random() * N);
      },
      fn: (n) => {
        p1.set(pks[n], 0);
      }
    });
  }
});
