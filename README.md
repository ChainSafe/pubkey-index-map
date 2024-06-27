# @chainsafe/pubkey-index-map

Small napi-rs shim with an interface similar to `Map<Uint8Array, number>` but only for length 48 Uint8Arrays for keys and unsigned 32 bit numbers for values.

```ts
import {PubkeyIndexMap} from "@chainsafe/pubkey-index-map"

// instantiate a new map
let map = new PubkeyIndexMap()

const pubkey: Uint8Array = ...;
const index: number = ...;

// Add a pubkey, index to the map
map.set(pubkey, index)

// Get an index from the map
map.get(pubkey) === index

// Delete a pubkey from the map
map.delete(pubkey)

// Clear the map
map.clear()

// Check the size of the map
map.size === 0
```

## License

MIT