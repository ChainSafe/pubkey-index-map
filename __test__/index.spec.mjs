import test from 'ava'

import { PubkeyIndexMap } from '../index.js'

test('PubkeyIndexMap basics', (t) => {
  const map = new PubkeyIndexMap()

  t.is(map.size, 0)

  const pubkey = Buffer.from('pubkey')
  const index = 42
  t.is(map.has(pubkey), false)

  // Add a pubkey
  map.set(pubkey, index)
  map.set(pubkey, index+1)
  map.set(pubkey, index)

  t.is(map.size, 1)
  t.is(map.get(pubkey), index)
  t.is(map.get(Buffer.from('pubkey')), index)
  t.is(map.has(pubkey), true)
  t.is(map.has(Buffer.from('pubkey')), true)

  // Add another pubkey
  const pubkey2 = Buffer.from('pubkey2')
  const index2 = 43
  map.set(pubkey2, index2)

  t.is(map.size, 2)
  t.is(map.get(pubkey2), index2)

  // Remove a pubkey
  map.delete(pubkey)

  t.is(map.size, 1)
  t.is(map.get(pubkey), null)
  t.is(map.get(pubkey2), index2)

  // Clear the map
  map.clear()

  t.is(map.size, 0)
  t.is(map.get(pubkey), null)
  t.is(map.get(pubkey2), null)

  // Ensure different instances of the same pubkey are treated as the same
  const pubkey3 = Buffer.from('pubkey')
  const index3 = 44
  map.set(pubkey, index)
  map.set(pubkey3, index3)
  t.is(map.get(pubkey), index3)
  t.is(map.get(pubkey3), index3)
})
